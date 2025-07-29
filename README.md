# Apeing.ai WebSocket Service

Streams real-time `token_created` events from the Pump.fun Solana program to WebSocket clients.

## Features

- Connects over Solana WebSocket RPC
- Parses transaction logs for token creation
- Broadcasts events to all WS clients
- Optional client-side filtering & rate limiting
- Prometheus metrics endpoint
- Graceful shutdown on Ctrl+C
- CLI configuration via flags/env
- Docker & docker-compose support

## Quickstart

```bash
# 1. Clone & enter
git clone <repo>
cd apeing_ws

# 2. Copy env example
cp .env.example .env

# 3. Build & run
cargo run --release

# 4. Point your WS client at ws://localhost:8080/ws
```
