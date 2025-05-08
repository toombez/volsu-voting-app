use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;

use crate::{app_state::AppState, repository::user_repository::GetUserQuery, utils::get_internal_error_response};
use entity::user::Model as UserModel;

pub async fn user_vote(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(user): Extension<UserModel>
) -> impl IntoResponse {
    let user = state
        .user_repository
        .get_user(GetUserQuery::from(user.id))
        .await;

    let user = match user {
        Err(_error) => return get_internal_error_response(),
        Ok(user) => user.unwrap(),
    };

    let voting = state
        .voting_repository
        .vote(&user.into(), id)
        .await;

    let voting = match voting {
        Err(_error) => return get_internal_error_response(),
        Ok(voting) => voting,
    };

    (
        StatusCode::CREATED,
        Json(voting)
    ).into_response()
}
