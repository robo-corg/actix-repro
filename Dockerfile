FROM rust:latest

COPY src /app/actix-repro/src
COPY Cargo.toml Cargo.lock /app/actix-repro/

WORKDIR /app/actix-repro

RUN cargo build --release

ENV UPLOADER_PATH=/app/actix-repro/target/release/uploader
