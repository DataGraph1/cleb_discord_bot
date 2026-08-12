# Build stage
FROM rust:bookworm AS builder

WORKDIR /app

# Copy dependency files first so Docker can cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy project to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source
COPY src ./src
COPY migrations ./migrations

# Build the bot
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Required for HTTPS/TLS connections
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cleb_discord_bot /app/cleb_discord_bot

CMD ["/app/cleb_discord_bot"]