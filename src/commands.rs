use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase", description = "")]
pub(crate) enum Commands {
    #[command(description = "")]
    Points { name: String, point: String },
    #[command(description = "")]
    Bapack { point: String },
}
