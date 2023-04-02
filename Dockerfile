FROM rust:alpine3.17

RUN rustup component add rust-src rust-analyzer \
   && mkdir -p /root/.cargo/bin \
   && ln -sf $(rustup which rust-analyzer) /root/.cargo/bin

ENV PATH="$PATH:/root/.cargo/bin"
