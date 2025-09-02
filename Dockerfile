# Use official Rust image as build environment
FROM rust:latest as builder

# Set working directory
WORKDIR /app

# Copy Cargo.toml first for dependency caching
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will generate a new Cargo.lock)
RUN cargo build --release && rm -rf src target/release/deps/rice*

# Copy source code
COPY src ./src

# Build the actual application
RUN cargo build --release

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