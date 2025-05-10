use axum::extract::{Query, State, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use types::dto::request::PaginationQuery;
use types::dto::response::GetUsersListPayload;

use crate::app_state::AppState;
use crate::utils::get_internal_error_response;

pub async fn get_users_list(
    Query(pagination): Query<PaginationQuery>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let users = state
        .user_repository
        .get_users(pagination)
        .await;

    match users {
        Err(_error) => get_internal_error_response(),
        Ok(users) => (
            StatusCode::OK,
            Json(GetUsersListPayload::new(&users))
        ).into_response()
    }
}
