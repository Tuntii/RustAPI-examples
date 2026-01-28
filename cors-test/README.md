# CORS Test Example

A minimal example demonstrating Cross-Origin Resource Sharing (CORS) configuration with RustAPI.

> 📖 **Cookbook**: [CORS Configuration](https://tuntii.github.io/RustAPI/)

## Overview

This example shows how to configure CORS for your RustAPI application, enabling cross-origin requests from web browsers. Essential for:

- Frontend applications on different domains
- Single Page Applications (SPAs)
- Mobile apps using web views
- Third-party API integrations

## Prerequisites

- Rust 1.70+
- Understanding of CORS concepts
- Basic [hello-world](../hello-world/) knowledge

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `CorsLayer::permissive()` | Allow all origins (development) |
| `CorsLayer::new()` | Custom CORS configuration |
| Layer composition | Multiple middleware layers |

## Quick Start

```bash
# Run the example
cargo run -p cors-test

# Server starts at http://127.0.0.1:3030
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Hello message with CORS headers |

## Testing

### Test CORS Headers

```bash
# Simple request
curl -v http://127.0.0.1:3030/

# Preflight request (OPTIONS)
curl -v -X OPTIONS http://127.0.0.1:3030/ \
  -H "Origin: http://localhost:3000" \
  -H "Access-Control-Request-Method: GET"

# Check response headers for:
# Access-Control-Allow-Origin: *
# Access-Control-Allow-Methods: GET, POST, ...
# Access-Control-Allow-Headers: ...
```

### Test from Browser Console

```javascript
// Open browser console on any page
fetch('http://127.0.0.1:3030/')
  .then(r => r.text())
  .then(console.log);
// Should work without CORS errors
```

## Code Walkthrough

### Permissive CORS (Development)

```rust
RustApi::new()
    .route("/", get(hello))
    .layer(CorsLayer::permissive())  // Allow all origins
    .run("127.0.0.1:3030")
    .await
```

`CorsLayer::permissive()` allows:
- All origins (`*`)
- All standard methods
- All headers
- Credentials

⚠️ **Use only for development!**

### Production CORS Configuration

```rust
use rustapi_rs::middleware::CorsLayer;

let cors = CorsLayer::new()
    .allow_origins([
        "https://example.com",
        "https://app.example.com",
    ])
    .allow_methods(["GET", "POST", "PUT", "DELETE"])
    .allow_headers(["Content-Type", "Authorization"])
    .allow_credentials(true)
    .max_age(3600);  // Cache preflight for 1 hour

RustApi::new()
    .route("/", get(hello))
    .layer(cors)
    .run("127.0.0.1:3030")
    .await
```

## CORS Configuration Options

| Option | Description | Example |
|--------|-------------|---------|
| `allow_origins()` | Allowed origin domains | `["https://example.com"]` |
| `allow_methods()` | Allowed HTTP methods | `["GET", "POST"]` |
| `allow_headers()` | Allowed request headers | `["Content-Type"]` |
| `expose_headers()` | Headers client can access | `["X-Custom-Header"]` |
| `allow_credentials()` | Allow cookies/auth | `true` |
| `max_age()` | Preflight cache duration | `3600` (seconds) |

## Common Configurations

### API with Authentication

```rust
CorsLayer::new()
    .allow_origins(["https://app.example.com"])
    .allow_methods(["GET", "POST", "PUT", "DELETE"])
    .allow_headers(["Content-Type", "Authorization"])
    .allow_credentials(true)
```

### Public API

```rust
CorsLayer::new()
    .allow_origins(["*"])  // Any origin
    .allow_methods(["GET"])
    .max_age(86400)  // Cache for 24 hours
```

### Multiple Frontends

```rust
CorsLayer::new()
    .allow_origins([
        "https://web.example.com",
        "https://admin.example.com",
        "http://localhost:3000",  // Development
    ])
```

## Layer Composition

This example also demonstrates combining multiple layers:

```rust
RustApi::new()
    .route("/", get(hello))
    .layer(CorsLayer::permissive())
    .layer(RequestIdLayer::new())
    .layer(TracingLayer::new())
    .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
    .run("127.0.0.1:3030")
    .await
```

**Execution order** (outside → inside):
1. CORS (handles preflight)
2. Request ID (adds tracking)
3. Tracing (logs request)
4. Rate Limit (checks limits)
5. Handler

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.1", features = ["cors", "rate-limit"] }
tokio = { version = "1", features = ["full"] }
serde = "1"
```

## Troubleshooting

### "CORS error" in Browser

1. Check `Access-Control-Allow-Origin` header matches your origin
2. Ensure preflight (OPTIONS) is handled
3. Verify `allow_credentials()` if using cookies

### Credentials Not Working

```rust
// Must specify exact origins (not "*") when using credentials
CorsLayer::new()
    .allow_origins(["https://specific-domain.com"])  // Not "*"
    .allow_credentials(true)
```

### Custom Headers Not Accessible

```rust
CorsLayer::new()
    .expose_headers(["X-Custom-Header", "X-Request-Id"])
```

## Next Steps

- **[middleware-chain](../middleware-chain/)** — Custom middleware creation
- **[auth-api](../auth-api/)** — CORS with authentication

## Related Documentation

- [FEATURES.md](../FEATURES.md#cors) — CORS feature reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/) — Full documentation

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
