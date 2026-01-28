# WebSocket Example

Real-time bidirectional communication with WebSocket support in RustAPI.

> 📖 **Cookbook**: [Recipes → WebSockets](https://tuntii.github.io/RustAPI/)

## Overview

This example demonstrates WebSocket capabilities:

- Basic echo server
- JSON message handling
- Broadcast to multiple clients (chat room)
- Connection lifecycle management
- Split sender/receiver pattern

## Prerequisites

- Rust 1.70+
- Understanding of async/await
- WebSocket client tool (websocat, browser, etc.)

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `WebSocket` | WebSocket upgrade extractor |
| `WebSocketUpgrade` | Connection upgrade response |
| `Message` | Text/Binary/Ping/Pong/Close |
| `Broadcast` | Multi-client broadcasting |
| `socket.split()` | Separate send/receive streams |

## Quick Start

```bash
# Run the example
cargo run -p websocket-example

# Server starts at http://127.0.0.1:8080
```

## WebSocket Endpoints

| Path | Description |
|------|-------------|
| `/ws/echo` | Echo server — returns what you send |
| `/ws/json` | JSON message handler |
| `/ws/chat` | Broadcast chat room |

## Testing

### Using websocat

```bash
# Install websocat
cargo install websocat

# Echo server
websocat ws://localhost:8080/ws/echo
# Type messages, see them echoed back

# JSON echo
websocat ws://localhost:8080/ws/json
# Send: {"username":"alice","content":"hello","timestamp":123}

# Chat room (open multiple terminals)
websocat ws://localhost:8080/ws/chat
```

### Using Browser Console

```javascript
// Echo server
const ws = new WebSocket('ws://localhost:8080/ws/echo');
ws.onmessage = (e) => console.log('Received:', e.data);
ws.onopen = () => ws.send('Hello WebSocket!');

// Chat room
const chat = new WebSocket('ws://localhost:8080/ws/chat');
chat.onmessage = (e) => console.log('Chat:', e.data);
chat.onopen = () => chat.send(JSON.stringify({
    username: 'browser',
    content: 'Hello from browser!',
    timestamp: Date.now()
}));
```

### Using curl (HTTP upgrade)

```bash
# Note: curl doesn't fully support WebSocket, use for debugging
curl -v -H "Upgrade: websocket" \
     -H "Connection: Upgrade" \
     -H "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==" \
     -H "Sec-WebSocket-Version: 13" \
     http://localhost:8080/ws/echo
```

## Code Walkthrough

### 1. Echo Server

```rust
async fn ws_echo(ws: WebSocket) -> WebSocketUpgrade {
    ws.on_upgrade(|mut socket| async move {
        while let Some(result) = socket.recv().await {
            match result {
                Ok(Message::Text(text)) => {
                    // Echo back with prefix
                    socket.send(Message::text(format!("Echo: {}", text))).await.ok();
                }
                Ok(Message::Binary(data)) => {
                    socket.send(Message::binary(data)).await.ok();
                }
                Ok(Message::Ping(data)) => {
                    socket.send(Message::pong(data)).await.ok();
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    tracing::error!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    })
}
```

### 2. JSON Message Handling

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: u64,
}

async fn ws_json(ws: WebSocket) -> WebSocketUpgrade {
    ws.on_upgrade(|mut socket| async move {
        while let Some(Ok(msg)) = socket.recv().await {
            if msg.is_text() {
                // Parse JSON message
                if let Ok(chat_msg) = msg.as_json::<ChatMessage>() {
                    // Create response
                    let response = ChatMessage {
                        username: "server".to_string(),
                        content: format!("Received: {}", chat_msg.content),
                        timestamp: now(),
                    };
                    
                    // Send JSON response
                    socket.send_json(&response).await.ok();
                }
            }
        }
    })
}
```

### 3. Broadcast Chat Room

```rust
struct AppState {
    chat_broadcast: Arc<Broadcast>,
}

async fn ws_chat(
    ws: WebSocket,
    State(state): State<Arc<AppState>>,
) -> WebSocketUpgrade {
    ws.on_upgrade(move |socket| async move {
        let (mut sender, mut receiver) = socket.split();
        let broadcast = state.chat_broadcast.clone();
        
        // Subscribe to broadcasts
        let mut rx = broadcast.subscribe();
        
        // Announce new user
        broadcast.send_json(&ChatMessage {
            username: "system".to_string(),
            content: "A new user has joined".to_string(),
            timestamp: now(),
        });
        
        // Spawn task to forward broadcasts to this client
        let send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if sender.send(msg).await.is_err() {
                    break;
                }
            }
        });
        
        // Handle incoming messages from this client
        while let Some(Ok(msg)) = receiver.recv().await {
            if msg.is_text() {
                // Broadcast to all clients
                broadcast.send(msg);
            }
        }
        
        // Cleanup
        send_task.abort();
    })
}
```

### 4. Server Setup

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState {
        chat_broadcast: Arc::new(Broadcast::new()),
    });
    
    RustApi::new()
        .route("/ws/echo", get(ws_echo))
        .route("/ws/json", get(ws_json))
        .route("/ws/chat", get(ws_chat))
        .state(state)
        .run("127.0.0.1:8080")
        .await
}
```

## Key Concepts

### Message Types

| Type | Description |
|------|-------------|
| `Message::Text(String)` | UTF-8 text message |
| `Message::Binary(Vec<u8>)` | Binary data |
| `Message::Ping(Vec<u8>)` | Keepalive ping |
| `Message::Pong(Vec<u8>)` | Ping response |
| `Message::Close(Option<CloseFrame>)` | Close connection |

### Broadcast Pattern

```rust
// Create broadcast channel
let broadcast = Broadcast::new();

// Subscribe (each client)
let rx = broadcast.subscribe();

// Send to all subscribers
broadcast.send(Message::text("Hello everyone!"));

// Send JSON
broadcast.send_json(&my_data);
```

### Split Pattern

```rust
// Split for concurrent send/receive
let (sender, receiver) = socket.split();

// Now you can:
// - Spawn a task for sending
// - Use receiver in main loop
```

### Connection Lifecycle

```
Client                    Server
   │                         │
   │─── HTTP Upgrade ────────▶│
   │◀── 101 Switching ───────│
   │                         │
   │◀═══ WebSocket ══════════▶│  (bidirectional)
   │                         │
   │─── Message ─────────────▶│
   │◀── Message ─────────────│
   │                         │
   │─── Close ───────────────▶│
   │◀── Close ───────────────│
```

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.1", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
futures-util = "0.3"
tracing = "0.1"
```

## Common Patterns

### Heartbeat/Keepalive

```rust
let heartbeat = tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        if sender.send(Message::Ping(vec![])).await.is_err() {
            break;
        }
    }
});
```

### Room-Based Chat

```rust
struct Rooms {
    rooms: DashMap<String, Broadcast>,
}

impl Rooms {
    fn join(&self, room_id: &str) -> broadcast::Receiver<Message> {
        self.rooms
            .entry(room_id.to_string())
            .or_insert_with(Broadcast::new)
            .subscribe()
    }
}
```

### Authentication

```rust
async fn ws_auth(
    ws: WebSocket,
    Query(params): Query<AuthParams>,
) -> Result<WebSocketUpgrade, ApiError> {
    // Validate token before upgrade
    let claims = validate_token(&params.token)?;
    
    Ok(ws.on_upgrade(move |socket| async move {
        // claims available in handler
    }))
}
```

## Next Steps

- **[templates](../templates/)** — Server-side rendering
- **[proof-of-concept](../proof-of-concept/)** — Full application with SSE

## Related Documentation

- [FEATURES.md](../FEATURES.md#ws) — WebSocket feature reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
