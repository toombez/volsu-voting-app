mod auth;
mod users;
mod votings;
pub mod middleware;

use axum::Router;
use crate::app_state::AppState;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router::create_router(state.clone()))
        .nest("/users", users::router::create_router(state.clone()))
        .nest("/votings", votings::create_router(state.clone()))
}
