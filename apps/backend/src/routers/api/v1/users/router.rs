use axum::{middleware, routing, Router};

use crate::{app_state::AppState, routers::api::v1::middleware::auth_only::auth_only_middleware};

use super::handlers;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/register",
            routing::post(handlers::register_user::register)
        )
        .route(
            "/",
            routing::get(handlers::get_users_list::get_users_list)
            .patch(routing::patch(handlers::update_user::update_user)
                .layer(middleware::from_fn_with_state(state.clone(), auth_only_middleware))
            )
        )
        .route(
            "/me",
            routing::get(handlers::me::me)
                .layer(middleware::from_fn_with_state(state.clone(), auth_only_middleware))
        )
        .route("/{id}", routing::get(handlers::get_user_by_id::get_user_by_id))
}
