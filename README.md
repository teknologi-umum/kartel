# Kartel

Telegram bot in rust. Comes from `karat` and `telegram`. `kar-tel`.

This bot use [Teloxide](https://github.com/teloxide/teloxide) framework to interact with Telegram.

There are 2 mode, local/dev mode which use long polling and prod mode which use webhook.

Local/dev mode run in local which only need telegram bot token to run, while prod mode requires bot token and webhook endpoint for telegram to hit.

Create Your bot and its token in telegram with @BotFather.

Webhook endpoint will point to the server running this code. Make sure the endpoint is reachable publicly.

## Codebase Structure
This repo has simple project structure. They are:
At the root:
- Dockerfile
- docker-compose.yml
- Cargo.toml

The source code is inside `src/` directory:
- `main.rs`: entrypoint for executable.
- `error.rs`: contains error handling helpers.
- `config.rs`: contains configuration reading from env vars.
- `commands.rs`: contains list of bot commands.
- *handlers*: contains all commands implementations defined in `commands.rs`. Each implementation can be in single file or inside a directory, depends on complexity.
- *deps*: contains all dependencies. Many dependencies are statics or `clone`. Dependencies as much as it can initialized once and used everywhere as global vars.

Additional codes can be added into module like `utils` or `utils.rs`.

No unsafe code allowed as per lint restriction.

## Running locally
To run it locally, you need to add env vars defined in `src/config.rs` in struct `Config`. The aliases are the env vars name.

## Adding New Command
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