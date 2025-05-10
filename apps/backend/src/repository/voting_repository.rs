use chrono::NaiveDateTime;
use sea_orm::{DbConn, DbErr, EntityTrait, FromQueryResult, QueryFilter};
use types::dto::models::{User, Voting, VotingWithAuthor};
use types::dto::request::{CreateVotingBody, PaginationQuery};
use types::dto::response::{List, ListPagination};
use uuid::Uuid;

use entity::user::Entity as UserEntity;
use entity::voting::{Entity as VotingEntity, Column as VotingColumn, ActiveModel as VotingActiveModel};
use entity::user_vote::{Entity as UserVoteEntity, Column as UserVoteColumn, ActiveModel as UserVoteActiveModel};

use sea_orm::{query::*, entity::*};

#[derive(Debug, Clone)]
pub struct VotingRepository {
    db: DbConn,
}

impl From<DbConn> for VotingRepository {
    fn from(value: DbConn) -> Self {
        Self { db: value }
    }
}

#[derive(Debug, FromQueryResult)]
struct TempVoting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub votes_count: u32,
    pub author_id: Uuid,
    // pub author: User,
}

impl VotingRepository {
    pub async fn get_voting(&self, id: Uuid) -> Result<Option<VotingWithAuthor>, DbErr> {
        let voting = VotingEntity
            ::find_by_id(id)
            .column_as(UserVoteColumn::Id.count(), "votes_count")
            .left_join(UserVoteEntity)
            .group_by(VotingColumn::Id)
            .into_model::<TempVoting>()
            .one(&self.db)
            .await;

        let voting = match voting {
            Err(error) => return Err(error),
            Ok(voting) => match voting {
                None => return Ok(None),
                Some(voting) => voting,
            }
        };

        let user = UserEntity
            ::find_by_id(voting.author_id)
            .one(&self.db)
            .await;

        let user = match user {
            Err(error) => return Err(error),
            Ok(user) => user.unwrap(),
        };

        let author_votings_count = VotingEntity
            ::find()
            .filter(VotingColumn::AuthorId.eq(user.id))
            .count(&self.db)
            .await;

        let author_votings_count = match author_votings_count {
            Err(error) => return Err(error),
            Ok(author_votings_count) => author_votings_count,
        };

        Ok(Some(VotingWithAuthor {
            author: User {
                id: user.id,
                status: user.status.clone(),
                username: user.username.clone(),
                votings_count: author_votings_count,
            },
            author_id: user.id,
            created_at: voting.created_at,
            id: voting.id,
            text: voting.text.clone(),
            title: voting.title.clone(),
            votes_count: voting.votes_count as usize,
        }))
    }

    pub async fn get_votings(&self, pagination: PaginationQuery) -> Result<List<Voting>, DbErr> {
        let votings_count = VotingEntity
            ::find()
            .count(&self.db)
            .await;

        let votings_count = match votings_count {
            Err(error) => return Err(error),
            Ok(votings_count) => votings_count,
        };

        let last_page = votings_count / pagination.per_page;

        let votings = VotingEntity
            ::find()
            .column_as(UserVoteColumn::Id.count(), "votes_count")
            .left_join(UserVoteEntity)
            .group_by(VotingColumn::Id)
            .order_by_desc(VotingColumn::CreatedAt)
            .into_model::<TempVoting>()
            .paginate(&self.db, pagination.per_page)
            .fetch_page(pagination.page)
            .await
            .map(|votings| votings
                .iter()
                .map(|voting| Voting {
                    author_id: voting.author_id,
                    created_at: voting.created_at,
                    id: voting.id,
                    text: voting.text.clone(),
                    title: voting.title.clone(),
                    votes_count: voting.votes_count as usize,
                })
                .collect()
            );

        let votings: Vec<Voting> = match votings {
            Err(error) => return Err(error),
            Ok(votings) => votings,
        };

        Ok(List {
            items: votings,
            pagination: ListPagination {
                last_page: last_page as usize,
                page: pagination.page as usize,
                per_page: pagination.per_page as usize,
            }
        })
    }

    pub async fn create_voting(&self, user: &User, data: &CreateVotingBody) -> Result<VotingWithAuthor, DbErr> {
        let voting = VotingActiveModel {
            author_id: Set(user.id),
            text: Set(data.text.clone()),
            title: Set(data.title.clone()),
            id: Set(Uuid::new_v4()),
            ..Default::default()
        };

        let voting = voting
            .insert(&self.db)
            .await;

        let voting = match voting {
            Err(error) => return Err(error),
            Ok(voting) => voting,
        };

        let voting = self
            .get_voting(voting.id)
            .await;

        let voting = match voting {
            Err(error) => return Err(error),
            Ok(voting) => voting.unwrap(),
        };

        Ok(voting)
    }

    pub async fn vote(&self, user: &User, id: Uuid) -> Result<VotingWithAuthor, DbErr> {
        let user_vote = UserVoteActiveModel {
            user_id: Set(user.id),
            voting_id: Set(id),
            ..Default::default()
        };

        let user_vote = user_vote.insert(&self.db).await;

        match user_vote {
            Err(error) => return Err(error),
            Ok(user_vote) => user_vote,
        };

        self
            .get_voting(id)
            .await
            .map(|voting| voting.unwrap())
    }
}
