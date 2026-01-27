FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy source
COPY . .

# Build specific binaries
RUN cargo build --release -p microservices-advanced

# Runtime base
FROM debian:bookworm-slim as runtime
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*

# Registry
FROM runtime as registry
COPY --from=builder /app/target/release/service-registry /usr/local/bin/
CMD ["service-registry"]

# Product Service
FROM runtime as product-service
COPY --from=builder /app/target/release/product-service /usr/local/bin/
CMD ["product-service"]

# Order Service
FROM runtime as order-service
COPY --from=builder /app/target/release/order-service /usr/local/bin/
CMD ["order-service"]
