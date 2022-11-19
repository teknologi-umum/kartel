mod commands;

use commands::Commands;
use std::env;
use teloxide::prelude::{Bot, Message, ResponseResult};

#[tokio::main]
async fn main() {
    let bot_token = env::var("BOT_TOKEN").unwrap_or_else(|_| String::from(""));

    // TODO: open sqlite database here
    let bot = Bot::new(bot_token);

    teloxide::repl(bot, handler).await;
}
async fn handler(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Bapack { point } => {
            // TODO: implement handler here (call to other file, not here)
        }
        Commands::Points { name, point } => {
            // TODO: implement handler here (call to other file, not here)
        }
    };

    Ok(())
}
