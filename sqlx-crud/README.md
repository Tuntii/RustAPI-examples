# SQLx CRUD Example

Database integration with SQLx demonstrating production-ready patterns for RustAPI applications.

> 📖 **Cookbook**: [Recipes → Database Integration](https://tuntii.github.io/RustAPI/)

## Overview

This example shows how to integrate SQLx with RustAPI for:

- SQLite database operations (easily adaptable to PostgreSQL/MySQL)
- Connection pooling for performance
- Transaction handling for data integrity
- Error conversion with extension traits
- Full CRUD operations

## Prerequisites

- Rust 1.70+
- SQLite installed (or Docker for PostgreSQL)
- Completed [crud-api](../crud-api/) example

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `State<Arc<DbPool>>` | Database pool sharing |
| `SqlxErrorExt` | Error conversion trait |
| `sqlx::query_as!` | Type-safe queries |
| Transactions | Atomic batch operations |
| Connection pooling | Efficient connection reuse |

## Quick Start

```bash
# Run with SQLite (default)
cargo run -p sqlx-crud

# Server starts at http://127.0.0.1:8080
# SQLite database created at ./data.db
```

### With PostgreSQL

```bash
# Start PostgreSQL
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres

# Set connection string
export DATABASE_URL="postgres://postgres:postgres@localhost/rustapi"

# Run with PostgreSQL
cargo run -p sqlx-crud
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/users` | List all users |
| GET | `/users/{id}` | Get user by ID |
| POST | `/users` | Create a new user |
| PUT | `/users/{id}` | Update a user |
| DELETE | `/users/{id}` | Delete a user |
| POST | `/users/batch` | Create multiple users (transaction) |

## Testing

### List Users

```bash
curl http://127.0.0.1:8080/users
```

### Create User

```bash
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'
```

### Get User

```bash
curl http://127.0.0.1:8080/users/1
```

### Update User

```bash
curl -X PUT http://127.0.0.1:8080/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Updated", "email": "alice.new@example.com"}'
```

### Delete User

```bash
curl -X DELETE http://127.0.0.1:8080/users/1
```

### Batch Create (Transaction)

```bash
curl -X POST http://127.0.0.1:8080/users/batch \
  -H "Content-Type: application/json" \
  -d '{
    "users": [
      {"name": "Bob", "email": "bob@example.com"},
      {"name": "Carol", "email": "carol@example.com"}
    ]
  }'
```

## Code Walkthrough

### 1. Database Pool Setup

```rust
use sqlx::{Pool, Sqlite, SqlitePool};

pub type DbPool = Pool<Sqlite>;

async fn main() {
    // Create pool
    let pool = SqlitePool::connect("sqlite:./data.db").await?;
    
    // Initialize schema
    init_db(&pool).await?;
    
    // Share pool via state
    let app = RustApi::auto()
        .state(Arc::new(pool));
}
```

### 2. Schema Initialization

```rust
async fn init_db(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )
    "#)
    .execute(pool)
    .await?;
    Ok(())
}
```

### 3. Query Handlers

```rust
#[rustapi_rs::get("/users")]
async fn list_users(State(pool): State<Arc<DbPool>>) -> Result<Json<UsersResponse>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users"
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| e.into_api_error())?;
    
    Ok(Json(UsersResponse { users }))
}
```

### 4. Error Conversion

Using `rustapi_extras::SqlxErrorExt`:

```rust
use rustapi_extras::SqlxErrorExt;

// Automatically converts SQLx errors to ApiError
let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.into_api_error())?;

// SQLx NotFound → ApiError::not_found()
// SQLx UniqueViolation → ApiError::conflict()
// Other errors → ApiError::internal()
```

### 5. Transaction Handling

```rust
#[rustapi_rs::post("/users/batch")]
async fn batch_create(
    State(pool): State<Arc<DbPool>>,
    Json(body): Json<BatchCreateRequest>,
) -> Result<Json<BatchResponse>> {
    // Start transaction
    let mut tx = pool.begin().await.map_err(|e| e.into_api_error())?;
    
    let mut ids = Vec::new();
    
    for user in body.users {
        let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(&user.name)
            .bind(&user.email)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.into_api_error())?;
        
        ids.push(result.last_insert_rowid());
    }
    
    // Commit transaction
    tx.commit().await.map_err(|e| e.into_api_error())?;
    
    Ok(Json(BatchResponse {
        created: ids.len(),
        ids,
    }))
}
```

## Key Concepts

### Connection Pooling

```rust
// Pool handles connection reuse automatically
let pool = SqlitePool::connect_with(
    SqliteConnectOptions::new()
        .filename("./data.db")
        .create_if_missing(true)
)
.await?;

// Configure pool size
let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .connect("sqlite:./data.db")
    .await?;
```

### Type-Safe Queries

```rust
// Using sqlx::query_as with derive
#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
```

### Parameterized Queries

```rust
// Safe from SQL injection
sqlx::query("SELECT * FROM users WHERE id = ? AND name = ?")
    .bind(id)
    .bind(name)
    .fetch_one(&pool)
    .await?;
```

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.1" }
rustapi-extras = { version = "0.1", features = ["sqlx"] }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
serde = { version = "1", features = ["derive"] }
utoipa = "4"
```

### For PostgreSQL

```toml
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
```

## Database Migrations

For production, use SQLx migrations:

```bash
# Install SQLx CLI
cargo install sqlx-cli

# Create migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run
```

## Next Steps

- **[event-sourcing](../event-sourcing/)** — Advanced data patterns
- **[proof-of-concept](../proof-of-concept/)** — Full application

## Related Documentation

- [FEATURES.md](../FEATURES.md) — Feature reference
- [SQLx Documentation](https://docs.rs/sqlx)
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
