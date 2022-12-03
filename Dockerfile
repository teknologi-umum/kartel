# syntax=docker/dockerfile:1
FROM rust:1.65.0-bullseye as build-env
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

FROM debian:bullseye as run-env
RUN apt-get update && apt-get install -y curl ca-certificates openssl sqlite3 libsqlite3-dev
COPY --from=build-env /app/target/release/kartel /usr/bin/kartel
CMD ["/usr/bin/kartel"]