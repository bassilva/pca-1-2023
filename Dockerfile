FROM rust:alpine3.17

RUN rustup component add rust-src rust-analyzer

WORKDIR /root
