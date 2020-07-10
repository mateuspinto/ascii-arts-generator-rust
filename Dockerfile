FROM rust:latest

WORKDIR /usr/src/ascii-arts-generator-ufv-rust

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/ascii-arts-generator-ufv-rust"]
