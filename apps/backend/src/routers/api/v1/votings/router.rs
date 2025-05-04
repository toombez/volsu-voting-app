use axum::{middleware, routing, Router};

use crate::{app_state::AppState, routers::api::v1::middleware::auth_only::auth_only_middleware};

use super::handlers;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            routing::post(handlers::create_voting::create_voting)
                .layer(middleware::from_fn_with_state(state.clone(), auth_only_middleware))
        )
        .route(
            "/{id}",
            routing::post(handlers::user_vote::user_vote)
                .layer(middleware::from_fn_with_state(state.clone(), auth_only_middleware))
        )
        .route("/", routing::get(handlers::get_votings_list::get_votings_list))
        .route("/{id}", routing::get(handlers::get_voting_by_id::get_voting_by_id))
}
