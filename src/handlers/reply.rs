use teloxide::prelude::*;

// example of command handler handling reply
pub(crate) async fn reply_handler(bot: Bot, msg: &Message) -> ResponseResult<()> {
    if let Some(reply_data) = msg.reply_to_message()
        && let Some(text) = reply_data.text()
    {
        bot.send_message(msg.chat.id, format!("reply: {}", text))
            .await?;
    }

    Ok(())
}
