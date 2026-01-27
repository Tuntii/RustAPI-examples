# RustAPI Learning Path

A structured guide to learning RustAPI through progressive examples. Each step builds upon the previous, introducing new concepts while reinforcing fundamentals.

> 📖 **Cookbook**: [tuntii.github.io/RustAPI](https://tuntii.github.io/RustAPI/)  
> 🔗 **Framework**: [github.com/Tuntii/RustAPI](https://github.com/Tuntii/RustAPI)

---

## Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        RUSTAPI LEARNING PATH                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  BEGINNER                                                               │
│  ────────                                                               │
│  1. hello-world      → Basic routing, RustApi::auto(), responses        │
│          ↓                                                              │
│  2. crud-api         → CRUD, validation, state, error handling          │
│          ↓                                                              │
│  INTERMEDIATE                                                           │
│  ────────────                                                           │
│  3. auth-api         → JWT authentication, protected routes             │
│          ↓                                                              │
│  4. middleware-chain → Custom middleware, composition, ordering         │
│          ↓                                                              │
│  5. templates        → Server-side rendering, Tera templates            │
│          ↓                                                              │
│  ADVANCED                                                               │
│  ────────                                                               │
│  6. sqlx-crud        → Database integration, SQLx, transactions         │
│          ↓                                                              │
│  7. websocket        → Real-time communication, broadcast channels      │
│          ↓                                                              │
│  8. proof-of-concept → Full application combining all concepts          │
│          ↓                                                              │
│  EXPERT                                                                 │
│  ──────                                                                 │
│  9. microservices    → API Gateway, service-to-service communication    │
│          ↓                                                              │
│  10. phase11-demo    → Guards, circuit breaker, advanced middleware     │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Beginner Level

### Step 1: hello-world

**Time**: ~15 minutes  
**Prerequisites**: Basic Rust knowledge

#### What You'll Learn
- Setting up a RustAPI project
- Using `RustApi::auto()` for automatic route discovery
- Defining route handlers with `#[get]` macro
- Path parameters with `Path<T>` extractor
- Returning different response types

#### Key Concepts
```rust
// Route definition
#[get("/hello/{name}")]
async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

// Auto-discovery - no manual route registration!
RustApi::auto().run("127.0.0.1:8080").await
```

#### Run It
```bash
cargo run -p hello-world

# Test
curl http://localhost:8080/
curl http://localhost:8080/hello/World
```

#### Next Steps
Once comfortable, move to crud-api to learn about state management and validation.

---

### Step 2: crud-api

**Time**: ~45 minutes  
**Prerequisites**: Completed hello-world

#### What You'll Learn
- Full CRUD operations (Create, Read, Update, Delete)
- Request body parsing with `Json<T>`
- Input validation with `#[validate]`
- Shared state with `State<T>` and `Arc<RwLock<T>>`
- Query parameters with `Query<T>`
- Pagination patterns
- Error handling with `ApiError`
- Body size limits

#### Key Concepts
```rust
// Validated request body
#[derive(Deserialize, Validate)]
struct CreateUser {
    #[validate(length(min = 1, max = 100))]
    name: String,
    #[validate(email)]
    email: String,
}

// CRUD handler with validation
#[post("/users")]
async fn create_user(
    State(db): State<AppState>,
    #[validate] Json(payload): Json<CreateUser>,
) -> Result<Json<User>, ApiError> {
    // Implementation
}
```

#### Run It
```bash
cargo run -p crud-api

# Test CRUD operations
curl http://localhost:8080/users
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'
```

#### Checkpoint Quiz
Before moving on, ensure you can answer:
- [ ] How does `State<T>` work for sharing data?
- [ ] What happens when validation fails?
- [ ] How do you handle optional query parameters?

---

## Intermediate Level

### Step 3: auth-api

**Time**: ~1 hour  
**Prerequisites**: Completed crud-api

#### What You'll Learn
- JWT token generation and validation
- `JwtLayer` middleware configuration
- `AuthUser<T>` extractor for protected routes
- Skip paths for public endpoints
- Login/registration flows
- Token refresh patterns

#### Key Concepts
```rust
// JWT Claims structure
#[derive(Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,      // Subject (user ID)
    exp: usize,       // Expiration time
    role: String,     // User role
}

// Protected route
#[get("/profile")]
async fn profile(auth: AuthUser<Claims>) -> Json<UserProfile> {
    // auth.claims contains the validated JWT claims
    Json(get_user_profile(auth.claims.sub))
}

// JWT Layer with skip paths
JwtLayer::<Claims>::new("secret")
    .skip_paths(vec!["/login", "/register", "/docs"])
```

#### Run It
```bash
cargo run -p auth-api

# Login to get token
TOKEN=$(curl -s -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | jq -r '.token')

# Access protected route
curl -H "Authorization: Bearer $TOKEN" http://localhost:8080/profile
```

#### Security Considerations
- Never hardcode secrets in production
- Use appropriate token expiration times
- Implement token refresh for long sessions
- Consider rate limiting login endpoints

---

### Step 4: middleware-chain

**Time**: ~45 minutes  
**Prerequisites**: Completed auth-api

#### What You'll Learn
- Creating custom middleware with `Layer` trait
- Middleware execution order
- Request/response transformation
- Request ID tracking
- Request timing/metrics
- Combining multiple middleware layers

#### Key Concepts
```rust
// Custom middleware implementation
pub struct TimingLayer;

impl<S> Layer<S> for TimingLayer {
    type Service = TimingService<S>;
    
    fn layer(&self, inner: S) -> Self::Service {
        TimingService { inner }
    }
}

// Middleware ordering matters!
RustApi::auto()
    .layer(RequestIdLayer::new())   // Runs first (outermost)
    .layer(TimingLayer::new())      // Runs second
    .layer(JwtLayer::new("secret")) // Runs third (innermost)
    .run("127.0.0.1:8080")
    .await
```

#### Middleware Execution Flow
```
Request → RequestId → Timing → JWT → Handler → JWT → Timing → RequestId → Response
           (pre)       (pre)   (pre)           (post)  (post)    (post)
```

#### Run It
```bash
cargo run -p middleware-chain

# Observe headers
curl -v http://localhost:8080/
# Look for: X-Request-Id, X-Response-Time headers
```

---

### Step 5: templates

**Time**: ~30 minutes  
**Prerequisites**: Basic HTML knowledge

#### What You'll Learn
- Server-side rendering with Tera
- Template inheritance (base templates)
- Passing data to templates
- Static file serving
- `View<T>` response type

#### Key Concepts
```rust
// Template context
#[derive(Serialize)]
struct PageData {
    title: String,
    user: Option<User>,
    items: Vec<Item>,
}

// Render template
#[get("/")]
async fn index(engine: State<ViewEngine>) -> View<PageData> {
    View::new("index.html", PageData {
        title: "Home".into(),
        user: None,
        items: vec![],
    })
}
```

```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html>
<head><title>{{ title }}</title></head>
<body>{% block content %}{% endblock %}</body>
</html>

<!-- templates/index.html -->
{% extends "base.html" %}
{% block content %}
<h1>Welcome!</h1>
{% for item in items %}
  <p>{{ item.name }}</p>
{% endfor %}
{% endblock %}
```

#### Run It
```bash
cargo run -p templates

# Visit in browser
open http://localhost:8080/
```

---

## Advanced Level

### Step 6: sqlx-crud

**Time**: ~1.5 hours  
**Prerequisites**: Completed crud-api, basic SQL knowledge

#### What You'll Learn
- Database integration with SQLx
- Connection pooling
- Database migrations
- Transaction management
- Error handling for database operations
- Async query execution

#### Key Concepts
```rust
// Database pool in state
let pool = SqlitePool::connect("sqlite:./data.db").await?;

// Query with SQLx
#[get("/users/{id}")]
async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(ApiError::not_found("User not found"))?;
    
    Ok(Json(user))
}

// Transaction
let mut tx = pool.begin().await?;
sqlx::query!("INSERT INTO users ...").execute(&mut *tx).await?;
sqlx::query!("INSERT INTO audit_log ...").execute(&mut *tx).await?;
tx.commit().await?;
```

#### Run It
```bash
# SQLite (easiest)
cargo run -p sqlx-crud

# Or with PostgreSQL
docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres
DATABASE_URL=postgres://postgres:postgres@localhost/test cargo run -p sqlx-crud
```

---

### Step 7: websocket

**Time**: ~1 hour  
**Prerequisites**: Understanding of async/await

#### What You'll Learn
- WebSocket connections
- Upgrading HTTP to WebSocket
- Message handling (text/binary)
- Broadcast channels for pub/sub
- Connection lifecycle management
- Split sender/receiver pattern

#### Key Concepts
```rust
// WebSocket upgrade
#[get("/ws")]
async fn ws_handler(
    ws: WebSocket,
    State(broadcast): State<broadcast::Sender<String>>,
) -> WsConnection {
    ws.on_upgrade(move |conn| handle_connection(conn, broadcast))
}

async fn handle_connection(
    mut conn: WsConnection,
    tx: broadcast::Sender<String>,
) {
    let mut rx = tx.subscribe();
    
    loop {
        tokio::select! {
            // Receive from client
            Some(msg) = conn.recv() => {
                tx.send(msg.to_string()).ok();
            }
            // Broadcast to client
            Ok(msg) = rx.recv() => {
                conn.send(msg).await.ok();
            }
        }
    }
}
```

#### Run It
```bash
cargo run -p websocket

# Connect with websocat
websocat ws://localhost:8080/ws

# Or use browser console
# new WebSocket('ws://localhost:8080/ws')
```

---

### Step 8: proof-of-concept

**Time**: ~2 hours (study existing code)  
**Prerequisites**: All previous steps

#### What You'll Learn
- Combining all concepts in one application
- Project structure for larger applications
- Modular handler organization
- Real-world patterns and practices

#### Application Features
- **JWT Authentication** — Login, register, protected routes
- **CRUD Operations** — Bookmarks, categories
- **Server-Sent Events** — Real-time updates
- **Swagger UI** — API documentation
- **Rate Limiting** — API protection
- **CORS** — Frontend integration ready

#### Project Structure
```
proof-of-concept/
├── src/
│   ├── main.rs          # App setup, middleware
│   ├── models.rs        # Data structures
│   ├── stores.rs        # In-memory storage
│   ├── sse.rs           # Server-Sent Events
│   └── handlers/
│       ├── mod.rs       # Handler exports
│       ├── auth.rs      # Authentication
│       ├── bookmarks.rs # Bookmark CRUD
│       ├── categories.rs # Category CRUD
│       └── events.rs    # SSE endpoints
```

---

## Expert Level

### Step 9: microservices

**Time**: ~2 hours  
**Prerequisites**: Understanding of distributed systems

#### What You'll Learn
- API Gateway pattern
- Service-to-service communication
- Request routing and proxying
- Service health checks

#### Architecture
```
┌─────────────────────────────────────────────────────┐
│                    API Gateway                       │
│                  (Port 8080)                         │
├─────────────────────────────────────────────────────┤
│                        │                             │
│    ┌──────────────────┼──────────────────┐          │
│    ▼                  ▼                  ▼          │
│ ┌──────────┐   ┌──────────┐   ┌──────────┐         │
│ │  User    │   │  Order   │   │ Product  │         │
│ │ Service  │   │ Service  │   │ Service  │         │
│ │ (:8081)  │   │ (:8082)  │   │ (:8083)  │         │
│ └──────────┘   └──────────┘   └──────────┘         │
└─────────────────────────────────────────────────────┘
```

---

### Step 10: phase11-demo

**Time**: ~1.5 hours  
**Prerequisites**: All previous steps

#### What You'll Learn
- Request guards with `#[guard]`
- Circuit breaker pattern
- Request timeouts
- Advanced logging
- Feature-gated middleware

#### Key Concepts
```rust
// Request guard
#[guard(AdminGuard)]
#[get("/admin/settings")]
async fn admin_settings() -> Json<Settings> { ... }

// Circuit breaker
CircuitBreakerLayer::new()
    .failure_threshold(5)
    .recovery_timeout(Duration::from_secs(30))

// Timeout
TimeoutLayer::new(Duration::from_secs(10))
```

---

## Specialized Tracks

After completing the main path, explore specialized areas:

### AI/LLM Track
1. **toon-api** — TOON format for token optimization
2. **mcp-server** — Model Context Protocol integration

### Real-time Track
1. **websocket** — Bidirectional communication
2. **proof-of-concept** — SSE for server push

### Database Track
1. **sqlx-crud** — SQL databases
2. **event-sourcing** — Event sourcing pattern

### Enterprise Track
1. **microservices** — Service architecture
2. **microservices-advanced** — Service discovery
3. **phase11-demo** — Resilience patterns

---

## Study Tips

1. **Run the code** — Don't just read; execute and experiment
2. **Check /docs** — Every example has Swagger documentation
3. **Modify examples** — Break things to understand them
4. **Read error messages** — RustAPI provides helpful errors
5. **Use RUST_LOG** — Enable debug logging to see internals

```bash
# Enable detailed logging
RUST_LOG=debug cargo run -p <example>

# Just RustAPI logs
RUST_LOG=rustapi=debug cargo run -p <example>
```

---

## Getting Help

- 📖 [Cookbook](https://tuntii.github.io/RustAPI/) — Comprehensive documentation
- 💬 [GitHub Discussions](https://github.com/Tuntii/RustAPI/discussions) — Community support
- 🐛 [Issues](https://github.com/Tuntii/RustAPI/issues) — Bug reports

---

<div align="center">

**[← Back to Examples](README.md)** · **[Feature Reference →](FEATURES.md)**

</div>
