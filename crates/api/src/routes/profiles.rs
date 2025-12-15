use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use shared::types::Profile;
use tower_cookies::Cookies;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/profiles", get(list_profiles))
        .route("/api/profiles/{id}", get(get_profile).patch(update_profile))
}

async fn list_profiles(State(state): State<AppState>) -> Result<Json<Vec<Profile>>, AppError> {
    let profiles = db::list_profiles(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(profiles))
}

async fn get_profile(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Profile>, AppError> {
    let profile = db::get_profile_by_id(&state.db, id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    Ok(Json(profile))
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
    let profile = db::get_profile_by_id(&state.db, id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    // Check ownership
    if profile.user_id != user_id {
        return Err(AppError::Unauthorized);
    }

    // Update profile
    let updated = db::update_profile(
        &state.db,
        id,
        req.display_name.as_deref(),
        req.bio.as_deref(),
    )
    .await
    .map_err(|e| AppError::Internal(e.into()))?
    .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    // Broadcast update
    let _ = state
        .events_tx
        .send(shared::types::WsEvent::Profile(updated.clone()));

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
