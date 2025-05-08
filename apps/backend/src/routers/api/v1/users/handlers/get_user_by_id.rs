use uuid::Uuid;

use axum::extract::{Path, State, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use types::api_response::{ErrorApiResponse, ErrorApiResponseData};
use types::dto::response::GetUserPayload;

use crate::app_state::AppState;
use crate::repository::user_repository::GetUserQuery;
use crate::utils::get_internal_error_response;

pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let user = state.user_repository
        .get_user(GetUserQuery::from(id))
        .await;

    match user {
        Err(_error) => get_internal_error_response(),
        Ok(user) => match user {
            None => (
                StatusCode::NOT_FOUND,
                Json(ErrorApiResponse::from(
                    ErrorApiResponseData::new(
                        StatusCode::NOT_FOUND.as_str(),
                        Some("User not found".to_string())
                    )
                ))
            ).into_response(),
            Some(user) => (
                StatusCode::OK,
                Json(GetUserPayload::new(&user)),
            ).into_response()
        }
    }
}
