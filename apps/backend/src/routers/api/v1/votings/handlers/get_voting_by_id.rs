use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use entity::voting::Entity as VotingEntity;
use sea_orm::entity::*;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicVoting {
    id: Uuid,
    title: String,
    text: String,
    author_id: Uuid
}

pub async fn get_voting_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let voting = VotingEntity::find_by_id(id)
        .one(&state.connection)
        .await;

    let voting = match voting {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(voting) => voting,
    };

    let voting = match voting {
        None => return (
            StatusCode::NOT_FOUND,
            ErrorResponse::new(json!("Voting not fount"))
        ).into_response(),
        Some(voting) => voting,
    };

    (
        StatusCode::OK,
        SuccessResponse::new(json!(PublicVoting {
            author_id: voting.author_id,
            id: voting.id,
            text: voting.text,
            title: voting.title,
        }))
    ).into_response()
}
