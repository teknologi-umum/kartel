use std::fmt::Display;

use anyhow::{Context, anyhow};
use chrono::{DateTime, NaiveDate, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{prelude::*, types::ParseMode};

use crate::commands::Args;
use crate::deps::http_client::http_client;
use crate::error::{AsInternalError, HandlerError};
use crate::handlers::forex::{ConvertResponseData, ForexResp};

// format of a currency code: USD, IDR, BTC, XAU. Case insensitive.
static CURRENCY_FORMAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^[a-z]{3}$").expect("failed initializing currency regex"));

// format for amount: optional commas for thousands, optional decimal point
static AMOUNT_FORMAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\d,]+(?:\.\d+)?$").expect("failed initializing amount regex"));

static CONVERT_ENDPOINT: &'static str = "https://api.mfirhas.com/pfm/v2/forex/convert";

static EMPTY_ARGS_DEFAULT: &'static str = "USD 1";
static EMPTY_ARGS_TO: &'static str = "IDR";

// Fallback values for display
static INVALID_CURRENCY: &'static str = "INVALID";
static ZERO_AMOUNT: &'static str = "0";

#[derive(Debug, Clone)]
pub(crate) struct ConvertArg {
    pub(super) from_currency: String,
    pub(super) from_amount: String,
    pub(super) to_currency: String,
    pub(super) date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub(crate) enum ConvertArgs {
    Empty,
    Convert(ConvertArg),
}

impl TryFrom<Args> for ConvertArg {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let trimmed = value.0.trim();

        // Split by semicolon
        let parts: Vec<&str> = trimmed.split(';').collect();

        if parts.len() < 2 || parts.len() > 3 {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "Arguments must be in format: <FROM_CODE> <AMOUNT> ; <TO_CODE> [; <DATE>]\nExample: USD 50,000 ; IDR\nWith date: USD 50,000 ; IDR ; 2022-02-02"
            )));
        }

        let from_part = parts[0].trim();
        let to_part = parts[1].trim();

        // Parse FROM part (currency code and amount)
        let from_tokens: Vec<&str> = from_part.split_whitespace().collect();

        if from_tokens.len() != 2 {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "FROM part must have format: <CURRENCY_CODE> <AMOUNT>\nExample: USD 1000"
            )));
        }

        let from_currency = from_tokens[0];
        let from_amount = from_tokens[1];

        // Validate currency code format
        if !CURRENCY_FORMAT.is_match(from_currency) {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "Currency code must be 3 letters (case insensitive). Got: {}",
                from_currency
            )));
        }

        // Validate amount format
        if !AMOUNT_FORMAT.is_match(from_amount) {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "Amount must be a number with optional commas and decimal point. Got: {}",
                from_amount
            )));
        }

        // Parse TO part (just currency code)
        if !CURRENCY_FORMAT.is_match(to_part) {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "TO currency code must be 3 letters (case insensitive). Got: {}",
                to_part
            )));
        }

        // Parse optional date (third part after second semicolon)
        let date = if parts.len() == 3 {
            let date_str = parts[2].trim();

            let naive = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
                HandlerError::InvalidArguments(anyhow!(
                    "Invalid date format \"{}\": {}. Expected YYYY-MM-DD format.",
                    date_str,
                    e
                ))
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

        Ok(ConvertArg {
            from_currency: from_currency.to_ascii_uppercase(),
            from_amount: from_amount.to_string(),
            to_currency: to_part.to_ascii_uppercase(),
            date,
        })
    }
}

impl TryFrom<Args> for ConvertArgs {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let args = value.0.as_str().trim();

        if args.is_empty() {
            return Ok(ConvertArgs::Empty);
        }

        let convert_arg = ConvertArg::try_from(value)?;
        Ok(ConvertArgs::Convert(convert_arg))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConvertResponse {
    Single(ForexResp<ConvertResponseData>),
}

impl Display for ConvertResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match self {
            Self::Single(resp) => {
                if let Some(ref err) = resp.error {
                    format!("forex api error: {}", err)
                } else {
                    match resp.data {
                        Some(ref data) if data.from.is_empty() || data.to.is_empty() => {
                            format!("invalid response: empty data")
                        }

                        Some(ref data) => {
                            let from_currency = data
                                .from
                                .keys()
                                .next()
                                .cloned()
                                .unwrap_or(INVALID_CURRENCY.into());
                            let from_amount = data
                                .from
                                .values()
                                .next()
                                .cloned()
                                .unwrap_or(ZERO_AMOUNT.into());

                            format!(
                                "Conversion on {}:\n<b>{} {} = {}</b>",
                                data.date.format("%Y-%m-%d %H:%M:%S %:z").to_string(),
                                from_currency,
                                from_amount,
                                data.code,
                            )
                        }

                        None => String::from("no data returned"),
                    }
                }
            }
        };

        write!(f, "{}", ret)
    }
}

pub(crate) async fn convert_handler(
    bot: Bot,
    msg: &Message,
    args: Args,
) -> Result<(), HandlerError> {
    let arg: ConvertArgs = args.try_into()?;

    match arg {
        ConvertArgs::Empty => empty_arg(bot, msg).await,
        ConvertArgs::Convert(convert_arg) => convert(bot, msg, convert_arg).await,
    }
}

async fn empty_arg(bot: Bot, msg: &Message) -> Result<(), HandlerError> {
    let http_client = http_client().clone();

    let query_params: Vec<(&str, &str)> = vec![("from", EMPTY_ARGS_DEFAULT), ("to", EMPTY_ARGS_TO)];

    let resp: ForexResp<ConvertResponseData> = http_client
        .get(CONVERT_ENDPOINT)
        .query(&query_params)
        .send()
        .await
        .context("failed calling forex convert api")
        .as_internal_err()?
        .json()
        .await?;

    bot.send_message(msg.chat.id, ConvertResponse::Single(resp).to_string())
        .reply_to(msg.id)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn convert(bot: Bot, msg: &Message, convert_arg: ConvertArg) -> Result<(), HandlerError> {
    let http_client = http_client().clone();

    let from_param = format!("{} {}", convert_arg.from_currency, convert_arg.from_amount);

    let mut query_params: Vec<(&str, String)> =
        vec![("from", from_param), ("to", convert_arg.to_currency)];

    if let Some(date) = convert_arg.date {
        query_params.push(("date", date.format("%Y-%m-%d").to_string()));
    }

    let resp: ForexResp<ConvertResponseData> = http_client
        .get(CONVERT_ENDPOINT)
        .query(&query_params)
        .send()
        .await
        .context("failed calling forex convert api")
        .as_internal_err()?
        .json()
        .await?;

    bot.send_message(msg.chat.id, ConvertResponse::Single(resp).to_string())
        .reply_to(msg.id)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
