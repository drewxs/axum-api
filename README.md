# Axum-Postgres Starter

Starter project for an API server using Axum and Postgres.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker)

```sh
# Start Docker
docker compose up -d

# Install sqlx-cli
cargo install sqlx-cli

# Install cargo watch
cargo install cargo-watch

# Add migration
sqlx migrate add -r <name>

# Run migrations
sqlx migrate run

# Start the server
cargo run

# Start the server with watch
cargo watch -q -c -w src/ -x run
```

