use crate::{error::AppError, state::AppState};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user_id: i64,
    pub email: String,
    pub profile: shared::types::Profile,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
}

async fn register(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate input
    if req.email.is_empty() || req.password.is_empty() || req.display_name.is_empty() {
        return Err(AppError::BadRequest("All fields are required".to_string()));
    }

    // Hash password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hashing failed: {e}")))?
        .to_string();

    // Create user
    let user_id = db::create_user(&state.db, &req.email, &password_hash)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                AppError::BadRequest("Email already registered".to_string())
            }
            _ => AppError::Internal(e.into()),
        })?;

    // Create profile (ProfileService handles broadcast automatically)
    let profile = state
        .profile_service
        .create_profile(user_id, &req.display_name)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    // Create session
    let session_id = Uuid::new_v4().to_string();
    state
        .sessions
        .write()
        .await
        .insert(session_id.clone(), user_id);

    // Set cookie
    let mut cookie = Cookie::new("session_id", session_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookies.add(cookie);

    Ok(Json(AuthResponse {
        user_id,
        email: req.email,
        profile,
    }))
}

async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find user
    let user = db::get_user_by_email(&state.db, &req.email)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::Unauthorized)?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid password hash: {e}")))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    // Get profile
    let profile = state
        .profile_service
        .get_profile_by_user_id(user.id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Profile not found")))?;

    // Create session
    let session_id = Uuid::new_v4().to_string();
    state
        .sessions
        .write()
        .await
        .insert(session_id.clone(), user.id);

    // Set cookie
    let mut cookie = Cookie::new("session_id", session_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookies.add(cookie);

    Ok(Json(AuthResponse {
        user_id: user.id,
        email: user.email,
        profile,
    }))
}

async fn logout(State(state): State<AppState>, cookies: Cookies) -> Result<(), AppError> {
    if let Some(cookie) = cookies.get("session_id") {
        state.sessions.write().await.remove(cookie.value());
        cookies.remove(Cookie::from("session_id"));
    }
    Ok(())
}
