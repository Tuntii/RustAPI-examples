// Run with: cargo run -p sse-stream
// Then open: http://127.0.0.1:3000/         (HTML test page)
//            http://127.0.0.1:3000/events   (raw SSE stream)
//
// Lesson: Server-Sent Events with #[get] auto-registration.
//         RustApi::auto() — zero .route() calls.

use rustapi_rs::prelude::*;
use rustapi_rs::{description, get, summary};
use std::convert::Infallible;

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Schema)]
struct Tick {
    count: u64,
    message: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

#[get("/events")]
#[summary("Event stream")]
#[description("Sends a `tick` SSE event once per second for 10 ticks, then closes.")]
async fn event_stream(
) -> Sse<impl futures_util::Stream<Item = std::result::Result<SseEvent, Infallible>>> {
    let events: Vec<Result<SseEvent, Infallible>> = (1..=10_u64)
        .map(|i| {
            Ok(SseEvent::json_data(&Tick {
                count: i,
                message: format!("tick {i} of 10"),
            })
            .expect("tick should serialize")
            .event("tick")
            .id(i.to_string()))
        })
        .collect();

    sse_from_iter(events).keep_alive(KeepAlive::new())
}

#[get("/")]
async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>SSE Demo</title></head>
<body>
  <h1>SSE Stream Demo</h1>
  <p>Events appear below (10 ticks, then stream closes):</p>
  <ul id="log"></ul>
  <script>
    const es = new EventSource('/events');
    es.addEventListener('tick', e => {
      const li = document.createElement('li');
      li.textContent = JSON.stringify(JSON.parse(e.data));
      document.getElementById('log').appendChild(li);
    });
    es.onerror = () => es.close();
  </script>
</body>
</html>"#,
    )
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting sse-stream example…");
    println!(" -> GET http://127.0.0.1:3000/         (HTML test page)");
    println!(" -> GET http://127.0.0.1:3000/events   (raw SSE stream)");
    println!(" -> GET http://127.0.0.1:3000/__rustapi/dashboard");

    // Zero configuration — #[get] macros register all routes at compile time.
    RustApi::auto()
        .dashboard(DashboardConfig::new())
        .run("127.0.0.1:3000")
        .await
}
