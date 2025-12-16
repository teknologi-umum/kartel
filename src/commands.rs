use std::fmt::Display;
use std::str::FromStr;

use teloxide::utils::command::BotCommands;

#[derive(Clone, Debug)]
pub(crate) struct Args(pub(crate) String);

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Args {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Args(s.into()))
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "This is Teknologi Umum Bot written in Rust supporting these commands:"
)]
pub(crate) enum Command {
    #[command(description = "Show this help message")]
    Help,

    #[command(description = r#"Fetch prices of some moneys.
Arguments:
- No arguments: Give list of author chosen rates.
- Forex pair:
  - USD/IDR (pair of currency, case insensitive)
  - USD/IDR 2022-02-02 (YYYY-MM-DD, date is optional)
- Base rates:
  - USD (currency as a base, case insensitive)
  - USD 2022-02-02 (YYYY-MM-DD, date is optional)
    "#)]
    Forex(Args),

    #[command(
        description = r#"Fetch Precious Metals(Gold & Silver) prices in multiple fiat currencies.
Optional params:
- Date of rates: YYYY-MM-DD
    "#
    )]
    PM(Args),

    #[command(
        description = r#"Fetch Zakat information, such current nishab in Gold and Silver.
Optional params:
- Amount: your current holding in fiat, and bot will calculate if you reach nishab. E.g. IDR 1,000,000,000.02
- Date of start: Date of you reach nishab, bot will calculate the end of the year in hijri calendar mapped into Gregory calendar.
        "#
    )]
    Zakat(Args),

    #[command(
        description = r#"Fetch information of a stock. Default to author picks.
Optional params:
- Stock ticker: e.g. BBCA
        "#
    )]
    Stock(Args),
}
