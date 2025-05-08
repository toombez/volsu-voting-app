use axum::extract::{State, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use types::dto::{request::CreateUserBody, response::CreateUserPayload};

use crate::{app_state::AppState, utils::get_internal_error_response};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> impl IntoResponse {
    let user = state.user_repository
        .create_user(&body)
        .await;

    match user {
        Err(_error) => get_internal_error_response(),
        Ok(user) => (
            StatusCode::CREATED,
            Json(CreateUserPayload::new(&user))
        ).into_response()
    }
}
