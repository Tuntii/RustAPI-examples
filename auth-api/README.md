# Authentication API Example

A complete JWT authentication system demonstrating secure API patterns with RustAPI.

> 📖 **Cookbook**: [Recipes → JWT Authentication](https://tuntii.github.io/RustAPI/)

## Overview

This example implements a full authentication flow including:

- JWT token generation and validation
- Login endpoint with credential verification
- Protected routes requiring valid tokens
- Role-based access control
- Rate limiting for security
- Basic Auth protected documentation

## Prerequisites

- Rust 1.70+
- Completed [crud-api](../crud-api/) example
- Understanding of JWT concepts

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `JwtLayer<Claims>` | JWT validation middleware |
| `AuthUser<T>` | Authenticated user extractor |
| `skip_paths()` | Public route configuration |
| `RateLimitLayer` | Request throttling |
| `docs_with_auth()` | Protected Swagger UI |
| `create_token()` | JWT generation |

## Quick Start

```bash
# Run the example
cargo run -p auth-api

# Server starts at http://127.0.0.1:8080
```

## API Endpoints

### Public Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Welcome message |
| GET | `/health` | Health check |
| POST | `/auth/login` | Login and get JWT token |

### Protected Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/protected/profile` | Get current user profile |
| GET | `/protected/admin` | Admin-only endpoint |
| GET | `/protected/data` | Protected data access |

### Documentation

| Path | Auth | Description |
|------|------|-------------|
| `/docs` | Basic Auth (docs/docs123) | Swagger UI |

## Testing

### 1. Login to Get Token

```bash
# Login with demo credentials
curl -X POST http://127.0.0.1:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "secret"}'

# Response:
# {
#   "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
#   "token_type": "Bearer",
#   "expires_in": 3600
# }
```

### 2. Access Protected Routes

```bash
# Save token to variable
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."

# Get user profile
curl http://127.0.0.1:8080/protected/profile \
  -H "Authorization: Bearer $TOKEN"

# Admin endpoint
curl http://127.0.0.1:8080/protected/admin \
  -H "Authorization: Bearer $TOKEN"
```

### 3. Test Without Token

```bash
# This will return 401 Unauthorized
curl http://127.0.0.1:8080/protected/profile
```

### 4. Access Documentation

```bash
# Open in browser with Basic Auth
open http://127.0.0.1:8080/docs
# Username: docs
# Password: docs123
```

## Code Walkthrough

### 1. JWT Claims Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // Subject (user ID)
    pub username: String,  // Username
    pub role: String,      // User role
    pub exp: u64,          // Expiration timestamp
}
```

### 2. Login Handler

```rust
#[rustapi_rs::post("/auth/login")]
async fn login(
    ValidatedJson(body): ValidatedJson<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    // Verify credentials (in production, check against database)
    if body.username != "admin" || body.password != "secret" {
        return Err(ApiError::unauthorized("Invalid credentials"));
    }

    // Calculate expiration
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + TOKEN_EXPIRY_SECS;

    // Create claims
    let claims = Claims {
        sub: "user-123".to_string(),
        username: body.username,
        role: "admin".to_string(),
        exp,
    };

    // Generate token
    let token = create_token(&claims, JWT_SECRET)?;

    Ok(Json(LoginResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: TOKEN_EXPIRY_SECS,
    }))
}
```

### 3. Protected Route Handler

```rust
#[rustapi_rs::get("/protected/profile")]
async fn get_profile(AuthUser(claims): AuthUser<Claims>) -> Json<UserProfile> {
    // claims is automatically extracted from validated JWT
    Json(UserProfile {
        user_id: claims.sub,
        username: claims.username,
        role: claims.role,
    })
}
```

### 4. Role-Based Access

```rust
#[rustapi_rs::get("/protected/admin")]
async fn admin_only(AuthUser(claims): AuthUser<Claims>) -> Result<Json<Message>, ApiError> {
    if claims.role != "admin" {
        return Err(ApiError::forbidden("Admin access required"));
    }
    Ok(Json(Message { message: "Admin area".to_string() }))
}
```

### 5. Server Configuration

```rust
let app = RustApi::config()
    .docs_enabled(false)  // We'll add docs with custom auth
    .body_limit(1024 * 1024)
    .layer(RequestIdLayer::new())
    .layer(TracingLayer::new())
    .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
    .layer(JwtLayer::<Claims>::new(JWT_SECRET)
        .skip_paths(vec!["/health", "/docs", "/auth/login", "/"]))
    .build()
    .docs_with_auth("/docs", "docs", "docs123");
```

## Key Concepts

### JWT Layer Configuration

The `JwtLayer` automatically:
- Validates JWT tokens in `Authorization: Bearer <token>` header
- Extracts claims into `AuthUser<T>` for handlers
- Returns 401 for invalid/expired tokens
- Skips validation for specified paths

```rust
JwtLayer::<Claims>::new("your-secret-key")
    .skip_paths(vec!["/login", "/register", "/public"])
```

### AuthUser Extractor

```rust
// Simply add AuthUser<YourClaimsType> to handler parameters
async fn handler(AuthUser(claims): AuthUser<Claims>) -> impl IntoResponse {
    // claims.sub, claims.username, etc. are available
}
```

### Security Best Practices

1. **Never hardcode secrets** — Use environment variables in production
2. **Set appropriate expiration** — Balance security vs UX
3. **Rate limit auth endpoints** — Prevent brute force attacks
4. **Use HTTPS in production** — JWT tokens are sensitive
5. **Implement token refresh** — For long-lived sessions

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.2", features = ["jwt", "rate-limit"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
validator = "0.16"
utoipa = "4"
```

## Next Steps

After mastering authentication:

1. **[middleware-chain](../middleware-chain/)** — Custom middleware composition
2. **[proof-of-concept](../proof-of-concept/)** — Full application with auth

## Related Documentation

- [LEARNING_PATH.md](../LEARNING_PATH.md) — Learning progression
- [FEATURES.md](../FEATURES.md#jwt) — JWT feature reference
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/) — Full documentation

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
