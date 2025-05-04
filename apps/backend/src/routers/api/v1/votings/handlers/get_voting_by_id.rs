use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use uuid::Uuid;

use entity::voting::Entity as VotingEntity;
use entity::user_vote::{Entity as UserVoteEntity, Column as UserVoteColumn};
use entity::user::Entity as UserEntity;
use sea_orm::{entity::*, PaginatorTrait, QueryFilter};

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};
use crate::types::{PublicVoting, PublicVotingAuthor};

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

    let author = UserEntity::find_by_id(voting.author_id)
        .one(&state.connection)
        .await;

    let author = match author {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(author) => author,
    };

    let author = match author {
        None => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Some(author) => author,
    };

    let votes_count = UserVoteEntity::find()
        .filter(UserVoteColumn::VotingId.eq(voting.id))
        .count(&state.connection)
        .await;

    let votes_count = match votes_count {
        Err(_error) =>  return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(votes_count) => votes_count,
    };

    (
        StatusCode::OK,
        SuccessResponse::new(json!(PublicVoting {
            created_at: voting.created_at,
            id: voting.id,
            text: voting.text,
            title: voting.title,
            votes_count: votes_count as usize,
            author: PublicVotingAuthor {
                id: author.id,
                username: author.username,
            }
        }))
    ).into_response()
}
