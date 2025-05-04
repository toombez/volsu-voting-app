use axum::{routing, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/register",
            routing::post(handlers::register_user::register)
        )
        .route("/", routing::get(handlers::get_users_list::get_users_list))
        .route("/{id}", routing::get(handlers::get_user_by_id::get_user_by_id))
}
