use teloxide::prelude::*;

mod commands;
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
        commands::Command::Help => handlers::help::help_handler(&bot, &msg).await?,
        commands::Command::Test(args) => {
            handlers::test::test_handler(&bot, &msg, args.try_into()?).await?
        }
        commands::Command::Reply => handlers::reply::reply_handler(&bot, &msg).await?,
    }

    Ok(())
}
