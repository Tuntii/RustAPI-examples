# RustAPI Examples

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![RustAPI](https://img.shields.io/badge/RustAPI-0.2.x-blue)](https://github.com/Tuntii/RustAPI)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green)](LICENSE)
[![Cookbook](https://img.shields.io/badge/📖-Cookbook-purple)](https://tuntii.github.io/RustAPI/)

> **Production-ready examples** demonstrating the full power of [RustAPI](https://github.com/Tuntii/RustAPI) — the Rust web framework with FastAPI-like developer experience.

**📚 Official Resources:**
- 🔗 [RustAPI Framework](https://github.com/Tuntii/RustAPI) — Main repository
- 📖 [Cookbook Documentation](https://tuntii.github.io/RustAPI/) — Comprehensive guides & recipes
- 💬 [Discussions](https://github.com/Tuntii/RustAPI/discussions) — Community support

---

## 🚀 Why RustAPI?

> *"Rust Speed. Python Simplicity. AI Efficiency."*

| Metric | RustAPI | Actix | Axum | FastAPI (Python) |
|--------|---------|-------|------|------------------|
| **Performance** | ~92k req/s | ~105k | ~100k | ~12k |
| **Developer Experience** | 🟢 High | 🔴 Low | 🟡 Medium | 🟢 High |
| **Boilerplate** | Zero | High | Medium | Zero |
| **AI/LLM Native** | ✅ | ❌ | ❌ | ❌ |
| **Type Safety** | ✅ | ✅ | ✅ | ❌ |

### Key Features

- **🎯 Zero Boilerplate** — `RustApi::auto()` discovers all routes automatically
- **📝 Auto Documentation** — OpenAPI/Swagger at `/docs` out of the box
- **✅ Built-in Validation** — `#[validate]` with detailed error messages
- **🔐 JWT Authentication** — First-class `JwtLayer` and `AuthUser<T>` extractor
- **🤖 AI-First Architecture** — TOON format (50-58% token savings), MCP support
- **⚡ Middleware Stack** — Rate limiting, CORS, logging, circuit breaker, and more
- **🌐 Real-time** — WebSocket support with broadcast channels

---

## 📋 Prerequisites

Before running these examples, ensure you have:

```bash
# Rust 1.70 or later
rustc --version

# Clone the examples repository
git clone https://github.com/Tuntii/rustapi-rs-examples.git
cd rustapi-rs-examples

# Build all examples
cargo build --release
```

**Optional dependencies** (for specific examples):
- **Docker** — For `sqlx-crud` (PostgreSQL/SQLite)
- **SQLite** — For `sqlx-crud` local testing

---

## 📂 Examples Overview

This repository contains **18 production-ready examples** organized by category:

### 🌟 Getting Started

| Example | Difficulty | Description | Key Features |
|---------|------------|-------------|--------------|
| [hello-world](hello-world/) | ⭐ | Minimal 20-line API | `RustApi::auto()`, path params, `Json` response |
| [crud-api](crud-api/) | ⭐⭐ | Complete CRUD operations | Validation, pagination, error handling, body limits |
| [proof-of-concept](proof-of-concept/) | ⭐⭐⭐ | Full-featured bookmark manager | JWT, CRUD, SSE, modular handlers, Swagger UI |

### 🔐 Authentication & Security

| Example | Difficulty | Description | Key Features |
|---------|------------|-------------|--------------|
| [auth-api](auth-api/) | ⭐⭐⭐ | JWT authentication system | Login/register, `JwtLayer`, `AuthUser<T>`, protected routes |
| [rate-limit-demo](rate-limit-demo/) | ⭐⭐ | IP-based rate limiting | Per-endpoint limits, burst support, 429 handling |
| [middleware-chain](middleware-chain/) | ⭐⭐⭐ | Custom middleware composition | Request ID, timing, auth, middleware ordering |
| [cors-test](cors-test/) | ⭐⭐ | CORS configuration | `CorsLayer`, allowed origins/methods/headers |

### 🗄️ Database Integration

| Example | Difficulty | Description | Key Features |
|---------|------------|-------------|--------------|
| [sqlx-crud](sqlx-crud/) | ⭐⭐⭐ | SQLx + SQLite/PostgreSQL | Connection pooling, transactions, migrations |
| [event-sourcing](event-sourcing/) | ⭐⭐⭐⭐ | Event sourcing pattern | CQRS, domain events, aggregate reconstruction |

### 🤖 AI & LLM Integration

| Example | Difficulty | Description | Key Features |
|---------|------------|-------------|--------------|
| [toon-api](toon-api/) | ⭐⭐ | Token-optimized responses | `ToonResponse`, content negotiation, token headers |
| [mcp-server](05-mcp-server/) | ⭐⭐⭐ | Model Context Protocol (MCP tools) | In-process invocation (`InvocationMode::InProcess`), agent tools, Claude/Cursor integration, full pipeline respect |

### 🌐 Real-time & Web

| Example | Difficulty | Description | Key Features |
|---------|------------|-------------|--------------|
| [websocket](websocket/) | ⭐⭐⭐ | WebSocket chat server | Broadcast channels, pub/sub, connection management |
| [templates](templates/) | ⭐⭐ | Server-side rendering | Tera templates, inheritance, static files |

### 🏗️ Advanced Architecture

| Example | Difficulty | Description | Key Features |
|---------|------------|-------------|--------------|
| [graphql-api](graphql-api/) | ⭐⭐⭐⭐ | GraphQL integration | async-graphql, queries/mutations, playground |
| [microservices](microservices/) | ⭐⭐⭐⭐ | API Gateway pattern | Service-to-service communication, routing |
| [microservices-advanced](microservices-advanced/) | ⭐⭐⭐⭐ | Service discovery | Registry, heartbeat, Docker Compose |
| [phase11-demo](phase11-demo/) | ⭐⭐⭐⭐ | Advanced middleware | Guards, circuit breaker, timeout, logging |
| [serverless-lambda](serverless-lambda/) | ⭐⭐⭐ | AWS Lambda deployment | SAM template, cold start optimization |

> ⚠️ **Note**: `serverless-lambda` uses AWS Lambda HTTP runtime instead of RustAPI for serverless deployment patterns.

---

## 🎯 Feature Coverage Matrix

| Feature | Examples Using It |
|---------|-------------------|
| `RustApi::auto()` | All examples |
| `Json<T>` / `JsonResponse` | crud-api, auth-api, proof-of-concept, graphql-api |
| `#[validate]` | crud-api, auth-api, proof-of-concept |
| `JwtLayer` / `AuthUser<T>` | auth-api, middleware-chain, phase11-demo, proof-of-concept |
| `RateLimitLayer` | rate-limit-demo, auth-api, cors-test, proof-of-concept |
| `CorsLayer` | cors-test, middleware-chain, proof-of-concept |
| `ToonResponse` | toon-api, mcp-server |
| `MCP` (protocol-mcp) | mcp-server |
| `WebSocket` / `WsConnection` | websocket |
| `View<T>` / `ViewEngine` | templates |
| `State<T>` | All examples with shared state |
| `RequestIdLayer` | middleware-chain, phase11-demo |
| `CircuitBreakerLayer` | phase11-demo |
| `TimeoutLayer` | phase11-demo |

---

## 📊 RustAPI Feature Flags

Each example uses specific Cargo feature flags. See [FEATURES.md](FEATURES.md) for detailed documentation.

| Feature Flag | Purpose | Examples |
|--------------|---------|----------|
| `full` | All features enabled | crud-api, phase11-demo |
| `jwt` | JWT authentication | auth-api, middleware-chain, proof-of-concept |
| `cors` | CORS middleware | cors-test, middleware-chain, proof-of-concept |
| `rate-limit` | Rate limiting | rate-limit-demo, auth-api, cors-test, proof-of-concept |
| `toon` | TOON format for LLMs | toon-api, mcp-server |
| `protocol-mcp` | MCP / AI agent tools | mcp-server |
| `ws` | WebSocket support | websocket |
| `view` | Template rendering | templates |
| `swagger-ui` | Swagger UI at /docs | toon-api |

---

## 🚀 Quick Start

### Running an Example

```bash
# Navigate to repository
cd rustapi-rs-examples

# Run any example
cargo run -p hello-world

# Run the MCP + HTTP demo (AI agent tools)
cargo run -p mcp-server

# Run with debug logging
RUST_LOG=debug cargo run -p crud-api

# Access Swagger documentation (when running)
# http://127.0.0.1:8080/docs
```

### Testing API Endpoints

```bash
# Hello World
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8080/hello/RustAPI

# CRUD API
curl http://127.0.0.1:8080/users
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'

# Auth API
curl -X POST http://127.0.0.1:8080/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'
```

---

## 📚 Learning Path

For a structured learning experience, follow our recommended progression in [LEARNING_PATH.md](LEARNING_PATH.md):

```
1. hello-world     → Basic routing, responses
2. crud-api        → Validation, error handling, state management
3. auth-api        → JWT authentication, protected routes
4. middleware-chain → Custom middleware, composition
5. proof-of-concept → Full application architecture
```

---

## 🔗 Cookbook Cross-References

Each example maps to sections in the [RustAPI Cookbook](https://tuntii.github.io/RustAPI/):

| Example | Cookbook Section |
|---------|------------------|
| hello-world | [Getting Started → Quickstart](https://tuntii.github.io/RustAPI/) |
| crud-api | [Recipes → CRUD Resources](https://tuntii.github.io/RustAPI/) |
| auth-api | [Recipes → JWT Authentication](https://tuntii.github.io/RustAPI/) |
| middleware-chain | [Recipes → Custom Middleware](https://tuntii.github.io/RustAPI/) |
| sqlx-crud | [Recipes → Database Integration](https://tuntii.github.io/RustAPI/) |
| websocket | [Recipes → WebSockets](https://tuntii.github.io/RustAPI/) |
| templates | [Crates → rustapi-view](https://tuntii.github.io/RustAPI/) |
| toon-api | [Crates → rustapi-toon](https://tuntii.github.io/RustAPI/) |

---

## 📝 Creating Your Own Example

1. **Create directory**: `my-example/`

2. **Add Cargo.toml**:
   ```toml
   [package]
   name = "my-example"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   rustapi-rs = { path = "../../crates/rustapi-rs", features = ["full"] }
   tokio = { version = "1", features = ["full"] }
   serde = { version = "1", features = ["derive"] }
   ```

3. **Create src/main.rs**:
   ```rust
   use rustapi_rs::prelude::*;

   #[rustapi_rs::get("/")]
   async fn index() -> &'static str {
       "Hello from my example!"
   }

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
       RustApi::auto().run("127.0.0.1:8080").await
   }
   ```

4. **Run it**:
   ```bash
   cargo run -p my-example
   ```

---

## 🎯 Roadmap

### Coming Soon
- [ ] **redis-cache** — Redis caching layer
- [ ] **sse-events** — Server-Sent Events streaming
- [ ] **grpc-integration** — gRPC + REST hybrid API
- [ ] **distributed-tracing** — OpenTelemetry integration
- [ ] **kubernetes-ready** — Health checks, metrics, graceful shutdown

---

## 🤝 Contributing

Contributions are welcome! See our [Contributing Guide](https://github.com/Tuntii/RustAPI/blob/main/CONTRIBUTING.md) for details.

1. Fork this repository
2. Create your example in a new directory
3. Add comprehensive documentation (README.md)
4. Submit a pull request

---

## 📄 License

This project is licensed under the MIT OR Apache-2.0 license, same as [RustAPI](https://github.com/Tuntii/RustAPI).

---

<div align="center">

**Built with ❤️ using [RustAPI](https://github.com/Tuntii/RustAPI)**

[Framework](https://github.com/Tuntii/RustAPI) · [Cookbook](https://tuntii.github.io/RustAPI/) · [Examples](https://github.com/Tuntii/rustapi-rs-examples) · [Discussions](https://github.com/Tuntii/RustAPI/discussions)

</div>
