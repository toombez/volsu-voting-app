use axum::Router;

use crate::app_state::AppState;

pub mod v1;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::create_router(state.clone()))
}
