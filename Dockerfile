FROM rust:1.72-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get upgrade -y && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/apeing_ws /usr/local/bin/apeing_ws
COPY .env.example /app/.env
WORKDIR /app
CMD ["apeing_ws"]