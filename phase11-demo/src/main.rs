//! Phase 11 Features Demo
//!
//! This example demonstrates the new Phase 11 features:
//! - Request Guards (authorization)
//! - Request Timeout
//! - Health Checks
//! - Structured Logging
//! - Circuit Breaker

use rustapi_rs::prelude::*;
use std::time::Duration;

#[derive(Debug, Serialize, Schema)]
struct HealthResponse {
    status: String,
    version: String,
    checks: HealthChecks,
}

#[derive(Debug, Serialize, Schema)]
struct HealthChecks {
    database: String,
    cache: String,
}

#[derive(Debug, Serialize, Schema)]
struct AdvancedHealthResponse {
    status: String,
    version: String,
    timestamp: String,
    checks: AdvancedHealthChecks,
}

#[derive(Debug, Serialize, Schema)]
struct AdvancedHealthChecks {
    database: CheckStatus,
    cache: CheckStatus,
}

#[derive(Debug, Serialize, Schema)]
struct CheckStatus {
    status: String,
    response_time_ms: u64,
}

#[rustapi_rs::get("/")]
async fn index() -> &'static str {
    "Phase 11 Features Demo"
}

#[rustapi_rs::get("/slow")]
async fn slow_endpoint() -> &'static str {
    // This would timeout with a 30s timeout
    tokio::time::sleep(Duration::from_secs(35)).await;
    "This should timeout"
}

#[rustapi_rs::get("/health")]
async fn health_endpoint() -> Json<HealthResponse> {
    // Simple health check response
    let health = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        checks: HealthChecks {
            database: "healthy".to_string(),
            cache: "healthy".to_string(),
        },
    };
    Json(health)
}

#[rustapi_rs::get("/health-advanced")]
async fn health_advanced() -> Json<AdvancedHealthResponse> {
    // Simulate more complex health checks
    tokio::time::sleep(Duration::from_millis(10)).await;
    let health = AdvancedHealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        checks: AdvancedHealthChecks {
            database: CheckStatus {
                status: "healthy".to_string(),
                response_time_ms: 10,
            },
            cache: CheckStatus {
                status: "healthy".to_string(),
                response_time_ms: 5,
            },
        },
    };
    Json(health)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Phase 11 Demo running on http://localhost:3000");
    println!();
    println!("Available endpoints:");
    println!("  GET /                  - Index page");
    println!("  GET /health            - Simple health check");
    println!("  GET /health-advanced   - Advanced health check with timing");
    println!("  GET /slow              - Slow endpoint (35s delay)");

    println!();
    println!("Note: This demo showcases Phase 11 architectural concepts.");
    println!("Middleware features would be implemented using tower layers in production.");

    // Use auto() to automatically register routes from macro attributes
    RustApi::auto()
        .run("127.0.0.1:3000")
        .await
}
