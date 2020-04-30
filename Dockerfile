FROM rust:1.43 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new deciduously-com
WORKDIR /usr/src/deciduously-com
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/remove_image_file .
USER 0
CMD ["./remove_image_file"]