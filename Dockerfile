FROM rust:1.81.0

WORKDIR /app

# Copy the source code
COPY . /app

RUN cargo test

RUN cargo build --release

ENTRYPOINT ["/app/target/release/fibbot"]


