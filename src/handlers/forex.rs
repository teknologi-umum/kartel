use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, NaiveDate, Utc};

use anyhow::{Context, anyhow};
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{prelude::*, types::ParseMode};

use crate::error::HandlerError;
use crate::{commands::Args, deps::http_client::http_client, error::AsInternalError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// format pair of currencies: USD/IDR, BTC/USD, XAU/USD, etc. Case insensitive.
static FOREX_PAIR_FORMAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^[A-Z]{3}/[A-Z]{3}$").expect("failed initializing forex regex")
});

// format of a currency code: USD, IDR, BTC, XAU. Case insensitive.
static FOREX_FORMAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^[a-z]{3}$").expect("failed initializing forex regex"));

static FOREX_ENDPOINT: &'static str = "https://api.mfirhas.com/pfm/forex/convert";

static EMPTY_ARGS_ERROR: &'static str = "Arguments must be provided.\nArguments are: \n1. Pair of forex: e.g. \"USD/IDR\", \n2. (Optional) Date of rate, e.g.\"USD/IDR 2022-02-02\" ";

#[derive(Debug, Serialize, Deserialize)]
pub enum ForexResponse {
    EmptyArgResponse(Vec<ForexResp<ConvertResponseData>>),
    SinglePairArgResponse(ForexResp<ConvertResponseData>),
    BaseRatesResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexResp<T> {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResponseData {
    pub date: DateTime<Utc>,
    pub from: HashMap<String, String>,
    pub to: HashMap<String, String>,
    pub code: String,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SinglePairArg {
    // left and right denote pair of forex, e.g. BTC/USD, BTC is left, and USD is right
    pub(super) left: String,
    pub(super) right: String,

    // date of historical rates
    pub(super) date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub(crate) struct BaseRatesArg {
    pub(super) base: String,

    pub(super) date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub(crate) enum ForexArgs {
    Empty,
    SinglePair(SinglePairArg),
    BaseRates(BaseRatesArg),
}

impl TryFrom<Args> for SinglePairArg {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.0.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err(HandlerError::InvalidArguments(anyhow!(EMPTY_ARGS_ERROR)));
        }

        let pair = parts[0];
        let pair_parts: Vec<&str> = pair.split('/').collect();

        if pair_parts.len() != 2 || !FOREX_PAIR_FORMAT.is_match(pair) {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "Forex pair must be in format XXX/YYY"
            )));
        }

        let left = pair_parts[0].to_ascii_uppercase();
        let right = pair_parts[1].to_ascii_uppercase();

        let date = if parts.len() >= 2 {
            let date_str = parts[1];

            let naive = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
                HandlerError::InvalidArguments(anyhow!("invalid date {}, {}", date_str, e))
            })?;

            Some(DateTime::<Utc>::from_naive_utc_and_offset(
                naive
                    .and_hms_opt(0, 0, 0)
                    .ok_or(HandlerError::InvalidArguments(anyhow!("invalid date")))?,
                Utc,
            ))
        } else {
            None
        };

        Ok(SinglePairArg { left, right, date })
    }
}

impl TryFrom<Args> for BaseRatesArg {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.0.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err(HandlerError::InvalidArguments(anyhow!(EMPTY_ARGS_ERROR)));
        }

        let curr = parts[0];

        if !FOREX_FORMAT.is_match(curr) {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "Forex pair must be in format XXX, case insensitive"
            )));
        }

        let date = if parts.len() >= 2 {
            let date_str = parts[1];

            let naive = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
                HandlerError::InvalidArguments(anyhow!("invalid date {}, {}", date_str, e))
            })?;

            Some(DateTime::<Utc>::from_naive_utc_and_offset(
                naive
                    .and_hms_opt(0, 0, 0)
                    .ok_or(HandlerError::InvalidArguments(anyhow!("invalid date")))?,
                Utc,
            ))
        } else {
            None
        };

        Ok(BaseRatesArg {
            base: curr.to_ascii_uppercase(),
            date: date,
        })
    }
}

impl TryFrom<Args> for ForexArgs {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let args = value.0.as_str().trim();

        if args.is_empty() {
            return Ok(ForexArgs::Empty);
        }

        if let Ok(ret) = SinglePairArg::try_from(value.clone()) {
            return Ok(ForexArgs::SinglePair(ret));
        }

        if let Ok(ret) = BaseRatesArg::try_from(value) {
            return Ok(ForexArgs::BaseRates(ret));
        }

        Err(HandlerError::InvalidArguments(anyhow!("Invalid arguments")))
    }
}

impl Display for ForexResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match self {
            Self::EmptyArgResponse(resp) => {
                if resp.is_empty() {
                    "Empty forex data".to_owned()
                } else {
                    let date = if let Some(ref data) = resp.get(0)
                        && let Some(ref inner_data) = data.data
                    {
                        inner_data.date.format("%Y-%m-%d %H:%M:%S %:z").to_string()
                    } else {
                        Utc::now().format("%Y-%m-%d %H:%M:%S %:z").to_string()
                    };

                    let mut content: String = "".to_string();

                    for r in resp {
                        if let Some(ref data) = r.data {
                            let from = data.from.keys().next().cloned().unwrap_or("INVALID".into());
                            let to = data.to.keys().next().cloned().unwrap_or("INVALID".into());
                            let pair = format!("{}/{}", from, to);

                            content
                                .push_str(format!("\n- <b>{}= {}</b>", pair, data.code).as_str());
                        } else if let Some(ref err) = r.error {
                            content.push_str(format!("\nerror: {}", err).as_str());
                        } else {
                            content.push_str("\nno data");
                        }
                    }

                    format!("Forex data on {}:{}", date, content)
                }
            }

            Self::SinglePairArgResponse(resp) => {
                if let Some(ref err) = resp.error {
                    format!("forex api error: {}", err)
                } else {
                    let single_pair_ret = match resp.data {
                        Some(ref data) if data.from.is_empty() || data.to.is_empty() => {
                            format!("invalid response empty data")
                        }

                        Some(ref data) => {
                            let from = data.from.keys().next().cloned().unwrap_or("INVALID".into());
                            let to = data.to.keys().next().cloned().unwrap_or("INVALID".into());
                            let pair = format!("{}/{}", from, to);

                            format!(
                                "{} on {} is:\n<b>{}</b>",
                                pair,
                                data.date.format("%Y-%m-%d %H:%M:%S %:z").to_string(),
                                data.code,
                            )
                        }

                        None => String::from("no data returned"),
                    };
                    single_pair_ret
                }
            }

            Self::BaseRatesResponse => String::from("todo: return forex list of rates"),
        };

        write!(f, "{}", ret)
    }
}

pub(crate) async fn forex_handler(bot: Bot, msg: &Message, args: Args) -> Result<(), HandlerError> {
    let arg: ForexArgs = args.try_into()?;

    match arg {
        ForexArgs::Empty => empty_arg(bot.clone(), msg).await,
        ForexArgs::SinglePair(single_pair_arg) => {
            single_pair(bot.clone(), msg, single_pair_arg).await
        }
        ForexArgs::BaseRates(base_rates_arg) => base_rates(bot, msg, base_rates_arg).await,
    }
}

async fn empty_arg(bot: Bot, msg: &Message) -> Result<(), HandlerError> {
    let http_client = http_client().clone();

    let usd_idr_params: Vec<(&str, &str)> = vec![("from", "USD 1"), ("to", "IDR")];

    let btc_usd_params: Vec<(&str, &str)> = vec![("from", "BTC 1"), ("to", "USD")];

    let xau_usd_params: Vec<(&str, &str)> = vec![("from", "XAU 1"), ("to", "USD")];

    let xag_idr_params: Vec<(&str, &str)> = vec![("from", "XAG 1"), ("to", "IDR")];

    let query_params: Vec<Vec<(&str, &str)>> = vec![
        usd_idr_params,
        btc_usd_params,
        xau_usd_params,
        xag_idr_params,
    ];

    let mut resp: Vec<ForexResp<ConvertResponseData>> = vec![];
    for query in &query_params {
        let ret: ForexResp<ConvertResponseData> = http_client
            .get(FOREX_ENDPOINT)
            .query(query)
            .send()
            .await
            .context("failed calling forex convert api")
            .as_internal_err()?
            .json()
            .await?;

        resp.push(ret);
    }

    bot.send_message(
        msg.chat.id,
        ForexResponse::EmptyArgResponse(resp).to_string(),
    )
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}

async fn single_pair(
    bot: Bot,
    msg: &Message,
    single_pair_args: SinglePairArg,
) -> Result<(), HandlerError> {
    let http_client = http_client().clone();

    let query_params: Vec<(&str, String)> = if let Some(date) = single_pair_args.date {
        vec![
            ("from", format!("{} 1", &single_pair_args.left)),
            ("to", single_pair_args.right),
            ("date", date.format("%Y-%m-%d").to_string()),
        ]
    } else {
        vec![
            ("from", format!("{} 1", &single_pair_args.left)),
            ("to", single_pair_args.right),
        ]
    };

    let ret: ForexResp<ConvertResponseData> = http_client
        .get(FOREX_ENDPOINT)
        .query(&query_params)
        .send()
        .await
        .context("failed calling forex convert api")
        .as_internal_err()?
        .json()
        .await?;

    bot.send_message(
        msg.chat.id,
        ForexResponse::SinglePairArgResponse(ret).to_string(),
    )
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}

async fn base_rates(bot: Bot, msg: &Message, base_args: BaseRatesArg) -> Result<(), HandlerError> {
    bot.send_message(msg.chat.id, "menyusul".to_string())
        .reply_to(msg.id)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(()) // TODO
}
