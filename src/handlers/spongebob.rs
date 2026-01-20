use anyhow::anyhow;
use crate::error::HandlerError;
use crate::commands::Args;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestReplyExt;

/// Convert input text into sPoNgEbOb mocking case by alternating ASCII alphabetic character case,
/// starting with lowercase on the first alphabetic character.
pub(crate) fn to_spongebob(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = false; // start with lowercase for first alphabetic char

    for ch in s.chars() {
        if ch.is_ascii_alphabetic() {
            if upper {
                out.push(ch.to_ascii_uppercase());
            } else {
                out.push(ch.to_ascii_lowercase());
            }
            upper = !upper;
        } else {
            out.push(ch);
        }
    }

    out
}

pub(crate) async fn spongebob_handler(bot: Bot, msg: &Message, args: Args) -> Result<(), HandlerError> {
    // If invoked by replying to someone, use their message text (or caption). Otherwise use args.
    let maybe_reply = msg.reply_to_message();

    let (src_text, reply_to_id) = if let Some(reply) = maybe_reply {
        if let Some(text) = reply.text() {
            (text.to_string(), reply.id)
        } else if let Some(caption) = reply.caption() {
            (caption.to_string(), reply.id)
        } else {
            return Err(HandlerError::InvalidArguments(anyhow!("Provide text or reply to a message")));
        }
    } else if !args.0.trim().is_empty() {
        (args.0.clone(), msg.id)
    } else {
        return Err(HandlerError::InvalidArguments(anyhow!("Provide text or reply to a message")));
    };

    let transformed = to_spongebob(&src_text);

    // Reply to the replied-to message when available; otherwise reply to the caller message.
    bot.send_message(msg.chat.id, transformed).reply_to(reply_to_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_spongebob_basic() {
        assert_eq!(to_spongebob("hello world"), "hElLo WoRlD");
    }

    #[test]
    fn test_to_spongebob_with_numbers() {
        assert_eq!(to_spongebob("test 123"), "tEsT 123");
    }

    #[test]
    fn test_to_spongebob_with_punctuation() {
        assert_eq!(to_spongebob("Hello, World!"), "hElLo, WoRlD!");
    }

    #[test]
    fn test_to_spongebob_empty() {
        assert_eq!(to_spongebob(""), "");
    }

    #[test]
    fn test_to_spongebob_numbers_only() {
        assert_eq!(to_spongebob("12345"), "12345");
    }
}
