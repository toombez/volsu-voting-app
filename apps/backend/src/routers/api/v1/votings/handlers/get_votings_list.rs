use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use entity::voting::Entity as VotingEntity;
use sea_orm::{entity::*, PaginatorTrait};
use serde_json::json;
use uuid::Uuid;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicVoting {
    id: Uuid,
    title: String,
    text: String,
    author_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

pub async fn get_votings_list(
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let votings = VotingEntity::find()
        .paginate(&state.connection, pagination.per_page as u64)
        .fetch_page(pagination.page as u64)
        .await;

    let votings= match votings {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(votings) => votings,
    };

    let votings: Vec<PublicVoting> = votings
        .iter()
        .map(|voting| PublicVoting {
            author_id: voting.author_id,
            id: voting.id,
            text: voting.text.clone(),
            title: voting.title.clone(),
        })
        .collect();

    (
        StatusCode::OK,
        SuccessResponse::new(json!(votings))
    ).into_response()
}
