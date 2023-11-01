# FROM rust AS builder
# WORKDIR /app
# RUN rustup target add $(uname -m)-unknown-linux-musl

# RUN USER=root cargo new basic-strategies
# WORKDIR /app/basic-strategies
# COPY ./dummy.rs ./src/main.rs
# COPY Cargo.toml ./
# COPY Cargo.lock ./
# RUN cargo build --release --target $(uname -m)-unknown-linux-musl

# COPY src ./src
# RUN cargo build --features server --release --target $(uname -m)-unknown-linux-musl
# RUN mv ./target/$(uname -m)-unknown-linux-musl/release/basic-strategies /

# FROM scratch as runtime
# WORKDIR /
# COPY --from=builder /basic-strategies /
# ENTRYPOINT ["/basic-strategies"]

FROM alpine as minibuilder
WORKDIR /
COPY ./target-x86_64/x86_64-unknown-linux-musl/release/basic-strategies /basic-strategies-x86_64
COPY ./target-aarch64/aarch64-unknown-linux-musl/release/basic-strategies /basic-strategies-aarch64
RUN mv /basic-strategies-$(uname -m) /basic-strategies

FROM scratch as runtime
WORKDIR /
COPY --from=minibuilder /basic-strategies /
ENTRYPOINT ["/basic-strategies"]