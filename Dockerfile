FROM rust:latest

COPY . .

RUN cargo build --release 

CMD ["/app/target/release/fibbot"]

