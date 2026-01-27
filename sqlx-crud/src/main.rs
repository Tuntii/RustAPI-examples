//! SQLx CRUD Example for RustAPI
//!
//! This example demonstrates database integration patterns with RustAPI:
//! - Connection pool setup with `State<DbPool>`
//! - CRUD operations with SQLx
//! - Transaction handling
//! - Error conversion from SQLx to ApiError using the extension trait
//!
//! ## Running the Example
//!
//! ```bash
//! cargo run -p sqlx-crud
//! ```
//!
//! ## API Endpoints
//!
//! - `GET /users` - List all users
//! - `GET /users/{id}` - Get user by ID
//! - `POST /users` - Create a new user
//! - `PUT /users/{id}` - Update a user
//! - `DELETE /users/{id}` - Delete a user
//! - `POST /users/batch` - Create multiple users in a transaction

use rustapi_extras::SqlxErrorExt;
use rustapi_rs::prelude::*;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::sync::Arc;

// ============================================
// Database Types
// ============================================

/// Type alias for the database pool
pub type DbPool = Pool<Sqlite>;

// ============================================
// Data Models
// ============================================

/// User model from database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Schema)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

/// Request body for creating a user
#[derive(Debug, Deserialize, Schema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// Request body for updating a user
#[derive(Debug, Deserialize, Schema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// Request body for batch user creation
#[derive(Debug, Deserialize, Schema)]
pub struct BatchCreateRequest {
    pub users: Vec<CreateUserRequest>,
}

/// Response for batch operations
#[derive(Debug, Serialize, Schema)]
pub struct BatchResponse {
    pub created: usize,
    pub ids: Vec<i64>,
}

/// Response for list of users
#[derive(Debug, Serialize, Schema)]
pub struct UsersResponse {
    pub users: Vec<User>,
}

// ============================================
// Database Setup
// ============================================

/// Initialize the database with schema
async fn init_db(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================
// Handlers
// ============================================

/// List all users
#[rustapi_rs::get("/users")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("List Users")]
async fn list_users(State(pool): State<Arc<DbPool>>) -> Result<Json<UsersResponse>> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    Ok(Json(UsersResponse { users }))
}

/// Get a user by ID
#[rustapi_rs::get("/users/{id}")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Get User")]
async fn get_user(State(pool): State<Arc<DbPool>>, Path(id): Path<i64>) -> Result<Json<User>> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    Ok(Json(user))
}

/// Create a new user
#[rustapi_rs::post("/users")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Create User")]
async fn create_user(
    State(pool): State<Arc<DbPool>>,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<User>> {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&body.name)
        .bind(&body.email)
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    let user = User {
        id: result.last_insert_rowid(),
        name: body.name,
        email: body.email,
    };

    Ok(Json(user))
}

/// Update a user
#[rustapi_rs::put("/users/{id}")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Update User")]
async fn update_user(
    State(pool): State<Arc<DbPool>>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<User>> {
    // First check if user exists
    let existing = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    // Build update query dynamically
    let name = body.name.unwrap_or(existing.name);
    let email = body.email.unwrap_or(existing.email);

    sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
        .bind(&name)
        .bind(&email)
        .bind(id)
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    Ok(Json(User { id, name, email }))
}

/// Delete a user
#[rustapi_rs::delete("/users/{id}")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Delete User")]
async fn delete_user(State(pool): State<Arc<DbPool>>, Path(id): Path<i64>) -> Result<NoContent> {
    // Check if user exists first
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.into_api_error())?;

    Ok(NoContent)
}

/// Create multiple users in a transaction
///
/// This demonstrates transaction handling - if any insert fails,
/// all inserts are rolled back.
#[rustapi_rs::post("/users/batch")]
#[rustapi_rs::tag("Users")]
#[rustapi_rs::summary("Batch Create Users")]
async fn batch_create_users(
    State(pool): State<Arc<DbPool>>,
    Json(body): Json<BatchCreateRequest>,
) -> Result<Json<BatchResponse>> {
    // Start a transaction
    let mut tx = pool.begin().await.map_err(|e| {
        ApiError::internal("Failed to start transaction").with_internal(e.to_string())
    })?;

    let mut ids = Vec::with_capacity(body.users.len());

    for user in &body.users {
        let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(&user.name)
            .bind(&user.email)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.into_api_error())?;

        ids.push(result.last_insert_rowid());
    }

    // Commit the transaction
    tx.commit().await.map_err(|e| {
        ApiError::internal("Failed to commit transaction").with_internal(e.to_string())
    })?;

    Ok(Json(BatchResponse {
        created: ids.len(),
        ids,
    }))
}

// ============================================
// Main Entry Point
// ============================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 RustAPI SQLx CRUD Example");
    println!();

    // Create an in-memory SQLite database
    // In production, use a connection string like "sqlite:./data.db"
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    // Initialize the database schema
    init_db(&pool).await?;

    // Wrap pool in Arc for sharing across handlers
    let pool = Arc::new(pool);

    println!("📦 Database initialized (in-memory SQLite)");
    println!();
    println!("Routes:");
    println!("  GET    /users       - List all users");
    println!("  GET    /users/:id   - Get user by ID");
    println!("  POST   /users       - Create a new user");
    println!("  PUT    /users/:id   - Update a user");
    println!("  DELETE /users/:id   - Delete a user");
    println!("  POST   /users/batch - Create multiple users (transaction)");
    println!();
    println!("Example requests:");
    println!("  curl -X POST http://127.0.0.1:8080/users \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"name\": \"Alice\", \"email\": \"alice@example.com\"}}'");
    println!();
    println!("  curl http://127.0.0.1:8080/users");
    println!();

    // Phase 6 / zero-config: routes + schemas are auto-registered via macros.
    // Swagger UI is enabled at /docs by default (when built with swagger-ui feature).
    RustApi::auto().state(pool).run("127.0.0.1:8080").await
}
