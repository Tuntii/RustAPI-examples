# Phase 11: Production Features Demo

Advanced production-ready features including guards, circuit breaker, and structured logging.

> 📖 **Cookbook**: [Production Tuning](https://tuntii.github.io/RustAPI/) · [FEATURES.md](../FEATURES.md)

## Prerequisites

- Rust 1.70+
- Advanced understanding of middleware patterns
- Completed [middleware-chain](../middleware-chain/) example

## Overview

This example demonstrates the new Phase 11 production-ready features added to RustAPI.

## Features Implemented

### 1. Request Guards (Authorization) 🔒

Declarative route protection with compile-time type safety.

```rust
use rustapi_extras::{RoleGuard, PermissionGuard};

#[rustapi_rs::get("/admin")]
async fn admin_only(guard: RoleGuard<"admin">) -> &'static str {
    "Admin area"
}

#[rustapi_rs::get("/users/delete")]
async fn delete_user(guard: PermissionGuard<"users.delete">) -> &'static str {
    "Delete user"
}
```

**Features:**
- Role-based access control with `RoleGuard<ROLE>`
- Permission-based guards with `PermissionGuard<PERMISSION>`
- Compile-time const generics for zero-cost abstractions
- Custom guard support

### 2. Request Timeout Middleware ⏱️

Automatic request timeout handling to prevent long-running requests.

```rust
use rustapi_extras::TimeoutLayer;
use std::time::Duration;

let app = RustApi::new()
    .layer(TimeoutLayer::from_secs(30))  // 30 second timeout
    .run("0.0.0.0:3000")
    .await?;
```

**Features:**
- Per-request timeout configuration
- Helper methods: `from_secs()`, `from_millis()`
- Graceful timeout handling with `tokio::time::timeout`
- Returns 408 Request Timeout on expiration

### 3. Health Check System 🏥

Flexible health check builder for monitoring application and dependency health.

```rust
use rustapi_core::health::{HealthCheckBuilder, HealthStatus};

let health = HealthCheckBuilder::new(true)
    .add_check("database", || async {
        // Check DB connection
        HealthStatus::healthy()
    })
    .add_check("redis", || async {
        // Check Redis
        HealthStatus::healthy()
    })
    .version("1.0.0")
    .build();

let result = health.execute().await;
```

**Features:**
- Three health states: `Healthy`, `Unhealthy`, `Degraded`
- Async check functions
- Version tracking
- JSON result output
- Aggregated status reporting

### 4. Structured Logging Middleware 📊

Production-ready request/response logging with multiple formats.

```rust
use rustapi_extras::{LoggingLayer, LogFormat};

let app = RustApi::new()
    .layer(
        LoggingLayer::new()
            .format(LogFormat::Detailed)
            .log_request_headers(true)
            .log_response_headers(true)
            .skip_path("/health")
    )
    .run("0.0.0.0:3000")
    .await?;
```

**Features:**
- Three formats: `Compact`, `Detailed`, `JSON`
- Correlation ID tracking
- Request/response header logging
- Body logging (configurable size limit)
- Skip path configuration for health checks

**Log Formats:**

- **Compact**: One line per request
  ```
  INFO incoming request request_id=abc123 method=GET uri=/api/users
  INFO request completed request_id=abc123 status=200 duration_ms=45
  ```

- **Detailed**: Multi-line with full details
  ```
  INFO === Incoming Request ===
  DEBUG request header: accept: application/json
  INFO === Response Sent === status=200 duration_ms=45
  ```

- **JSON**: Structured logging
  ```json
  {"type":"request","request_id":"abc123","method":"GET","uri":"/api/users"}
  {"type":"response","request_id":"abc123","status":200,"duration_ms":45}
  ```

### 5. Circuit Breaker Middleware ⚡

Resilient service call pattern to prevent cascading failures.

```rust
use rustapi_extras::CircuitBreakerLayer;
use std::time::Duration;

let app = RustApi::new()
    .layer(
        CircuitBreakerLayer::new()
            .failure_threshold(5)      // Open after 5 failures
            .timeout(Duration::from_secs(60))  // Wait 60s before trying again
            .success_threshold(2)      // Close after 2 successes
    )
    .run("0.0.0.0:3000")
    .await?;
```

**Features:**
- Three states: `Closed`, `Open`, `HalfOpen`
- Configurable failure threshold
- Automatic state transitions
- Success threshold for recovery
- Statistics tracking

**Circuit States:**

1. **Closed** (Normal): Requests pass through normally
2. **Open** (Failing): Too many failures, requests fail fast (503)
3. **HalfOpen** (Testing): After timeout, test if service recovered

## Running the Example

```bash
cd examples/phase11-demo
cargo run
```

The demo will start on `http://localhost:3000` with endpoints:

- `GET /` - Index page
- `GET /health` - Health check with multiple checks
- `GET /slow` - Demonstrates timeout (will timeout after 30s)
- `GET /admin` - Requires admin role
- `GET /users/edit` - Requires users.edit permission

## Feature Flags

Enable features in your `Cargo.toml`:

```toml
[dependencies]
rustapi-extras = { version = "0.9", features = [
    "timeout",
    "guard",
    "logging",
    "circuit-breaker",
] }
```

## Production Deployment Recommendations

### 1. Timeout Configuration
- API endpoints: 30s
- Database queries: 10s
- External API calls: 15s

### 2. Circuit Breaker Settings
- Failure threshold: 5-10 failures
- Timeout: 30-60 seconds
- Success threshold: 2-3 successes

### 3. Health Checks
- Include all critical dependencies (DB, cache, message queue)
- Use `/health` for basic health
- Use `/readiness` for Kubernetes readiness probes
- Use `/liveness` for Kubernetes liveness probes

### 4. Logging
- Development: `Detailed` format
- Production: `JSON` format (for log aggregation)
- Skip paths: `/health`, `/metrics`, `/favicon.ico`
- Enable header logging for debugging
- Disable body logging in production (privacy)

## Performance Impact

All Phase 11 features are designed for minimal overhead:

- **Guards**: Zero-cost const generics, compiled away
- **Timeout**: Single `tokio::time::timeout` wrapper (~1μs overhead)
- **Health Checks**: Async, only executed when endpoint called
- **Logging**: Structured tracing, filtered by level
- **Circuit Breaker**: Lock-free `RwLock`, O(1) state checks

## Next Steps

Explore more Phase 11 features:
- [ ] Retry middleware with exponential backoff
- [ ] Response caching (in-memory + Redis)
- [ ] Request deduplication
- [ ] Security headers middleware
- [ ] API key authentication

## License

MIT OR Apache-2.0
