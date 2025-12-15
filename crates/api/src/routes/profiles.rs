use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    routing::patch,
    Json, Router,
};
use serde::Deserialize;
use shared::types::Profile;
use tower_cookies::Cookies;

pub fn routes() -> Router<AppState> {
    // Only mutation endpoint - reads come through WebSocket
    Router::new().route("/api/profiles/{id}", patch(update_profile))
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

async fn update_profile(
    State(state): State<AppState>,
    cookies: Cookies,
    Path(id): Path<i64>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<Profile>, AppError> {
    // Get current user from session
    let user_id = get_current_user_id(&state, &cookies).await?;

    // Get the profile to check ownership
    let profile = state
        .profile_service
        .get_profile_by_id(id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    // Check ownership
    if profile.user_id != user_id {
        return Err(AppError::Unauthorized);
    }

    // Update profile (ProfileService handles broadcast automatically)
    let updated = state
        .profile_service
        .update_profile(id, req.display_name.as_deref(), req.bio.as_deref())
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    Ok(Json(updated))
}

async fn get_current_user_id(state: &AppState, cookies: &Cookies) -> Result<i64, AppError> {
    let session_id = cookies
        .get("session_id")
        .ok_or(AppError::Unauthorized)?
        .value()
        .to_string();

    state
        .sessions
        .read()
        .await
        .get(&session_id)
        .copied()
        .ok_or(AppError::Unauthorized)
}
