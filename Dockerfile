FROM rust:1.37
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path ./lab1
CMD ["lab1"]