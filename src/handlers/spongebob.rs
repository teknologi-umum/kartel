use anyhow::anyhow;
use rand::Rng as _;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestReplyExt;

use crate::commands::Args;
use crate::error::HandlerError;

/// Convert text to sPoNgEbOb mocking case by alternating the case of ASCII alphabetic characters.
/// Randomly starts with uppercase or lowercase for the first alphabetic character, then alternates.
fn to_spongebob(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rng = rand::thread_rng();
    let mut lowercase_next = rng.r#gen::<bool>();

    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            if lowercase_next {
                result.push(ch.to_ascii_lowercase());
            } else {
                result.push(ch.to_ascii_uppercase());
            }
            lowercase_next = !lowercase_next;
        } else {
            result.push(ch);
        }
    }

    result
}

pub(crate) async fn spongebob_handler(bot: Bot, msg: &Message, args: Args) -> Result<(), HandlerError> {
    // Check if the command is a reply to another message
    if let Some(reply_to_msg) = msg.reply_to_message() {
        // Get text from the replied message (try text first, then caption)
        let text_to_convert = reply_to_msg
            .text()
            .or_else(|| reply_to_msg.caption())
            .ok_or_else(|| HandlerError::InvalidArguments(anyhow!("Replied message has no text or caption")))?;

        let mocked_text = to_spongebob(text_to_convert);

        // Reply to the message that was replied to
        bot.send_message(msg.chat.id, mocked_text)
            .reply_to(reply_to_msg.id)
            .await?;
    } else {
        // If not a reply, check if args were provided
        let text_to_convert = args.0.trim();

        if text_to_convert.is_empty() {
            return Err(HandlerError::InvalidArguments(anyhow!(
                "No text provided. Either provide text after the command or reply to a message."
            )));
        }

        let mocked_text = to_spongebob(text_to_convert);

        // Send the mocked text without replying to the command
        bot.send_message(msg.chat.id, mocked_text)
            .await?;
    }

    // Delete the caller's message (the command message)
    // Ignore errors if bot lacks permission to delete messages
    let _ = bot.delete_message(msg.chat.id, msg.id).await;

    Ok(())
}
