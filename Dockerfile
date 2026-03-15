FROM rust:1.94-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release -p libcut_api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/libcut_api /usr/local/bin/libcut_api

ENV LIBCUT_PORT=8080
ENV LIBCUT_MAX_CONCURRENT_OPTIMIZATIONS=1

EXPOSE 8080

CMD ["libcut_api"]
