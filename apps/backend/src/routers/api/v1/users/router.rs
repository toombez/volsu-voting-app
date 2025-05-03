use axum::{routing, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route(
            "/register",
            routing::post(handlers::register_user::register)
        )
}
