FROM rust:1.43 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new itopplus
WORKDIR /usr/src/itopplus
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/remove_image_file .
USER 0
RUN chmod 755 remove_image_file
CMD ["./remove_image_file"]