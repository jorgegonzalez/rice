# Use official Rust image as build environment
FROM rust:1.88-slim AS builder

# Set working directory
WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs and lib.rs to build dependencies
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs

# Build dependencies, then drop the dummy artifacts
RUN cargo build --release --locked \
    && rm -rf src target/release/deps/rice* target/release/rice*

# Copy source code
COPY src ./src

# Build the actual application
RUN cargo build --release --locked

# Runtime stage - use Ubuntu for better GLIBC compatibility  
FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rice /usr/local/bin/rice

# Set the entrypoint
ENTRYPOINT ["rice"]