FROM rust:latest

COPY . .

RUN cargo build --release 

ENTRYPOINT ["/app/target/release/fibbot"]

