use axum::Router;

use crate::app_state::AppState;

pub fn create_router(_state: AppState) -> Router<AppState> {
    Router::new()
}
