use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Extension, Json};
use entity::user::Model as UserModel;
use entity::voting::ActiveModel as VotingActiveModel;
use sea_orm::entity::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::types::{ErrorResponse, SuccessResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVotingPayload {
    text: String,
    title: String,
}

pub async fn create_voting(
    State(state): State<AppState>,
    Extension(user): Extension<UserModel>,
    Json(payload): Json<CreateVotingPayload>,
) -> impl IntoResponse {
    let voting = VotingActiveModel {
        id: Set(Uuid::new_v4()),
        author_id: Set(user.id),
        text: Set(payload.text),
        title: Set(payload.title),
        ..Default::default()
    }
        .insert(&state.connection)
        .await;

    let voting = match voting {
        Err(_error) => {
            println!("{:?}", _error);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(json!("Internal server error"))
            ).into_response()
        },
        Ok(voting) => voting,
    };

    (
        StatusCode::CREATED,
        SuccessResponse::new(json!({
            "id": voting.id,
        }))
    ).into_response()
}
