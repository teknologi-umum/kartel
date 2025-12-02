use teloxide::prelude::*;

use crate::handlers::error::SendIfError;

mod commands;
mod deps;
mod handlers;

#[tokio::main]
async fn main() {
    let bot = Bot::new("token");

    println!("kartel started...");

    commands::Command::repl(bot, handlers).await;

    ()
}

async fn handlers(bot: Bot, msg: Message, cmd: crate::commands::Command) -> ResponseResult<()> {
    match cmd {
        commands::Command::Help => handlers::help::help_handler(bot, &msg).await?,
        commands::Command::Test(args) => {
            handlers::test::test_handler(bot, &msg, args.try_into()?).await?
        }
        commands::Command::Reply => handlers::reply::reply_handler(bot, &msg).await?,
        commands::Command::Forex(args) => {
            handlers::forex::forex_handler(bot.clone(), &msg, args)
                .await
                .send_if_err(bot, &msg)
                .await?
        }
    }

    Ok(())
}
