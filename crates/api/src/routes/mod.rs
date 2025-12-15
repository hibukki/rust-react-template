mod auth;
mod health;
mod profiles;
mod ws;

use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(auth::routes())
        .merge(profiles::routes())
        .merge(ws::routes())
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
