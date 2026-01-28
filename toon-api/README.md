# TOON API Example

Token-Oriented Object Notation (TOON) for AI/LLM-optimized API responses with 20-40% token savings.

> 📖 **Cookbook**: [Crates → rustapi-toon](https://tuntii.github.io/RustAPI/)

## Overview

TOON is RustAPI's killer feature for AI applications. This example demonstrates:

- TOON format serialization (50-58% token savings)
- Content negotiation (JSON/TOON)
- Token count headers for cost tracking
- Side-by-side comparison with JSON
- LLM-friendly response formatting

## Prerequisites

- Rust 1.70+
- Understanding of [hello-world](../hello-world/) basics
- Interest in AI/LLM development

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `ToonResponse<T>` | TOON format response type |
| `Accept` header | Content negotiation |
| `X-Token-Count` | Token usage tracking |
| `LlmResponse` | Combined TOON + metadata |
| Format comparison | JSON vs TOON metrics |

## Quick Start

```bash
# Run the example
cargo run -p toon-api

# Server starts at http://127.0.0.1:8080
# Documentation at http://127.0.0.1:8080/docs
```

## API Endpoints

### JSON Endpoints (Comparison)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/json/users` | List users as JSON |
| POST | `/json/users` | Create user (JSON) |

### TOON Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/toon/users` | List users as TOON |
| POST | `/toon/users` | Create user (TOON) |

### Content Negotiation

| Method | Path | Description |
|--------|------|-------------|
| GET | `/users` | Auto-select format via Accept header |

### Utilities

| Method | Path | Description |
|--------|------|-------------|
| GET | `/compare` | JSON vs TOON comparison |
| GET | `/format-info` | TOON format documentation |

## Testing

### JSON Response

```bash
curl http://127.0.0.1:8080/json/users

# Response (40+ bytes):
# {"users":[{"id":1,"name":"Alice","email":"alice@example.com","role":"admin"},...]}
```

### TOON Response

```bash
curl http://127.0.0.1:8080/toon/users

# Response (compact, ~28 bytes):
# users[3]{id,name,email,role}:
#   1,Alice,alice@example.com,admin
#   2,Bob,bob@example.com,user
#   3,Charlie,charlie@example.com,user
```

### Content Negotiation

```bash
# Request JSON (default)
curl http://127.0.0.1:8080/users

# Request TOON
curl -H "Accept: application/toon" http://127.0.0.1:8080/users

# Request JSON explicitly
curl -H "Accept: application/json" http://127.0.0.1:8080/users
```

### Token Count Header

```bash
curl -v http://127.0.0.1:8080/toon/users

# Headers include:
# X-Token-Count: 13
# X-Format: toon
# Content-Type: application/toon
```

### Compare Formats

```bash
curl http://127.0.0.1:8080/compare

# Response:
# {
#   "json_bytes": 156,
#   "toon_bytes": 98,
#   "bytes_saved": 58,
#   "savings_percent": "37.2%",
#   "note": "TOON saves ~20-40% tokens for LLM communication"
# }
```

## TOON Format Explained

### JSON (Traditional)

```json
{
  "users": [
    { "id": 1, "name": "Alice", "email": "alice@example.com" },
    { "id": 2, "name": "Bob", "email": "bob@example.com" }
  ]
}
```

**16 tokens, 89 bytes**

### TOON (Optimized)

```
users[2]{id,name,email}:
  1,Alice,alice@example.com
  2,Bob,bob@example.com
```

**10 tokens, 62 bytes — 37.5% savings!**

### Format Structure

```
field_name[count]{schema}:
  value1,value2,value3
  value1,value2,value3
```

- `field_name` — The array/object name
- `[count]` — Number of items (optional)
- `{schema}` — Field names in order
- `:` — Separator
- Lines — One item per line, comma-separated values

## Code Walkthrough

### 1. TOON Response Type

```rust
use rustapi_rs::toon::{ToonResponse, Toon};

#[derive(Serialize, Toon)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[get("/toon/users")]
async fn get_users_toon() -> ToonResponse<UsersResponse> {
    ToonResponse::new(UsersResponse {
        users: get_sample_users(),
        total: 3,
    })
}
```

### 2. Content Negotiation

```rust
#[get("/users")]
async fn get_users(
    accept: Option<TypedHeader<Accept>>,
) -> impl IntoResponse {
    let users = get_sample_users();
    
    // Check Accept header
    let wants_toon = accept
        .map(|h| h.0.to_string().contains("application/toon"))
        .unwrap_or(false);
    
    if wants_toon {
        ToonResponse::new(users).into_response()
    } else {
        Json(users).into_response()
    }
}
```

### 3. LLM Response with Metadata

```rust
use rustapi_rs::toon::LlmResponse;

#[get("/llm/users")]
async fn get_users_llm() -> LlmResponse<UsersResponse> {
    LlmResponse::new(UsersResponse { ... })
        .with_description("User listing for AI processing")
        .with_token_count(true)  // Adds X-Token-Count header
}
```

## Token Savings Examples

| Data Type | JSON Tokens | TOON Tokens | Savings |
|-----------|-------------|-------------|---------|
| User list (10) | 45 | 28 | 38% |
| Product catalog | 120 | 72 | 40% |
| API response | 89 | 52 | 42% |
| Nested objects | 200 | 115 | 42% |

## Why TOON for LLMs?

### Cost Reduction

```
GPT-4 pricing: $0.03/1K input tokens

Daily API calls: 100,000
Avg tokens/call: 500

Without TOON: 50M tokens = $1,500/day
With TOON:    30M tokens = $900/day
                          ─────────
Monthly savings:          $18,000
```

### Faster Processing

- Fewer tokens = faster completion
- Less context window usage
- More room for actual content

### LLM-Native Format

- Schema-first design
- Consistent structure
- Easy to parse by AI

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.1", features = ["toon", "swagger-ui"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
utoipa = "4"
```

## Integration with LLMs

### System Prompt

```
You will receive data in TOON format:
field[count]{schema}:
  values per line

Parse the schema header to understand field order,
then read each line as comma-separated values.
```

### Response Parsing (Python)

```python
def parse_toon(toon_str):
    lines = toon_str.strip().split('\n')
    header = lines[0]
    # Parse header: users[3]{id,name,email}:
    schema = header.split('{')[1].split('}')[0].split(',')
    
    data = []
    for line in lines[1:]:
        values = line.strip().split(',')
        data.append(dict(zip(schema, values)))
    return data
```

## Next Steps

- **[mcp-server](../mcp-server/)** — Model Context Protocol with TOON
- **[proof-of-concept](../proof-of-concept/)** — Full application

## Related Documentation

- [FEATURES.md](../FEATURES.md#toon) — TOON feature reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
