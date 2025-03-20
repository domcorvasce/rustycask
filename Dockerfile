FROM rust:1.85

WORKDIR /code

RUN rustup component add rustfmt
