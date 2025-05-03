mod auth;
mod users;
mod votings;

use axum::Router;
use crate::app_state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::create_router())
        .nest("/users", users::router::create_router())
        .nest("/votings", votings::create_router())
}
