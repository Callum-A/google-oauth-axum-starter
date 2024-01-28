FROM rust:1.73.0
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
CMD ["google-oauth-axum-starter"]