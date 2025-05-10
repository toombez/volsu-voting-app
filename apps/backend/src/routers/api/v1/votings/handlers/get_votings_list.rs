use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use types::dto::{request::PaginationQuery, response::GetVotingsListPayload};

use crate::{app_state::AppState, utils::get_internal_error_response};

pub async fn get_votings_list(
    Query(pagination): Query<PaginationQuery>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let votings = state
        .voting_repository
        .get_votings(pagination)
        .await;

    let votings = match votings {
        Err(_error) => return get_internal_error_response(),
        Ok(votings) => votings,
    };

    (
        StatusCode::OK,
        Json(GetVotingsListPayload::new(&votings))
    ).into_response()
}
