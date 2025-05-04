use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension};
use sea_orm::{EntityTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use entity::user::Model as UserModel;
use entity::user_vote::{Entity as UserVoteEntity, ActiveModel as UserVoteActiveModel, Column as UserVoteColumn};
use entity::voting::Entity as VotingEntity;
use sea_orm::entity::*;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};

pub async fn user_vote(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(user): Extension<UserModel>
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
            ErrorResponse::new(json!("Voting not found"))
        ).into_response(),
        Some(voting) => voting,
    };

    let vote = UserVoteEntity::find()
        .filter(UserVoteColumn::VotingId
            .eq(voting.id)
            .and(UserVoteColumn::UserId.eq(user.id))
        )
        .one(&state.connection)
        .await;

    let vote = match vote {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(vote) => vote
    };

    match vote {
        Some(_vote) => return (
            StatusCode::BAD_REQUEST,
            ErrorResponse::new(json!("Already voted"))
        ).into_response(),
        None => {},
    };

    let voting = UserVoteActiveModel {
        user_id: Set(user.id),
        voting_id: Set(voting.id),
        ..Default::default()
    }
        .insert(&state.connection)
        .await;

    let voting = match voting {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(voting) => voting,
    };

    (
        StatusCode::CREATED,
        SuccessResponse::new(json!({
            "id": voting.id,
        }))
    ).into_response()
}
