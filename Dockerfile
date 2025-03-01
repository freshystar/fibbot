FROM rust:alpine AS build-env

WORKDIR /app

# Copy the source code
COPY . /app

# Install necessary build dependencies
RUN apk add --no-cache musl-dev upx

RUN cargo build --release

# Compress the binary using upx
RUN upx --best target/release/group-generator

# Use a minimal base image for the final image
FROM scratch

COPY --from=build-env /app/target/release/group-generator /app

ENTRYPOINT ["/app/target/release/fibbot"]



# FROM rust:latest

# WORKDIR /app

# COPY . /app

# RUN cargo build --release 

# ENTRYPOINT ["/app/target/release/fibbot"]



