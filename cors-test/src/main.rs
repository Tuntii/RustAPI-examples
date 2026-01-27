//! # CORS Test Example
//!
//! Demonstrating Cross-Origin Resource Sharing (CORS) configuration with RustAPI.
//!
//! ## Demonstrates
//! - `CorsLayer::permissive()` for development CORS
//! - Layer composition with multiple middleware
//! - `RequestIdLayer`, `TracingLayer`, `RateLimitLayer`
//!
//! ## Run
//! ```bash
//! cargo run -p cors-test
//! ```
//!
//! ## Test
//! ```bash
//! curl -v http://127.0.0.1:3030/
//! ```
//!
//! ## Cookbook
//! <https://tuntii.github.io/RustAPI/>

use rustapi_rs::prelude::*;
use std::time::Duration;

async fn hello() -> &'static str {
    "Hello from CORS-enabled API!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Testing CorsLayer with the exact user configuration...");
    println!("✅ If this compiles, CorsLayer works!");

    RustApi::new()
        .route("/", get(hello))
        .layer(CorsLayer::permissive())
        .layer(RequestIdLayer::new())
        .layer(TracingLayer::new())
        .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
        .run("127.0.0.1:3030")
        .await
}
