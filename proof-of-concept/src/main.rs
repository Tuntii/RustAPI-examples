//! Proof of Concept - Bookmark Manager
//!
//! A comprehensive example demonstrating all RustAPI features:
//! - JWT authentication
//! - CRUD operations with validation
//! - Category management
//! - Search and filtering with pagination
//! - Server-Sent Events (SSE)
//! - Static file serving
//! - Swagger UI documentation
//! - Rate limiting and CORS
//!
//! Run with: cargo run -p proof-of-concept
//! Then visit: http://127.0.0.1:8080

mod handlers;
mod models;
mod sse;
mod stores;

use rustapi_rs::prelude::*;
use std::sync::Arc;
use std::time::Duration;

use crate::stores::AppState;

/// JWT secret key (in production, use environment variable)
pub const JWT_SECRET: &str = "bookmark-manager-secret-key-change-in-production";

/// Token expiration time (1 hour)
pub const TOKEN_EXPIRY_SECS: u64 = 3600;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("📚 Bookmark Manager - RustAPI Proof of Concept");
    println!();
    println!("Public Endpoints:");
    println!("  GET  /              - Frontend (index.html)");
    println!("  GET  /health        - Health check");
    println!("  POST /auth/register - Register new user");
    println!("  POST /auth/login    - Login");
    println!();
    println!("Protected Endpoints (require JWT token):");
    println!("  GET    /bookmarks          - List bookmarks");
    println!("  POST   /bookmarks          - Create bookmark");
    println!("  GET    /bookmarks/:id      - Get bookmark");
    println!("  PUT    /bookmarks/:id      - Update bookmark");
    println!("  DELETE /bookmarks/:id      - Delete bookmark");
    println!("  GET    /bookmarks/export   - Export bookmarks");
    println!("  POST   /bookmarks/import   - Import bookmarks");
    println!("  GET    /categories         - List categories");
    println!("  POST   /categories         - Create category");
    println!("  PUT    /categories/:id     - Update category");
    println!("  DELETE /categories/:id     - Delete category");
    println!("  GET    /events             - SSE event stream");
    println!();
    println!("Documentation:");
    println!("  GET /docs - Swagger UI");
    println!();
    println!("Server running at http://127.0.0.1:8080");

    // Initialize application state
    let state = Arc::new(AppState::new());

    // Phase 6 / zero-config: routes + schemas are auto-registered via macros.
    // We use `config()` so we can attach layers/body limit while still using auto routes.
    let app = RustApi::config()
        .body_limit(1024 * 1024) // 1MB limit
        .layer(RequestIdLayer::new())
        .layer(TracingLayer::new())
        .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
        .layer(JwtLayer::<models::Claims>::new(JWT_SECRET).skip_paths(vec![
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
