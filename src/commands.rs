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

    #[command(description = "Fetch prices of some moneys. Optional date param: `YYYY-MM-DD`.")]
    Forex(Args),
}
