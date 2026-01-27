# CRUD API Example

A complete CRUD (Create, Read, Update, Delete) API demonstrating RustAPI's core features for building production-ready REST APIs.

> 📖 **Cookbook**: [Recipes → CRUD Resources](https://tuntii.github.io/RustAPI/)

## Overview

This example shows how to build a fully-featured task management API with:

- All HTTP methods (GET, POST, PUT, PATCH, DELETE)
- Input validation with detailed error messages
- Shared state management
- Query parameters and pagination
- Error handling patterns
- Middleware integration

## Prerequisites

- Rust 1.70+
- Completed [hello-world](../hello-world/) example

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `#[validate]` | Request body validation |
| `State<T>` | Shared state access |
| `Query<T>` | Query parameter extraction |
| `Json<T>` | JSON request/response |
| `ApiError` | Structured error responses |
| `RequestIdLayer` | Request tracking |
| `TracingLayer` | Request logging |
| Body limits | Request size protection |

## Quick Start

```bash
# Run the example
cargo run -p crud-api

# Server starts at http://127.0.0.1:8080
# Documentation at http://127.0.0.1:8080/docs
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/tasks` | List tasks with pagination and filtering |
| GET | `/tasks/{id}` | Get a specific task |
| POST | `/tasks` | Create a new task |
| PUT | `/tasks/{id}` | Full update of a task |
| PATCH | `/tasks/{id}` | Partial update of a task |
| DELETE | `/tasks/{id}` | Delete a task |

## Testing

### List Tasks

```bash
# Get all tasks
curl http://127.0.0.1:8080/tasks

# With pagination
curl "http://127.0.0.1:8080/tasks?page=1&limit=10"

# Filter by completion status
curl "http://127.0.0.1:8080/tasks?completed=false"
```

### Create Task

```bash
curl -X POST http://127.0.0.1:8080/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn RustAPI",
    "description": "Complete all examples"
  }'
```

### Update Task (Full)

```bash
curl -X PUT http://127.0.0.1:8080/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn RustAPI",
    "description": "Completed all examples!",
    "completed": true
  }'
```

### Update Task (Partial)

```bash
curl -X PATCH http://127.0.0.1:8080/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

### Delete Task

```bash
curl -X DELETE http://127.0.0.1:8080/tasks/1
```

## Code Walkthrough

### 1. Data Models with Validation

```rust
#[derive(Debug, Deserialize, Validate, Schema)]
pub struct CreateTask {
    #[validate(length(min = 1, max = 200, message = "Title must be 1-200 characters"))]
    pub title: String,
    
    #[validate(length(max = 1000, message = "Description must be at most 1000 characters"))]
    pub description: Option<String>,
}
```

The `#[validate]` attribute automatically validates incoming data and returns 400 Bad Request with details on failure.

### 2. Query Parameters

```rust
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListParams {
    pub completed: Option<bool>,
    #[param(minimum = 1)]
    pub page: Option<u32>,
    #[param(minimum = 1, maximum = 100)]
    pub limit: Option<u32>,
}

#[rustapi_rs::get("/tasks")]
async fn list_tasks(
    State(store): State<TaskStore>,
    Query(params): Query<ListParams>,
) -> Json<PaginatedTasks> {
    // Implementation
}
```

### 3. State Management

```rust
#[derive(Clone)]
pub struct TaskStore {
    tasks: Arc<RwLock<HashMap<u64, Task>>>,
    next_id: Arc<RwLock<u64>>,
}

// In handlers:
async fn create_task(
    State(store): State<TaskStore>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, ApiError> {
    let task = store.create(payload);
    Ok(Json(task))
}
```

### 4. Error Handling

```rust
async fn get_task(
    State(store): State<TaskStore>,
    Path(id): Path<u64>,
) -> Result<Json<Task>, ApiError> {
    store
        .get(id)
        .map(Json)
        .ok_or_else(|| ApiError::not_found(format!("Task {} not found", id)))
}
```

### 5. Middleware Configuration

```rust
RustApi::config()
    .body_limit(1024 * 1024)  // 1MB limit
    .layer(RequestIdLayer::new())
    .layer(TracingLayer::new())
    .build()
```

## Key Concepts

### Validation

RustAPI uses the `validator` crate for validation:

```rust
#[validate(length(min = 1, max = 100))]
#[validate(email)]
#[validate(range(min = 1, max = 1000))]
#[validate(url)]
#[validate(custom = "validate_username")]
```

### Pagination Pattern

```rust
#[derive(Debug, Serialize, Schema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: u32,
    pub limit: u32,
    pub has_more: bool,
}
```

### PUT vs PATCH

- **PUT** — Full replacement, all fields required
- **PATCH** — Partial update, only provided fields updated

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.2", features = ["full"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
validator = "0.16"
utoipa = "4"
```

## Next Steps

After mastering CRUD operations, continue with:

1. **[auth-api](../auth-api/)** — Add JWT authentication
2. **[sqlx-crud](../sqlx-crud/)** — Database integration

## Related Documentation

- [LEARNING_PATH.md](../LEARNING_PATH.md) — Learning progression
- [FEATURES.md](../FEATURES.md) — Feature flags reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/) — Full documentation

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
