use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use types::{api_response::{ErrorApiResponse, ErrorApiResponseData}, dto::response::GetVotingPayload};
use uuid::Uuid;

use crate::{app_state::AppState, utils::get_internal_error_response};

pub async fn get_voting_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let voting = state
        .voting_repository
        .get_voting(id)
        .await;

    let voting = match voting {
        Err(_error) => return get_internal_error_response(),
        Ok(voting) => match voting {
            None => return (
                StatusCode::NOT_FOUND,
                Json(ErrorApiResponse::from(ErrorApiResponseData::new(
                    StatusCode::NOT_FOUND.as_str(),
                    Some("Voting not found".to_string())
                )))
            ).into_response(),
            Some(voting) => voting,
        }
    };

    (
        StatusCode::OK,
        Json(GetVotingPayload::new(&voting))
    ).into_response()
}
