# Microservices Advanced Example

Advanced microservices architecture with service discovery and Docker deployment.

> 📖 **Cookbook**: [Deployment](https://tuntii.github.io/RustAPI/) · [Production Tuning](https://tuntii.github.io/RustAPI/)

## Prerequisites

- Rust 1.70+
- Docker and Docker Compose
- Understanding of [microservices](../microservices/) example

## Overview

This example extends the basic microservices pattern with:

- Service Registry for dynamic discovery
- Heartbeat-based health monitoring
- Docker Compose deployment
- Multiple service instances

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Service Registry                       │
│                    (Port 8080)                          │
└───────────────────────────┬─────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│    Product    │   │    Product    │   │     Order     │
│  Service (1)  │   │  Service (2)  │   │    Service    │
│   :8081       │   │   :8082       │   │    :8083      │
└───────────────┘   └───────────────┘   └───────────────┘
```

## Services

### Service Registry (Port 8080)

Central registry for service discovery:

| Method | Path | Description |
|--------|------|-------------|
| POST | `/register` | Register service instance |
| GET | `/discover/{service_name}` | Get service instances |
| POST | `/heartbeat` | Update heartbeat |

### Product Service

Product management microservice:

| Method | Path | Description |
|--------|------|-------------|
| GET | `/products` | List products |
| GET | `/products/{id}` | Get product |
| POST | `/products` | Create product |

### Order Service

Order management microservice:

| Method | Path | Description |
|--------|------|-------------|
| GET | `/orders` | List orders |
| GET | `/orders/{id}` | Get order |
| POST | `/orders` | Create order |

## Project Structure

```
microservices-advanced/
├── src/
│   ├── registry.rs    # Service Registry
│   ├── product.rs     # Product Service
│   └── order.rs       # Order Service
├── Cargo.toml
├── Dockerfile
└── docker-compose.yml
```

## Running

### Local Development

```bash
# Run all services
cargo run -p microservices-advanced
```

### Docker Deployment

```bash
# Build and run with Docker Compose
docker-compose up --build

# Scale services
docker-compose up --scale product=3 --scale order=2
```

## Testing

### Register a Service

```bash
curl -X POST http://localhost:8080/register \
  -H "Content-Type: application/json" \
  -d '{"service_name": "product", "url": "http://product:8081"}'
```

### Discover Services

```bash
curl http://localhost:8080/discover/product
```

### Send Heartbeat

```bash
curl -X POST http://localhost:8080/heartbeat \
  -H "Content-Type: application/json" \
  -d '{"service_name": "product", "url": "http://product:8081"}'
```

## Code Walkthrough

### Service Registry

```rust
#[derive(Clone)]
pub struct RegistryState {
    pub services: Arc<DashMap<String, Vec<ServiceInstance>>>,
}

#[rustapi_rs::post("/register")]
async fn register(
    State(state): State<RegistryState>,
    Json(req): Json<RegisterRequest>,
) -> Json<ServiceInstance> {
    let instance = ServiceInstance {
        url: req.url,
        last_heartbeat: now(),
    };
    
    state.services
        .entry(req.service_name)
        .or_default()
        .push(instance.clone());
    
    Json(instance)
}
```

### Service Discovery

```rust
#[rustapi_rs::get("/discover/{name}")]
async fn discover(
    State(state): State<RegistryState>,
    Path(name): Path<String>,
) -> Result<Json<DiscoverResponse>, ApiError> {
    let instances = state.services
        .get(&name)
        .map(|v| v.clone())
        .unwrap_or_default();
    
    // Filter out stale instances (no heartbeat in 30s)
    let healthy: Vec<_> = instances
        .into_iter()
        .filter(|i| now() - i.last_heartbeat < 30)
        .collect();
    
    Ok(Json(DiscoverResponse { instances: healthy }))
}
```

## Docker Compose

```yaml
version: '3.8'
services:
  registry:
    build: .
    command: ["./microservices-advanced", "registry"]
    ports:
      - "8080:8080"
  
  product:
    build: .
    command: ["./microservices-advanced", "product"]
    environment:
      - REGISTRY_URL=http://registry:8080
    depends_on:
      - registry
  
  order:
    build: .
    command: ["./microservices-advanced", "order"]
    environment:
      - REGISTRY_URL=http://registry:8080
    depends_on:
      - registry
```

## Key Concepts

### Service Discovery Pattern

```
1. Service starts
2. Service registers with Registry
3. Service sends periodic heartbeats
4. Clients discover services via Registry
5. Registry removes stale instances
```

### Heartbeat Mechanism

```rust
// Service sends heartbeat every 10 seconds
let heartbeat = tokio::spawn(async move {
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        client.post(&format!("{}/heartbeat", registry_url))
            .json(&HeartbeatRequest { service_name, url })
            .send()
            .await
            .ok();
    }
});
```

### Load Balancing

```rust
// Simple round-robin selection
fn select_instance(instances: &[ServiceInstance]) -> Option<&ServiceInstance> {
    if instances.is_empty() {
        return None;
    }
    let index = COUNTER.fetch_add(1, Ordering::Relaxed) % instances.len();
    instances.get(index)
}
```

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.1" }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
dashmap = "5.5"
uuid = { version = "1", features = ["v4"] }
tracing = "0.1"
```

## Production Considerations

1. **Use proper service mesh** — Consider Istio, Linkerd
2. **Health checks** — Add liveness and readiness probes
3. **Circuit breaker** — Handle service failures gracefully
4. **Distributed tracing** — OpenTelemetry integration
5. **Config management** — Externalize configuration

## Next Steps

- **[phase11-demo](../phase11-demo/)** — Circuit breaker, guards
- **[proof-of-concept](../proof-of-concept/)** — Full application

## Related Documentation

- [LEARNING_PATH.md](../LEARNING_PATH.md) — Learning progression
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
