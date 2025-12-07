use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase", description = "")]
pub(crate) enum Commands {
    #[command(description = "")]
    Points,
    #[command(description = "")]
    Bapack,
}
