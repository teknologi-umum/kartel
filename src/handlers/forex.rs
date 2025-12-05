use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, NaiveDate, Utc};

use anyhow::{Context, anyhow};
use teloxide::{prelude::*, types::ParseMode};

use crate::error::HandlerError;
use crate::{commands::Args, deps::http_client::http_client, error::AsInternalError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static FOREX_FORMAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]{3}(?:/[A-Z]{3})?$").expect("failed initializing forex regex")
});

static FOREX_ENDPOINT: &'static str = "https://api.mfirhas.com/pfm/forex/convert";

static EMPTY_ARGS_ERROR: &'static str = "Arguments must be provided.\nArguments are: \n1. Pair of forex: e.g. \"USD/IDR\", \n2. (Optional) Date of rate, e.g.\"USD/IDR 2022-02-02\" ";

#[derive(Debug, Serialize, Deserialize)]
pub struct ForexDTO {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<ConvertResponse>,

    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResponse {
    pub date: DateTime<Utc>,

    pub from: HashMap<String, String>,

    pub result: HashMap<String, String>,

    pub result_code: String,
    pub result_symbol: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ForexArgs {
    // left and right denote pair of forex, e.g. BTC/USD, BTC is left, and USD is right
    left: String,
    right: String,

    // date of historical rates
    date: Option<DateTime<Utc>>,
}

impl TryFrom<Args> for ForexArgs {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.0.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err(HandlerError::InvalidArguments(anyhow!(EMPTY_ARGS_ERROR)));
        }

        let pair = parts[0];
        let pair_parts: Vec<&str> = pair.split('/').collect();

        if pair_parts.len() != 2 || !FOREX_FORMAT.is_match(pair) {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "Forex pair must be in format XXX/YYY"
            )));
        }

        let left = pair_parts[0].to_string();
        let right = pair_parts[1].to_string();

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

        Ok(ForexArgs { left, right, date })
    }
}

#[cfg(test)]
mod test_forex {
    use chrono::{TimeZone, Utc};

    use crate::{commands::Args, handlers::forex::ForexArgs};

    #[test]
    fn argument_parsing_test() {
        let args = Args("USD/IDR".into());

        let ret: ForexArgs = args.try_into().unwrap();
        dbg!(&ret);
        assert_eq!("USD".to_string(), ret.left);
        assert_eq!("IDR".to_string(), ret.right);
        assert_eq!(None, ret.date);

        let args = Args("BTC/IDR 2022-02-02".into());

        let ret: ForexArgs = args.try_into().unwrap();
        dbg!(&ret);
        assert_eq!("BTC".to_string(), ret.left);
        assert_eq!("IDR".to_string(), ret.right);
        assert_eq!(
            Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(),
            ret.date.unwrap()
        );
    }
}

impl Display for ForexDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = || {
            if let Some(ref err) = self.error {
                return format!("forex api error: {}", err);
            }

            match self.data {
                Some(ref data) if data.from.is_empty() || data.result.is_empty() => {
                    return format!("invalid response empty data");
                }

                Some(ref data) => {
                    let from = data.from.keys().next().cloned().unwrap_or("INVALID".into());
                    let to = data
                        .result
                        .keys()
                        .next()
                        .cloned()
                        .unwrap_or("INVALID".into());
                    let pair = format!("{}/{}", from, to);

                    return format!(
                        "{} on {} is:\n<b>{}</b>",
                        pair,
                        data.date.format("%Y-%m-%d %H:%M:%S %:z").to_string(),
                        data.result_code,
                    );
                }

                None => {
                    return String::from("no data returned");
                }
            }
        };

        write!(f, "{}", ret())
    }
}

pub(crate) async fn forex_handler(bot: Bot, msg: &Message, args: Args) -> Result<(), HandlerError> {
    let forex_args: ForexArgs = args.try_into()?;

    let http_client = http_client().clone();

    let query_params: Vec<(&str, String)> = if let Some(date) = forex_args.date {
        vec![
            ("from", format!("{} 1", &forex_args.left)),
            ("to", forex_args.right),
            ("date", date.format("%Y-%m-%d").to_string()),
        ]
    } else {
        vec![
            ("from", format!("{} 1", &forex_args.left)),
            ("to", forex_args.right),
        ]
    };

    let ret: ForexDTO = http_client
        .get(FOREX_ENDPOINT)
        .query(&query_params)
        .send()
        .await
        .context("failed calling forex convert api")
        .as_internal_err()?
        .json()
        .await?;

    bot.send_message(msg.chat.id, ret.to_string())
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
