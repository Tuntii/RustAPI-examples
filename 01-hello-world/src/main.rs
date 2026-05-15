// Run with: cargo run -p hello-world
// Then visit: http://127.0.0.1:3000/hello
//             http://127.0.0.1:3000/docs  (Swagger UI)
//
// Lesson: basic routing, JSON responses, and auto-discovered OpenAPI docs.

use rustapi_rs::prelude::*;

/// A greeting returned by the API.
#[derive(Serialize, Schema)]
struct Greeting {
    message: String,
    framework: &'static str,
}

/// Plain text root endpoint.
#[summary("Root")]
async fn root() -> &'static str {
    "Welcome to RustAPI!"
}

/// JSON greeting endpoint.
#[tag("hello")]
#[summary("Say hello")]
#[description("Returns a JSON greeting with a custom name.")]
async fn hello(Path(name): Path<String>) -> Json<Greeting> {
    Json(Greeting {
        message: format!("Hello, {}!", name),
        framework: "RustAPI",
    })
}

/// Handler that always returns HTTP 204 No Content.
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

    // RustApi::auto() discovers all handlers defined with #[auto_route]
    // or registered manually via .route().
    RustApi::new()
        .swagger_ui("/docs")
        .route("/", get(root))
        .route("/hello/{name}", get(hello))
        .route("/health", get(health))
        .run("127.0.0.1:3000")
        .await
}
