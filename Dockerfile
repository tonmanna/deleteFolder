FROM rust:1.43
WORKDIR /usr/src/app
COPY . .
RUN cargo install
CMD ["remove_image_file"]