# Build the binary
FROM rust:1.73.0 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Bookworm is the same base image rust uses for its official images
# pull it in and only copy over binary for a lightweight image
FROM debian:bookworm-slim
WORKDIR /usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 5000

COPY --from=builder /usr/src/app/target/release/google-oauth-axum-starter ./app

CMD ["./app"]