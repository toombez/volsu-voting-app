mod auth;
mod users;
mod votings;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/auth", auth::create_router())
        .nest("/users", users::create_router())
        .nest("/votings", votings::create_router())
}
