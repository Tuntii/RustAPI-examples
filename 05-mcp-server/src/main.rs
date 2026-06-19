// Run with: cargo run -p mcp-server
// HTTP: http://127.0.0.1:8080
// MCP:  http://127.0.0.1:9090   (connect your AI agent here)
//
// This example demonstrates:
// - Exposing routes as MCP tools using tags
// - In-process MCP invocation for zero network overhead
// - Full pipeline (validation, middleware) still applies
//
// For any OpenAPI (not just RustAPI): use `cargo rustapi mcp generate`

use rustapi_rs::prelude::*;
use rustapi_rs::protocol::mcp::{InvocationMode, McpConfig, McpServer, ToolPolicy, run_rustapi_and_mcp_with_shutdown};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Schema)]
struct Weather {
    city: String,
    temperature: i32,
    unit: &'static str,
}

#[derive(Deserialize, Serialize, Schema)]
struct SumRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize, Schema)]
struct SumResponse {
    result: i32,
}

/// Exposed as MCP tool (has "agent" tag) → automatically treated as read
#[rustapi_rs::get("/weather/{city}")]
#[rustapi_rs::tag("agent")]
#[rustapi_rs::summary("Get current weather for a city")]
async fn get_weather(Path(city): Path<String>) -> Json<Weather> {
    Json(Weather {
        city,
        temperature: 22,
        unit: "C",
    })
}

/// Write operation — only exposed because we set ToolPolicy::All below.
/// Marked to require confirmation from the agent.
#[rustapi_rs::post("/calc/sum")]
#[rustapi_rs::tag("agent")]
#[rustapi_rs::mcp(write, require = "confirm")]
#[rustapi_rs::summary("Add two numbers")]
async fn sum_numbers(Json(req): Json<SumRequest>) -> Json<SumResponse> {
    Json(SumResponse {
        result: req.a + req.b,
    })
}

/// Explicitly skipped via the mcp attribute (never becomes a tool)
#[rustapi_rs::get("/admin/secret")]
#[rustapi_rs::mcp(skip)]
async fn admin_secret() -> &'static str {
    "This should never be visible to agents"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = RustApi::auto();

    // Use in-process mode for best performance with agents.
    // Explicitly allow writes here (for the demo). In real agent setups prefer ReadOnly.
    let mcp = McpServer::from_rustapi(
        &app,
        McpConfig::new()
            .name("rustapi-mcp-demo")
            .version("0.1.0")
            .description("RustAPI MCP example - expose endpoints as AI tools")
            .allowed_tags(["agent"])
            .tool_policy(ToolPolicy::All)
            .invocation_mode(InvocationMode::InProcess),
    );

    println!("🚀 HTTP API:  http://127.0.0.1:8080");
    println!("🧠 MCP tools: http://127.0.0.1:9090");
    println!();
    println!("Connect Claude, Cursor or any MCP client to port 9090.");
    println!("Test manually:");
    println!("  curl -X POST http://127.0.0.1:9090 -d '{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\"}}'");

    run_rustapi_and_mcp_with_shutdown(
        app,
        "0.0.0.0:8080",
        mcp,
        "0.0.0.0:9090",
        async { let _ = tokio::signal::ctrl_c().await; },
    )
    .await?;

    Ok(())
}
