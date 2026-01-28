# Proof of Concept — Bookmark Manager

A comprehensive example demonstrating all RustAPI features in a real-world application.

> 📖 **This example combines concepts from**: [auth-api](../auth-api/), [crud-api](../crud-api/), [websocket](../websocket/)

## Overview

The Bookmark Manager is a full-featured application showcasing:

- JWT authentication (login/register)
- CRUD operations with validation
- Category management
- Server-Sent Events (SSE) for real-time updates
- Modular code organization
- Rate limiting and CORS
- Swagger UI documentation

## Prerequisites

- Rust 1.70+
- Completed the [Learning Path](../LEARNING_PATH.md) through auth-api
- Understanding of REST API design

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `JwtLayer<Claims>` | Authentication middleware |
| `AuthUser<T>` | User context in handlers |
| `#[validate]` | Input validation |
| `SSE` | Server-Sent Events |
| Modular handlers | Code organization |
| `State<Arc<T>>` | Shared application state |

## Project Structure

```
proof-of-concept/
├── src/
│   ├── main.rs           # App setup, middleware
│   ├── models.rs         # Data structures, Claims
│   ├── stores.rs         # In-memory storage
│   ├── sse.rs            # SSE event handling
│   └── handlers/
│       ├── mod.rs        # Handler exports
│       ├── auth.rs       # Login, register
│       ├── bookmarks.rs  # Bookmark CRUD
│       ├── categories.rs # Category CRUD
│       └── events.rs     # SSE endpoints
└── Cargo.toml
```

## Quick Start

```bash
# Run the application
cargo run -p proof-of-concept

# Server starts at http://127.0.0.1:8080
```

## API Endpoints

### Public Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Frontend (if static files exist) |
| GET | `/health` | Health check |
| POST | `/auth/register` | Register new user |
| POST | `/auth/login` | Login and get JWT |
| GET | `/docs` | Swagger UI |

### Protected Endpoints (Require JWT)

#### Bookmarks

| Method | Path | Description |
|--------|------|-------------|
| GET | `/bookmarks` | List user's bookmarks |
| POST | `/bookmarks` | Create bookmark |
| GET | `/bookmarks/{id}` | Get bookmark |
| PUT | `/bookmarks/{id}` | Update bookmark |
| DELETE | `/bookmarks/{id}` | Delete bookmark |
| GET | `/bookmarks/export` | Export all bookmarks |
| POST | `/bookmarks/import` | Import bookmarks |

#### Categories

| Method | Path | Description |
|--------|------|-------------|
| GET | `/categories` | List categories |
| POST | `/categories` | Create category |
| PUT | `/categories/{id}` | Update category |
| DELETE | `/categories/{id}` | Delete category |

#### Events

| Method | Path | Description |
|--------|------|-------------|
| GET | `/events` | SSE event stream |

## Testing Workflow

### 1. Register a User

```bash
curl -X POST http://127.0.0.1:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "password123"
  }'
```

### 2. Login

```bash
TOKEN=$(curl -s -X POST http://127.0.0.1:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "password123"}' \
  | jq -r '.token')

echo "Token: $TOKEN"
```

### 3. Create a Category

```bash
curl -X POST http://127.0.0.1:8080/categories \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "Development", "color": "#3498db"}'
```

### 4. Create a Bookmark

```bash
curl -X POST http://127.0.0.1:8080/bookmarks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com/Tuntii/RustAPI",
    "title": "RustAPI Framework",
    "description": "FastAPI-like framework for Rust",
    "category_id": 1,
    "tags": ["rust", "web", "api"]
  }'
```

### 5. List Bookmarks

```bash
curl http://127.0.0.1:8080/bookmarks \
  -H "Authorization: Bearer $TOKEN"
```

### 6. Subscribe to Events (SSE)

```bash
# In a separate terminal
curl -N http://127.0.0.1:8080/events \
  -H "Authorization: Bearer $TOKEN"

# Events will stream as you create/update/delete bookmarks
```

### 7. Export Bookmarks

```bash
curl http://127.0.0.1:8080/bookmarks/export \
  -H "Authorization: Bearer $TOKEN" \
  -o bookmarks.json
```

## Code Highlights

### Modular Handler Organization

```rust
// handlers/mod.rs
pub mod auth;
pub mod bookmarks;
pub mod categories;
pub mod events;

pub use auth::*;
pub use bookmarks::*;
pub use categories::*;
pub use events::*;
```

### JWT Claims

```rust
// models.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub username: String,
    pub exp: u64,
}
```

### Application State

```rust
// stores.rs
pub struct AppState {
    pub users: RwLock<HashMap<String, User>>,
    pub bookmarks: RwLock<HashMap<u64, Bookmark>>,
    pub categories: RwLock<HashMap<u64, Category>>,
    pub sse_broadcaster: Broadcaster,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            bookmarks: RwLock::new(HashMap::new()),
            categories: RwLock::new(HashMap::new()),
            sse_broadcaster: Broadcaster::new(),
        }
    }
}
```

### Protected Handler

```rust
// handlers/bookmarks.rs
#[rustapi_rs::get("/bookmarks")]
#[tag("Bookmarks")]
async fn list_bookmarks(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser<Claims>,
) -> Json<Vec<Bookmark>> {
    let bookmarks = state.bookmarks.read().unwrap();
    
    let user_bookmarks: Vec<Bookmark> = bookmarks
        .values()
        .filter(|b| b.user_id == claims.sub)
        .cloned()
        .collect();
    
    Json(user_bookmarks)
}
```

### SSE Broadcasting

```rust
// handlers/events.rs
#[rustapi_rs::get("/events")]
async fn event_stream(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser<Claims>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.sse_broadcaster.subscribe();
    
    let stream = BroadcastStream::new(rx)
        .filter_map(move |msg| {
            // Filter events for this user
        });
    
    Sse::new(stream)
}

// When bookmark is created:
state.sse_broadcaster.send(SseEvent::BookmarkCreated {
    bookmark: bookmark.clone(),
});
```

### Main Setup

```rust
// main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = Arc::new(AppState::new());

    let app = RustApi::config()
        .body_limit(1024 * 1024)
        .layer(RequestIdLayer::new())
        .layer(TracingLayer::new())
        .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
        .layer(JwtLayer::<Claims>::new(JWT_SECRET).skip_paths(vec![
            "/",
            "/health",
            "/docs",
            "/auth/register",
            "/auth/login",
            "/static",
        ]))
        .build()
        .state(state);

    app.run("127.0.0.1:8080").await
}
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        Client                                │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Middleware Stack                         │
├─────────────────────────────────────────────────────────────┤
│  RequestIdLayer  →  TracingLayer  →  RateLimitLayer         │
│                           │                                  │
│                           ▼                                  │
│                      JwtLayer                                │
│              (skip: /, /health, /auth/*)                     │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                        Handlers                              │
├──────────────┬──────────────┬──────────────┬────────────────┤
│    auth.rs   │ bookmarks.rs │ categories.rs│   events.rs    │
│  - register  │  - CRUD      │  - CRUD      │  - SSE stream  │
│  - login     │  - export    │              │                │
│              │  - import    │              │                │
└──────────────┴──────────────┴──────────────┴────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     AppState (Arc)                           │
├──────────────┬──────────────┬──────────────┬────────────────┤
│    users     │  bookmarks   │  categories  │  broadcaster   │
│  (RwLock)    │  (RwLock)    │  (RwLock)    │    (SSE)       │
└──────────────┴──────────────┴──────────────┴────────────────┘
```

## Cargo.toml

```toml
[package]
name = "proof-of-concept"
version = "0.1.0"
edition = "2021"
description = "Comprehensive POC demonstrating all RustAPI features"

[dependencies]
rustapi-rs = { version = "0.1", features = ["jwt", "cors", "rate-limit"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
chrono = "0.4"
uuid = { version = "1", features = ["v4"] }
futures-util = "0.3"

[dev-dependencies]
proptest = "1.4"  # Property-based testing
```

## Next Steps

This example is the culmination of the learning path. From here, explore:

- **[microservices](../microservices/)** — Distributed architecture
- **[graphql-api](../graphql-api/)** — GraphQL integration
- **[phase11-demo](../phase11-demo/)** — Advanced middleware

## Related Documentation

- [LEARNING_PATH.md](../LEARNING_PATH.md) — Full learning progression
- [FEATURES.md](../FEATURES.md) — Feature reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
