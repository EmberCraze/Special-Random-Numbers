FROM rust:1.58.0

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/generator"]

