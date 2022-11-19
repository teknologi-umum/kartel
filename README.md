# Kartel

Telegram bot in rust. Comes from `karat` and `telegram`. `kar-tel`.

Main libs:

- [teloxide](https://github.com/teloxide/teloxide)
- [tokio](https://github.com/tokio-rs/tokio)
- [diesel-sqlite](https://github.com/teloxide/teloxide)

## Development

1. To install diesel CLI, you need to install client first if running debian based distrib

```sh
sudo apt install libsqlite3-dev default-libmysqlclient-dev libpq-dev
```

then run

```
cargo install diesel_cli
```

## Running in dev

Create `docker-compose-dev.yml` from `docker-compose-dev-example.yml` then just run:

```
docker build -t karteldev . && docker compose -f docker-compose-dev.yml up
```
or run `./run.sh`.