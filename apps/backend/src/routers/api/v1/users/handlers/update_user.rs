use axum::extract::{State, Extension, Json};
use axum::response::IntoResponse;
use axum::http::StatusCode;

use entity::user::Model as UserModel;
use types::dto::{request::PatchUserBody, response::PatchUserPayload};

use crate::{app_state::AppState, utils::get_internal_error_response};

pub async fn update_user(
    Extension(user): Extension<UserModel>,
    State(state): State<AppState>,
    Json(body): Json<PatchUserBody>
) -> impl IntoResponse {
    let user = state
        .user_repository
        .update_user(user.id, &body)
        .await;

    match user {
        Err(_error) => get_internal_error_response(),
        Ok(user) => (
            StatusCode::OK,
            Json(PatchUserPayload::new(&user))
        ).into_response()
    }
}
