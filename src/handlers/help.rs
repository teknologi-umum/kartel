use crate::error::HandlerError;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

pub(crate) async fn help_handler(bot: Bot, msg: &Message) -> Result<(), HandlerError> {
    bot.send_message(
        msg.chat.id,
        crate::commands::Command::descriptions().to_string(),
    )
    .await?;

    Ok(())
}
