# Build stage
FROM rust:1.73-slim-bookworm as builder

WORKDIR /app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Build in release mode
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/daemon /usr/local/bin/daemon

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/usr/local/bin/daemon"]
