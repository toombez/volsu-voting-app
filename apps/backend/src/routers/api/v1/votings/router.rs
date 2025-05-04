use axum::{middleware, routing, Router};

use crate::{app_state::AppState, routers::api::v1::middleware::auth_only::auth_only_middleware};

use super::handlers::create_voting::create_voting;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            routing::post(create_voting)
        )
        .layer(middleware::from_fn_with_state(state.clone(), auth_only_middleware))
}
