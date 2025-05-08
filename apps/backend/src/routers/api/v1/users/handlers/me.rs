use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::{Extension, State, Json};

use types::dto::response::GetMePayload;

use entity::user::Model as UserModel;

use crate::app_state::AppState;
use crate::repository::user_repository::GetUserQuery;
use crate::utils::get_internal_error_response;

pub async fn me(
    State(state): State<AppState>,
    Extension(user): Extension<UserModel>,
) -> impl IntoResponse {
    let user = state
        .user_repository
        .get_user(GetUserQuery::from(user.id))
        .await;

    match user {
        Err(_error) => get_internal_error_response(),
        Ok(user) => match user {
            None => get_internal_error_response(),
            Some(user) => (
                StatusCode::OK,
                Json(GetMePayload::new(&user))
            ).into_response()
        }
    }
}
