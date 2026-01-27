# Hello World Example

The minimal RustAPI application — your first step into the framework.

> 📖 **Cookbook**: [Getting Started → Quickstart](https://tuntii.github.io/RustAPI/)

## Overview

This example demonstrates the absolute minimum needed to create a working API with RustAPI. In just ~20 lines of code, you get:

- Automatic route discovery with `RustApi::auto()`
- Path parameter extraction
- JSON response serialization
- OpenAPI documentation at `/docs`

## Prerequisites

- Rust 1.70+
- Basic Rust knowledge

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `RustApi::auto()` | Zero-config route discovery |
| `#[rustapi_rs::get]` | Route macro for GET endpoints |
| `Path<T>` | Path parameter extractor |
| `Json<T>` | JSON response type |
| `Schema` derive | OpenAPI schema generation |

## Quick Start

```bash
# Run the example
cargo run -p hello-world

# Server starts at http://127.0.0.1:8080
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/hello/{name}` | Returns greeting with name |

## Testing

```bash
# Basic greeting
curl http://127.0.0.1:8080/hello/World
# Response: {"greeting":"Hello, World!"}

# With different name
curl http://127.0.0.1:8080/hello/RustAPI
# Response: {"greeting":"Hello, RustAPI!"}

# View API documentation
open http://127.0.0.1:8080/docs
```

## Code Walkthrough

### 1. Response Model

```rust
#[derive(Serialize, utoipa::ToSchema)]
struct Message {
    greeting: String,
}
```

The `Serialize` derive enables JSON serialization, and `ToSchema` generates OpenAPI schema.

### 2. Route Handler

```rust
#[rustapi_rs::get("/hello/{name}")]
async fn hello(Path(name): Path<String>) -> Json<Message> {
    Json(Message {
        greeting: format!("Hello, {name}!"),
    })
}
```

- `#[rustapi_rs::get("/hello/{name}")]` — Defines a GET endpoint with path parameter
- `Path(name): Path<String>` — Extracts the `{name}` parameter from the URL
- `Json<Message>` — Returns JSON response

### 3. Server Startup

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    RustApi::auto().run("0.0.0.0:8080").await
}
```

`RustApi::auto()` automatically discovers all routes marked with RustAPI macros — no manual registration needed!

## Key Concepts

### Auto-Discovery

Unlike other frameworks that require manual route registration:

```rust
// Other frameworks:
app.route("/hello/:name", get(hello))
   .route("/users", get(list_users))
   .route("/users/:id", get(get_user));

// RustAPI:
RustApi::auto()  // That's it!
```

### Path Parameters

Path parameters are defined with `{param}` syntax and extracted with `Path<T>`:

```rust
#[rustapi_rs::get("/users/{id}/posts/{post_id}")]
async fn get_post(
    Path((id, post_id)): Path<(u64, u64)>
) -> String {
    format!("User {} Post {}", id, post_id)
}
```

## Next Steps

Once comfortable with this example, move to:

1. **[crud-api](../crud-api/)** — Learn validation, state management, and error handling
2. **[auth-api](../auth-api/)** — Add JWT authentication

## Related Documentation

- [LEARNING_PATH.md](../LEARNING_PATH.md) — Structured learning progression
- [FEATURES.md](../FEATURES.md) — Feature flags reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/) — Full documentation

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
