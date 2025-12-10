use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
    types::{ParseMode, Update},
    update_listeners::webhooks,
};

use crate::error::SendIfError;

mod commands;
mod config;
mod deps;
mod error;
mod handlers;
use config::config;

static WEBHOOK_ENDPOINT: &'static str = "https://api.mfirhas.com/webhook";

#[tokio::main]
async fn main() {
    let bot = Bot::new(config().bot_token.clone());

    // dev/local mode
    if cfg!(debug_assertions) {
        println!("kartel started in dev mode...");
        commands::Command::repl(bot, handlers).await;
        return ();
    }

    println!("kartel started in production mode...");

    let webhook_url = WEBHOOK_ENDPOINT
        .parse()
        .expect("failed parsing webhook url");

    let addr = ([0, 0, 0, 0], config().webhook_port).into();

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, webhook_url))
        .await
        .expect("Couldn't setup webhook");

    Dispatcher::builder(bot, handler())
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;

    ()
}

fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry().branch(
        Update::filter_message()
            .filter_command::<crate::commands::Command>()
            .endpoint(
                |bot: Bot, msg: Message, cmd: crate::commands::Command| async move {
                    handlers(bot, msg, cmd)
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                },
            ),
    )
}

async fn handlers(bot: Bot, msg: Message, cmd: crate::commands::Command) -> ResponseResult<()> {
    match cmd {
        commands::Command::Help => handlers::help::help_handler(bot, &msg).await?,

        commands::Command::Forex(args) => {
            handlers::forex::forex_handler(bot.clone(), &msg, args)
                .await
                .send_if_err(bot, &msg)
                .await?
        }

        commands::Command::Rates(args) => {
            // TODO
            bot.send_message(msg.chat.id, "Coming soon...!")
                .parse_mode(ParseMode::Html)
                .await?;
            ()
        }

        commands::Command::Gold(args) => {
            // TODO
            bot.send_message(msg.chat.id, "Coming soon...!")
                .parse_mode(ParseMode::Html)
                .await?;
            ()
        }

        commands::Command::Silver(args) => {
            // TODO
            bot.send_message(msg.chat.id, "Coming soon...!")
                .parse_mode(ParseMode::Html)
                .await?;
            ()
        }

        commands::Command::Zakat(args) => {
            // TODO
            bot.send_message(msg.chat.id, "Coming soon...!")
                .parse_mode(ParseMode::Html)
                .await?;
            ()
        }

        commands::Command::Stock(args) => {
            // TODO
            bot.send_message(msg.chat.id, "Coming soon...!")
                .parse_mode(ParseMode::Html)
                .await?;
            ()
        }
    }

    Ok(())
}
