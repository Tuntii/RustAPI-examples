// Run with: cargo run -p sse-stream
// Then open: http://127.0.0.1:3000/events  (SSE stream)
//            http://127.0.0.1:3000/        (HTML test page)
//
// Lesson: Server-Sent Events (SSE) — push real-time updates from the server
//         to the client over a plain HTTP connection, with keep-alive pings.

use rustapi_rs::prelude::*;
use std::convert::Infallible;
use tokio::time::{Duration, interval};
use tokio_stream::wrappers::IntervalStream;
use futures_util::StreamExt;

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

/// Streams a numbered tick every second for 10 ticks, then closes.
#[summary("Event stream")]
#[description("Sends a `tick` SSE event once per second for 10 seconds.")]
async fn event_stream(
) -> Sse<impl futures_util::Stream<Item = std::result::Result<SseEvent, Infallible>>> {
    let mut counter = 0_u64;
    let stream = IntervalStream::new(interval(Duration::from_secs(1)))
        .take(10)
        .map(move |_| {
            counter += 1;
            let tick = Tick {
                count: counter,
                message: format!("tick {counter} of 10"),
            };
            Ok::<_, Infallible>(
                SseEvent::json_data(&tick)
                    .expect("tick should serialize")
                    .event("tick")
                    .id(counter.to_string()),
            )
        });

    sse_from_stream(stream).keep_alive(KeepAlive::new())
}

/// Tiny HTML page that consumes the SSE stream in the browser.
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

    RustApi::new()
        .route("/", get(index))
        .route("/events", get(event_stream))
        .run("127.0.0.1:3000")
        .await
}
