FROM rust:1.43 AS builder
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new itopplus
WORKDIR /usr/src/itopplus
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN pwd & ls target/release

FROM rust:slim-stretch
COPY --from=builder /usr/src/itopplus/target/release/remove_image_file /usr/bin
RUN chmod 755 /usr/bin/remove_image_file

CMD ["./usr/bin/remove_image_file"]