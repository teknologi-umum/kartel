mod handler;
mod model;

use crate::handler::CommandHandler;
use crate::model::Model;
use rusqlite::Connection;
use std::env;
use teloxide::prelude::*;
use teloxide::types::ChatKind::Private;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

#[tokio::main]
async fn main() {
    let bot_token = env::var("BOT_TOKEN").unwrap_or(String::from(""));
    let sentry_dsn = env::var("SENTRY_DSN").unwrap_or(String::from(""));
    let database_path = env::var("DATABASE_PATH").unwrap_or(String::from("kartel.db"));

    let database = Connection::open(database_path).unwrap();
    let _guard = sentry::init(sentry_dsn);
    let bot = Bot::new(bot_token);

    let model = Model::new(database);
    let command_handler = CommandHandler::new(model);

    Commands::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "")]
pub enum Commands {
    #[command(description = "", parse_with = "split")]
    Points { name: String, point: String },
    #[command(description = "")]
    Bapack { point: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Points { name, point } => {
            if let Private(_) = msg.chat.kind {
                // TODO
            };
        }
        Commands::Bapack { point } => {
            if let Private(_) = msg.chat.kind {
                // TODO
            };
        }
    };

    Ok(())
}
