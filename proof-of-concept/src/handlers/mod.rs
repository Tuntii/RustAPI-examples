//! Request handlers for the Bookmark Manager POC

pub mod auth;
pub mod bookmarks;
pub mod categories;
pub mod events;

use rustapi_rs::prelude::*;

use crate::models::HealthResponse;

/// Frontend (very small landing page)
#[rustapi_rs::get("/")]
#[rustapi_rs::tag("System")]
#[rustapi_rs::summary("Frontend")]
async fn index() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang=\"en\">
    <head>
        <meta charset=\"utf-8\" />
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
        <title>RustAPI Proof of Concept</title>
    </head>
    <body>
        <h1>RustAPI Proof of Concept</h1>
        <p>Swagger UI: <a href=\"/docs\">/docs</a></p>
        <p>Health: <a href=\"/health\">/health</a></p>
    </body>
</html>"#,
    )
}

/// Health check endpoint
#[rustapi_rs::get("/health")]
#[rustapi_rs::tag("System")]
#[rustapi_rs::summary("Health Check")]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
