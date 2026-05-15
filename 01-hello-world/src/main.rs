// Run with: cargo run -p hello-world
// Then visit: http://127.0.0.1:3000/docs
//
// Lesson: #[get] attribute macros auto-register routes — no .route() calls needed.
//         RustApi::auto() discovers them all and serves /docs automatically.

use rustapi_rs::prelude::*;
use rustapi_rs::{description, get, summary, tag};

#[derive(Serialize, Schema)]
struct Greeting {
    message: String,
    framework: &'static str,
}

#[get("/")]
#[summary("Root")]
async fn root() -> &'static str {
    "Welcome to RustAPI!"
}

#[get("/hello/{name}")]
#[tag("hello")]
#[summary("Say hello")]
#[description("Returns a JSON greeting with a custom name.")]
async fn hello(Path(name): Path<String>) -> Json<Greeting> {
    Json(Greeting {
        message: format!("Hello, {}!", name),
        framework: "RustAPI",
    })
}

#[get("/health")]
#[summary("Health check")]
async fn health() -> NoContent {
    NoContent
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting hello-world example…");
    println!(" -> GET  http://127.0.0.1:3000/");
    println!(" -> GET  http://127.0.0.1:3000/hello/{{name}}");
    println!(" -> GET  http://127.0.0.1:3000/health");
    println!(" -> GET  http://127.0.0.1:3000/docs");
    println!(" -> GET  http://127.0.0.1:3000/__rustapi/dashboard");

    // Zero configuration — #[get] macros register every route at compile time.
    // Swagger UI lands at /docs automatically.
    // Dashboard lands at /__rustapi/dashboard.
    RustApi::auto()
        .dashboard(DashboardConfig::new())
        .run("127.0.0.1:3000")
        .await
}
