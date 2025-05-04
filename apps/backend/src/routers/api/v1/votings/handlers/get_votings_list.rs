use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;

use entity::voting::{Entity as VotingEntity, Column as VotingColumn, Relation as VotingRelation};
use entity::user_vote::Column as UserVoteColumn;
use entity::user::Column as UserColumn;

use sea_orm::{entity::*, FromQueryResult, PaginatorTrait, QuerySelect};
use serde_json::json;
use uuid::Uuid;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};

use crate::types::{PublicVoting, PublicVotingAuthor};

#[derive(Debug, FromQueryResult)]
struct TempVoting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub author_id: Uuid,
    pub author_username: String,
    pub votes_count: i64,
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
        .column_as(VotingColumn::Id, "id")
        .column_as(VotingColumn::Title, "title")
        .column_as(VotingColumn::Text, "text")
        .column_as(VotingColumn::CreatedAt, "created_at")
        .column_as(UserColumn::Id, "author_id")
        .column_as(UserColumn::Username, "author_username")
        .column_as(UserVoteColumn::Id.count(), "votes_count")

        .join(sea_orm::JoinType::Join, VotingRelation::UserVote.def())
        .join(sea_orm::JoinType::Join, VotingRelation::User.def())

        .group_by(VotingColumn::Id)

        .into_model::<TempVoting>()
        .paginate(&state.connection, pagination.per_page as u64)
        .fetch_page(pagination.page as u64)
        .await
        .map(|votings| votings
            .iter()
            .map(|voting| PublicVoting {
                author: PublicVotingAuthor {
                    id: voting.author_id,
                    username: voting.author_username.clone(),
                },
                id: voting.id,
                created_at: voting.created_at,
                text: voting.text.clone(),
                title: voting.title.clone(),
                votes_count: voting.votes_count as usize,
            })
            .collect::<Vec<PublicVoting>>()
        )
    ;

    let votings= match votings {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(votings) => votings,
    };

    (
        StatusCode::OK,
        SuccessResponse::new(json!(votings))
    ).into_response()
}
