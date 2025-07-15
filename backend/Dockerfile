# ---------- build ----------
FROM rust:1.88-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev
COPY . .
ENV SQLX_OFFLINE true
RUN cargo fetch
RUN cargo build --release

# ---------- runtime ----------
FROM debian:12.11-slim
WORKDIR /app
ENV RUST_LOG=info
COPY --from=builder /app/target/release/backend .
COPY ./migrations ./migrations
CMD ["./backend"]
