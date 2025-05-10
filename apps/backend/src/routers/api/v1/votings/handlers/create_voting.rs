use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Extension, Json};

use entity::user::Model as UserModel;

use types::dto::request::CreateVotingBody;
use types::dto::response::CreateVotingPayload;

use crate::app_state::AppState;
use crate::repository::user_repository::GetUserQuery;
use crate::utils::get_internal_error_response;

pub async fn create_voting(
    State(state): State<AppState>,
    Extension(user): Extension<UserModel>,
    Json(body): Json<CreateVotingBody>,
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
        .create_voting(&user.into(), &body)
        .await;

    let voting = match voting {
        Err(_error) => return get_internal_error_response(),
        Ok(voting) => voting,
    };

    (
        StatusCode::CREATED,
        Json(CreateVotingPayload::new(&voting.into()))
    ).into_response()
}
