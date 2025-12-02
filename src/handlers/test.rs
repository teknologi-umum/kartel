use anyhow::{Context, anyhow};
use teloxide::prelude::*;

use super::error::HandlerError;
use crate::{commands::Args, handlers::error::AsClientError};

// example of command handler handling arguments and parsing it into handler arguments type

pub(crate) struct TestArgs {
    x: i32,
    y: i32,
}

impl TryFrom<Args> for TestArgs {
    type Error = HandlerError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let a = value
            .0
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>();

        if a.len() != 2 {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "invalid arguments for test command: {:?}",
                a
            )));
        }

        let x: i32 = a[0]
            .parse()
            .context("test command invalid first argument: must be a number")
            .as_client_err()?;

        let y: i32 = a[1]
            .parse()
            .context("test command invalid second argument: must be a number")
            .as_client_err()?;

        Ok(TestArgs { x, y })
    }
}

pub(crate) async fn test_handler(bot: Bot, msg: &Message, args: TestArgs) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, format!("test: {} {}", args.x, args.y))
        .await?;

    Ok(())
}
