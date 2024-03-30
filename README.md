# Axum-Postgres Starter

Starter project for an API server using Axum and Postgres.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker)

```sh
# Start Docker
docker compose up -d

# Install sqlx and cargo watch CLIs
cargo install sqlx-cli cargo-watch

# Add migration
sqlx migrate add -r <name>

# Run migrations
sqlx migrate run

# Start the server
cargo run

# Start the server with watch
cargo watch -qcw src -x run

# Enter db shell
docker exec -it postgres bash

# Enter db psql shell
docker exec -it postgres psql -U postgres -d dev_db
```
