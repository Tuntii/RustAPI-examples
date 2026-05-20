// Run with: cargo run -p jwt-auth
// Then visit: http://127.0.0.1:3000/docs
//
// Quick test:
//   1. POST /auth/login  {"username":"alice","password":"secret"}
//   2. Copy the token from the response.
//   3. GET /profile  -H "Authorization: Bearer <token>"
//
// Lesson: JWT authentication with zero manual route registration.
//         #[post/get] macros handle routing; JwtLayer + AuthUser<T> handle auth.

use rustapi_rs::prelude::*;
use rustapi_rs::{description, get, post, summary, tag};
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
// Error type
// ---------------------------------------------------------------------------

// NOTE: #[derive(ApiError)] generates IntoResponse for custom error enums.
//       Handler return types must use ApiError directly (it implements ResponseModifier).
#[derive(ApiError)]
#[allow(dead_code)]
enum AuthError {
    #[error(status = 401, code = "UNAUTHORIZED", message = "Invalid credentials")]
    InvalidCredentials,
}

impl From<AuthError> for ApiError {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::InvalidCredentials => ApiError::unauthorized("Invalid credentials"),
        }
    }
}

// ---------------------------------------------------------------------------
// Handlers — zero .route() calls needed
// ---------------------------------------------------------------------------

#[post("/auth/login")]
#[tag("auth")]
#[summary("Login")]
#[description("Returns a signed JWT that must be sent as `Authorization: Bearer <token>`.")]
async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<TokenResponse>, ApiError> {
    // Hard-coded credentials for the demo.  Use a real user store in production.
    if payload.username != "alice" || payload.password != "secret" {
        return Err(ApiError::unauthorized("Invalid credentials"));
    }

    let ttl = 3600_u64;
    let claims = Claims {
        sub: payload.username,
        role: "user".to_string(),
        exp: now_plus_secs(ttl),
    };

    let token = create_token(&claims, JWT_SECRET).map_err(|_| ApiError::unauthorized("Invalid credentials"))?;
    Ok(Json(TokenResponse {
        token,
        expires_in: ttl,
    }))
}

#[get("/profile")]
#[tag("profile")]
#[summary("Get my profile")]
#[description("Requires `Authorization: Bearer <token>` header.")]
async fn profile(AuthUser(claims): AuthUser<Claims>) -> Json<ProfileResponse> {
    Json(ProfileResponse {
        username: claims.sub,
        role: claims.role,
    })
}

#[get("/health")]
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
    println!(" -> GET  http://127.0.0.1:3000/__rustapi/dashboard");

    // Only the JWT layer is wired up — all routes are auto-discovered from the macros above.
    RustApi::auto()
        .layer(
            JwtLayer::<Claims>::new(JWT_SECRET)
                .skip_paths(vec!["/auth/login", "/health", "/docs", "/__rustapi/dashboard"]),
        )
        .dashboard(DashboardConfig::new())
        .run("127.0.0.1:3000")
        .await
}
