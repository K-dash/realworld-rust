FROM rust:1.75.0-slim-bookworm

RUN apt-get update \
    && apt-get install -y build-essential git \ 
    pkg-config libssl-dev \
    && rm -fr /var/lib/apt/lists/*

RUN adduser rustuser --gecos "" --disabled-password

# RUN rustup component add rustfmt clippy \
#     && cargo install cargo-watch
RUN rustup component add rustfmt clippy

# /usr/local/cargoの権限をrustuserに変更
RUN chown -R rustuser:rustuser /usr/local/cargo

USER rustuser

WORKDIR /usr/src/myapp
