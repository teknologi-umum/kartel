# Kartel

Telegram bot in rust. Comes from `karat` and `telegram`. `kar-tel`.

This bot use [Teloxide](https://github.com/teloxide/teloxide) framework to interact with Telegram.

There are 2 mode, local/dev mode which use long polling and prod mode which use webhook.

Local/dev mode run in local which only need telegram bot token to run, while prod mode requires bot token and webhook endpoint for telegram to hit.

Bot username is: @kartel_teknumbot

For webhook endpoint, it hits into this app hosted in a VPS.

## Add New Command
All commands defined inside `src/commands.rs` file. Simply add your new command(s) there and create the handler in `src/handlers/your-command-handler.rs`. Then you can register command -> handler mapping in `src/main.rs` file.

The function signature for the handler is:
```rust
pub(crate) async fn forex_handler(bot: Bot, msg: &Message, args: Args) -> Result<(), HandlerError>
```
but if your command doesn't pass arguments, just omit it:
```rust
pub(crate) async fn forex_handler(bot: Bot, msg: &Message) -> Result<(), HandlerError>
```

Write conversion from `Args` to your handler type.