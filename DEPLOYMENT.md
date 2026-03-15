# LibCut API — Deployment Guide

## Overview

LibCut API is an HTTP service for 2D guillotine cutting optimization of rectangular sheet materials. It accepts sheet dimensions and a list of parts, then returns an optimal placement layout that minimizes material waste.

**Stack:** Rust, Axum, Tokio
**Container:** Multi-stage Docker (rust:1.94 build → debian:bookworm-slim runtime)
**Default port:** 8080

---

## Table of Contents

- [Quick Start (Docker)](#quick-start-docker)
- [Build from Source](#build-from-source)
- [Configuration](#configuration)
- [Docker Compose](#docker-compose)
- [Reverse Proxy (nginx)](#reverse-proxy-nginx)
- [systemd Service](#systemd-service)
- [Health Check](#health-check)
- [Smoke Test](#smoke-test)

---

## Quick Start (Docker)

```bash
# Clone the repository
git clone https://github.com/zorgoalex/cutlib.git
cd cutlib

# Build the image
docker build -t libcut-api .

# Run
docker run -d \
  --name libcut-api \
  -p 8080:8080 \
  libcut-api

# Verify
curl http://localhost:8080/health
# {"status":"ok","service":"LibCut.Api"}
```

---

## Build from Source

### Prerequisites

- Rust 1.94+ ([rustup.rs](https://rustup.rs))

### Build & Run

```bash
# Release build
cargo build --release -p libcut_api

# Run
./target/release/libcut_api
# LibCut API listening on 0.0.0.0:8080
```

### Run Tests

```bash
cargo test --workspace
```

---

## Configuration

All configuration is done via environment variables.

| Variable | Type | Default | Description |
|---|---|---|---|
| `LIBCUT_PORT` | u16 | `8080` | TCP port the server listens on |
| `LIBCUT_MAX_CONCURRENT_OPTIMIZATIONS` | usize | `1` | Max concurrent optimization requests. Additional requests wait in queue. |

### Examples

```bash
# Change port
LIBCUT_PORT=3000 ./target/release/libcut_api

# Allow 4 concurrent optimizations
docker run -d \
  -e LIBCUT_PORT=8080 \
  -e LIBCUT_MAX_CONCURRENT_OPTIMIZATIONS=4 \
  -p 8080:8080 \
  libcut-api
```

### Concurrency Tuning

The optimization algorithm is CPU-bound. The concurrency gate limits how many requests execute simultaneously to prevent CPU exhaustion.

**Recommendations:**
- **1 (default)** — safe for single-core / low-memory environments
- **N = CPU cores** — maximizes throughput on dedicated hardware
- **N = CPU cores / 2** — leaves headroom for the OS and other processes

---

## Docker Compose

```yaml
# docker-compose.yml
services:
  libcut-api:
    build: .
    ports:
      - "127.0.0.1:8080:8080"   # localhost only — nginx proxies to this
    environment:
      LIBCUT_PORT: 8080
      LIBCUT_MAX_CONCURRENT_OPTIMIZATIONS: 2
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "cat < /dev/tcp/localhost/8080 || exit 1"]
      interval: 30s
      timeout: 5s
      retries: 3
      start_period: 5s
```

> **Note:** The base image (`debian:bookworm-slim`) does not include `curl`. To use the healthcheck above, either install curl in the Dockerfile runtime stage or use a TCP check instead:
>
> ```yaml
> healthcheck:
>   test: ["CMD-SHELL", "cat < /dev/tcp/localhost/8080 || exit 1"]
> ```

---

## Authentication (Bearer Token via nginx)

LibCut API has no built-in authentication. Access control is handled by nginx using a Bearer token.

### Generate a Token

```bash
# Generate a random token
openssl rand -hex 32
# Example output: a3f1c9e8b7d6...
```

Save the token in a file readable only by nginx:

```bash
sudo sh -c 'echo "a3f1c9e8b7d6..." > /etc/nginx/libcut_token'
sudo chmod 600 /etc/nginx/libcut_token
sudo chown root:root /etc/nginx/libcut_token
```

### Client Usage

All requests to `/api/*` must include the `Authorization` header:

```bash
curl -X POST https://cut.example.com/api/cut/optimize \
  -H "Authorization: Bearer a3f1c9e8b7d6..." \
  -H "Content-Type: application/json" \
  -d '{ "sheet": { "length": 2440, "width": 1220 }, "parts": [{ "length": 800, "width": 400 }] }'
```

The `/health` endpoint is open (no token required) for monitoring and load balancer probes.

### Token Rotation

1. Generate a new token
2. Update `/etc/nginx/libcut_token`
3. `sudo nginx -s reload` — zero-downtime, no service restart

---

## Reverse Proxy (nginx)

### HTTP-only (dev / internal)

```nginx
upstream libcut {
    server 127.0.0.1:8080;
}

server {
    listen 80;
    server_name cut.example.com;

    location / {
        proxy_pass http://libcut;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        proxy_read_timeout 120s;
        proxy_send_timeout 120s;
        client_max_body_size 1m;
    }
}
```

### Production (TLS + Bearer Token)

```nginx
upstream libcut {
    server 127.0.0.1:8080;
}

# Read token from file into variable
map "" $libcut_token {
    default "";
}

geo $libcut_token_file {
    default /etc/nginx/libcut_token;
}

server {
    listen 80;
    server_name cut.example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name cut.example.com;

    ssl_certificate     /etc/letsencrypt/live/cut.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/cut.example.com/privkey.pem;
    ssl_protocols       TLSv1.2 TLSv1.3;
    ssl_ciphers         HIGH:!aNULL:!MD5;

    client_max_body_size 1m;

    # Health endpoint — open, no auth
    location = /health {
        proxy_pass http://libcut;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # API endpoints — require Bearer token
    location /api/ {
        # Check Authorization header
        set $expected_token "Bearer PASTE_YOUR_TOKEN_HERE";

        if ($http_authorization = "") {
            return 401 '{"type":"about:blank","title":"Unauthorized","status":401,"detail":"Missing Authorization header."}';
        }
        if ($http_authorization != $expected_token) {
            return 403 '{"type":"about:blank","title":"Forbidden","status":403,"detail":"Invalid token."}';
        }

        proxy_pass http://libcut;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Strip Authorization header — libcut doesn't need it
        proxy_set_header Authorization "";

        proxy_read_timeout 120s;
        proxy_send_timeout 120s;
    }

    # Block everything else
    location / {
        return 404 '{"type":"about:blank","title":"Not Found","status":404,"detail":"Unknown endpoint."}';
    }

    # Error responses in JSON
    default_type application/json;
}
```

### TLS Certificate (Let's Encrypt)

```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d cut.example.com
# Auto-renewal is configured by certbot
```

---

## systemd Service

```ini
# /etc/systemd/system/libcut-api.service
[Unit]
Description=LibCut Cutting Optimization API
After=network.target

[Service]
Type=simple
User=libcut
Group=libcut
ExecStart=/usr/local/bin/libcut_api
Environment=LIBCUT_PORT=8080
Environment=LIBCUT_MAX_CONCURRENT_OPTIMIZATIONS=2
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```bash
# Install binary
sudo cp target/release/libcut_api /usr/local/bin/

# Create service user
sudo useradd --system --no-create-home libcut

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable --now libcut-api

# Check status
sudo systemctl status libcut-api
journalctl -u libcut-api -f
```

---

## Health Check

```bash
curl http://localhost:8080/health
```

Response:
```json
{"status": "ok", "service": "LibCut.Api"}
```

Use this endpoint for load balancer probes, Docker healthchecks, and uptime monitoring.

---

## Smoke Test

A built-in smoke test script validates the full Docker pipeline:

```bash
bash scripts/docker_smoke.sh
```

The script:
1. Builds the Docker image
2. Creates an isolated Docker network
3. Starts the API container
4. Sends test requests via `curlimages/curl` container
5. Validates the response (expects 2 sheets, 19 parts)
6. Cleans up all resources automatically
