use axum::{routing, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn create_router(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(handlers::login_user::login_user))
}
