use axum::Router;

use crate::app_state::AppState;

pub mod api;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/api", api::create_router(state.clone()))
}
