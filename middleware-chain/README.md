# Middleware Chain Example

Composing custom middleware for request processing pipelines in RustAPI.

> 📖 **Cookbook**: [Recipes → Custom Middleware](https://tuntii.github.io/RustAPI/)

## Prerequisites

- Rust 1.70+
- Understanding of Tower's `Layer` trait
- Completed [auth-api](../auth-api/) example

## Overview

This example demonstrates how to compose custom middleware in RustAPI.

## Features

- **Request ID tracking** — Add unique ID to each request
- **Request timing** — Log execution duration
- **Custom authentication** — Token-based auth middleware
- **Error handling** — Graceful error responses
- **Middleware composition** — Chain multiple middleware together

## Running

```bash
cargo run -p middleware-chain
```

Then test:
```bash
# Public endpoint (no auth required)
curl http://127.0.0.1:8080/api/public

# Protected endpoint (requires auth token)
curl -H "Authorization: Bearer token123" http://127.0.0.1:8080/api/protected

# Protected endpoint without token (should fail)
curl http://127.0.0.1:8080/api/protected
```

## Middleware Execution Order

```
Request → RequestID → Timing → Auth → Handler → Response
                                       ↓
                              (if auth fails)
                                       ↓
                              401 Unauthorized
```

## Custom Middleware Examples

### 1. Request ID Middleware
Adds a unique UUID to each request and includes it in response headers.

```rust
struct RequestIdMiddleware;

impl RequestIdMiddleware {
    async fn handle<B>(&self, req: Request<B>, next: Next<B>) -> Response {
        let request_id = Uuid::new_v4().to_string();
        let mut response = next.run(req).await;
        response.headers_mut().insert("X-Request-ID", request_id.parse().unwrap());
        response
    }
}
```

### 2. Timing Middleware
Logs the duration of each request.

```rust
struct TimingMiddleware;

impl TimingMiddleware {
    async fn handle<B>(&self, req: Request<B>, next: Next<B>) -> Response {
        let start = Instant::now();
        let response = next.run(req).await;
        println!("⏱️  Request took {}ms", start.elapsed().as_millis());
        response
    }
}
```

### 3. Custom Auth Middleware
Validates Bearer tokens for protected routes.

```rust
struct CustomAuthMiddleware;

impl CustomAuthMiddleware {
    async fn handle<B>(&self, req: Request<B>, next: Next<B>) -> Response {
        if req.uri().path().starts_with("/api/protected") {
            // Validate auth header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if is_valid_token(auth_header) {
                    return next.run(req).await;
                }
            }
            return unauthorized_response();
        }
        next.run(req).await
    }
}
```

## Composing Middleware

```rust
RustApi::auto()
    .middleware(RequestIdMiddleware::new())  // First
    .middleware(TimingMiddleware::new())      // Second
    .middleware(CustomAuthMiddleware::new())  // Third
    .run("127.0.0.1:8080")
    .await
```

## Use Cases

- **Logging & Tracing** — Track requests across services
- **Authentication** — JWT validation, API keys
- **Rate Limiting** — Throttle requests per user/IP
- **CORS** — Handle cross-origin requests
- **Compression** — gzip/brotli response compression
- **Caching** — Redis/memory cache layer
- **Error Handling** — Convert errors to proper HTTP responses

## Production Tips

1. **Order matters** — Put fast middleware first (auth before DB)
2. **Short-circuit on failure** — Don't call `next` if validation fails
3. **Add timeouts** — Prevent slow requests from blocking
4. **Use tower layers** — Leverage existing middleware ecosystem
5. **Test middleware independently** — Unit test each middleware
