FROM rust:alpine AS builder

WORKDIR /usr/src/fibbot

COPY . .

RUN cargo build --release 

FROM alpine:latest

COPY --from=builder /usr/src/fibbot/target/release/fibbot /

CMD ["./fibbot"] 