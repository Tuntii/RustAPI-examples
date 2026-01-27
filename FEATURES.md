# RustAPI Feature Flags Reference

This document provides a comprehensive reference for all RustAPI Cargo feature flags used across the examples in this repository.

> 📖 **Related**: [RustAPI Crates Documentation](https://tuntii.github.io/RustAPI/)

---

## Overview

RustAPI uses Cargo feature flags to enable optional functionality. This modular approach keeps binary sizes small and compile times fast while allowing access to powerful features when needed.

```toml
[dependencies]
rustapi-rs = { version = "0.2", features = ["jwt", "cors"] }
```

---

## Feature Flags

### `full`

**Enables all features** — Use this when you need everything or for rapid prototyping.

```toml
rustapi-rs = { version = "0.2", features = ["full"] }
```

**Includes**: jwt, cors, rate-limit, toon, ws, view, swagger-ui, and all middleware

**Used in examples**:
- [crud-api](crud-api/) — Complete CRUD with all features
- [phase11-demo](phase11-demo/) — Advanced middleware demonstration

**When to use**:
- ✅ Prototyping and experimentation
- ✅ When you need multiple features
- ❌ Production (prefer explicit features for smaller binaries)

---

### `jwt`

**JWT Authentication** — Adds `JwtLayer` middleware and `AuthUser<T>` extractor.

```toml
rustapi-rs = { version = "0.2", features = ["jwt"] }
```

**Provides**:
- `JwtLayer<Claims>` — Middleware for JWT validation
- `AuthUser<T>` — Extractor for authenticated user claims
- `JwtConfig` — Configuration for token generation/validation
- Skip paths configuration for public routes

**Used in examples**:
- [auth-api](auth-api/) — Full authentication system
- [middleware-chain](middleware-chain/) — JWT with other middleware
- [phase11-demo](phase11-demo/) — Advanced auth patterns
- [proof-of-concept](proof-of-concept/) — Real-world auth implementation

**Example usage**:
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::middleware::JwtLayer;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Protected route - requires valid JWT
#[get("/profile")]
async fn profile(auth: AuthUser<Claims>) -> Json<Claims> {
    Json(auth.claims)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let jwt_layer = JwtLayer::<Claims>::new("your-secret-key")
        .skip_paths(vec!["/login", "/register", "/docs"]);

    RustApi::auto()
        .layer(jwt_layer)
        .run("127.0.0.1:8080")
        .await
}
```

**Cookbook**: [JWT Authentication Recipe](https://tuntii.github.io/RustAPI/)

---

### `cors`

**Cross-Origin Resource Sharing** — Adds `CorsLayer` for configuring CORS headers.

```toml
rustapi-rs = { version = "0.2", features = ["cors"] }
```

**Provides**:
- `CorsLayer` — Configurable CORS middleware
- Origin, method, and header allowlists
- Credentials and max-age configuration

**Used in examples**:
- [cors-test](cors-test/) — CORS configuration patterns
- [middleware-chain](middleware-chain/) — CORS with other layers
- [proof-of-concept](proof-of-concept/) — Production CORS setup

**Example usage**:
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::middleware::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cors = CorsLayer::new()
        .allow_origins(["https://example.com", "http://localhost:3000"])
        .allow_methods(["GET", "POST", "PUT", "DELETE"])
        .allow_headers(["Content-Type", "Authorization"])
        .allow_credentials(true)
        .max_age(3600);

    RustApi::auto()
        .layer(cors)
        .run("127.0.0.1:8080")
        .await
}
```

**Cookbook**: [CORS Configuration](https://tuntii.github.io/RustAPI/)

---

### `rate-limit`

**Rate Limiting** — Adds `RateLimitLayer` for IP-based request throttling.

```toml
rustapi-rs = { version = "0.2", features = ["rate-limit"] }
```

**Provides**:
- `RateLimitLayer` — IP-based rate limiting middleware
- Configurable requests per second/minute
- Burst support
- Automatic `429 Too Many Requests` responses
- Rate limit headers (`X-RateLimit-*`)

**Used in examples**:
- [rate-limit-demo](rate-limit-demo/) — Rate limiting patterns
- [auth-api](auth-api/) — Protect auth endpoints
- [cors-test](cors-test/) — Combined with CORS
- [proof-of-concept](proof-of-concept/) — Production rate limiting

**Example usage**:
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::middleware::RateLimitLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 100 requests per minute with burst of 10
    let rate_limit = RateLimitLayer::new()
        .requests_per_minute(100)
        .burst_size(10);

    RustApi::auto()
        .layer(rate_limit)
        .run("127.0.0.1:8080")
        .await
}
```

**Cookbook**: [Rate Limiting](https://tuntii.github.io/RustAPI/)

---

### `toon`

**Token-Oriented Object Notation** — AI-optimized response format with 50-58% token savings.

```toml
rustapi-rs = { version = "0.2", features = ["toon"] }
```

**Provides**:
- `ToonResponse<T>` — Response type with TOON serialization
- `Accept` header content negotiation
- `X-Token-Count` header for token metrics
- Automatic JSON/TOON format switching

**Used in examples**:
- [toon-api](toon-api/) — TOON format demonstration
- [mcp-server](mcp-server/) — MCP with TOON optimization

**Example usage**:
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::toon::ToonResponse;

#[derive(Serialize, Toon)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Returns TOON or JSON based on Accept header
#[get("/user/{id}")]
async fn get_user(Path(id): Path<u64>) -> ToonResponse<User> {
    ToonResponse::new(User {
        id,
        name: "Alice".into(),
        email: "alice@example.com".into(),
    })
}
```

**Content Negotiation**:
```bash
# Get JSON (default)
curl http://localhost:8080/user/1

# Get TOON format
curl -H "Accept: application/toon" http://localhost:8080/user/1
```

**Cookbook**: [TOON Format](https://tuntii.github.io/RustAPI/)

---

### `ws`

**WebSocket Support** — Real-time bidirectional communication.

```toml
rustapi-rs = { version = "0.2", features = ["ws"] }
```

**Provides**:
- `WebSocket` — WebSocket upgrade handler
- `WsConnection` — Connection management
- `WsSender` / `WsReceiver` — Split stream handling
- Broadcast channel integration
- Ping/pong heartbeat support

**Used in examples**:
- [websocket](websocket/) — Chat server with broadcast

**Example usage**:
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::ws::{WebSocket, WsConnection};

#[get("/ws")]
async fn websocket_handler(ws: WebSocket) -> WsConnection {
    ws.on_upgrade(|mut conn| async move {
        while let Some(msg) = conn.recv().await {
            if let Ok(text) = msg.to_text() {
                conn.send(format!("Echo: {}", text)).await.ok();
            }
        }
    })
}
```

**Cookbook**: [WebSockets](https://tuntii.github.io/RustAPI/)

---

### `view`

**Template Rendering** — Server-side HTML with Tera templates.

```toml
rustapi-rs = { version = "0.2", features = ["view"] }
```

**Provides**:
- `View<T>` — Template response type
- `ViewEngine` — Tera template engine wrapper
- Template inheritance support
- Static file serving integration

**Used in examples**:
- [templates](templates/) — Server-side rendering

**Example usage**:
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::view::{View, ViewEngine};

#[derive(Serialize)]
struct PageContext {
    title: String,
    items: Vec<String>,
}

#[get("/")]
async fn index(engine: State<ViewEngine>) -> View<PageContext> {
    View::new("index.html", PageContext {
        title: "Home".into(),
        items: vec!["Item 1".into(), "Item 2".into()],
    })
}
```

**Cookbook**: [Template Rendering](https://tuntii.github.io/RustAPI/)

---

### `swagger-ui`

**Swagger UI** — Interactive API documentation at `/docs`.

```toml
rustapi-rs = { version = "0.2", features = ["swagger-ui"] }
```

**Provides**:
- Swagger UI served at `/docs`
- OpenAPI spec at `/openapi.json`
- Automatic route documentation
- Try-it-out functionality

**Used in examples**:
- [toon-api](toon-api/) — API with Swagger documentation

**Note**: Basic OpenAPI generation works without this feature. This flag adds the interactive Swagger UI interface.

---

## Additional Crates

Some examples use additional RustAPI ecosystem crates:

### `rustapi-core`

Low-level core functionality for advanced use cases.

```toml
rustapi-core = { version = "0.2" }
```

**Used in**: middleware-chain, phase11-demo

### `rustapi-macros`

Procedural macros (usually re-exported from `rustapi-rs`).

```toml
rustapi-macros = { version = "0.2" }
```

**Used in**: cors-test, phase11-demo

### `rustapi-extras`

Additional middleware and utilities.

```toml
rustapi-extras = { version = "0.2", features = ["sqlx", "timeout", "guard"] }
```

**Features**:
- `sqlx` — Database integration helpers
- `timeout` — Request timeout middleware
- `guard` — Request guard macros
- `logging` — Structured logging middleware
- `circuit-breaker` — Circuit breaker pattern
- `jwt` — JWT utilities

**Used in**: sqlx-crud, phase11-demo

---

## Feature Combinations

Common feature combinations for different use cases:

### REST API with Auth
```toml
rustapi-rs = { version = "0.2", features = ["jwt", "cors", "rate-limit"] }
```

### AI/LLM Backend
```toml
rustapi-rs = { version = "0.2", features = ["toon", "cors"] }
```

### Full-Stack Web App
```toml
rustapi-rs = { version = "0.2", features = ["view", "jwt", "cors"] }
```

### Real-time Application
```toml
rustapi-rs = { version = "0.2", features = ["ws", "cors"] }
```

### Production API
```toml
rustapi-rs = { version = "0.2", features = ["jwt", "cors", "rate-limit", "swagger-ui"] }
rustapi-extras = { version = "0.2", features = ["timeout", "logging", "circuit-breaker"] }
```

---

## Examples by Feature

| Feature | Examples |
|---------|----------|
| `full` | crud-api, phase11-demo |
| `jwt` | auth-api, middleware-chain, phase11-demo, proof-of-concept |
| `cors` | cors-test, middleware-chain, proof-of-concept |
| `rate-limit` | rate-limit-demo, auth-api, cors-test, proof-of-concept |
| `toon` | toon-api, mcp-server |
| `ws` | websocket |
| `view` | templates |
| `swagger-ui` | toon-api |

---

## Related Documentation

- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/) — Complete framework documentation
- [LEARNING_PATH.md](LEARNING_PATH.md) — Structured learning progression
- [Main README](README.md) — Examples overview

---

<div align="center">

**[← Back to Examples](README.md)**

</div>
