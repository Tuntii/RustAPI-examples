// Run with: cargo run -p jwt-auth
// Then visit: http://127.0.0.1:3000/docs
//
// Quick test:
//   1. POST /auth/login  {"username":"alice","password":"secret"}
//   2. Copy the token from the response.
//   3. GET /profile  -H "Authorization: Bearer <token>"
//
// Lesson: JWT authentication, JwtLayer middleware, AuthUser<T> extractor,
//         and skipping validation for public routes.

use rustapi_rs::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// JWT Claims
// ---------------------------------------------------------------------------

/// Claims embedded in the JWT.  Must impl `Deserialize + Clone + Send + Sync`.
#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
struct Claims {
    /// Subject — the user's identifier.
    sub: String,
    /// Role for simple RBAC.
    role: String,
    /// Expiry (Unix timestamp).
    exp: u64,
}

const JWT_SECRET: &str = "change-me-in-production";

fn now_plus_secs(secs: u64) -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
        + secs
}

// ---------------------------------------------------------------------------
// Request / response types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Schema)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Schema)]
struct TokenResponse {
    token: String,
    expires_in: u64,
}

#[derive(Debug, Serialize, Schema)]
struct ProfileResponse {
    username: String,
    role: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Public login endpoint — issues a JWT on success.
#[tag("auth")]
#[summary("Login")]
#[description("Returns a signed JWT that must be sent as `Authorization: Bearer <token>`.")]
async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<TokenResponse>, Unauthorized> {
    // Hard-coded credentials for the demo.  Use a real user store in production.
    if payload.username != "alice" || payload.password != "secret" {
        return Err(Unauthorized);
    }

    let ttl = 3600_u64;
    let claims = Claims {
        sub: payload.username,
        role: "user".to_string(),
        exp: now_plus_secs(ttl),
    };

    let token = create_token(&claims, JWT_SECRET).map_err(|_| Unauthorized)?;
    Ok(Json(TokenResponse {
        token,
        expires_in: ttl,
    }))
}

/// Protected endpoint — requires a valid JWT.
#[tag("profile")]
#[summary("Get my profile")]
#[description("Requires `Authorization: Bearer <token>` header.")]
async fn profile(AuthUser(claims): AuthUser<Claims>) -> Json<ProfileResponse> {
    Json(ProfileResponse {
        username: claims.sub,
        role: claims.role,
    })
}

/// Public health check — no JWT needed.
#[tag("ops")]
#[summary("Health check")]
async fn health() -> &'static str {
    "ok"
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting jwt-auth example…");
    println!(" -> POST http://127.0.0.1:3000/auth/login   {{\"username\":\"alice\",\"password\":\"secret\"}}");
    println!(" -> GET  http://127.0.0.1:3000/profile      (Authorization: Bearer <token>)");
    println!(" -> GET  http://127.0.0.1:3000/health       (public)");
    println!(" -> GET  http://127.0.0.1:3000/docs         (Swagger UI)");

    RustApi::new()
        .swagger_ui("/docs")
        // JwtLayer validates every request except the skipped paths.
        // Layer execution order is LIFO — place auth after logging/CORS.
        .layer(
            JwtLayer::<Claims>::new(JWT_SECRET)
                .skip_paths(vec!["/auth/login", "/health", "/docs"]),
        )
        .route("/auth/login", post(login))
        .route("/profile", get(profile))
        .route("/health", get(health))
        .run("127.0.0.1:3000")
        .await
}
