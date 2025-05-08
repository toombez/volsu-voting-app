use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use chrono::NaiveDateTime;
use sea_orm::{entity::*, query::*, FromQueryResult};

use sea_orm::{DbConn, DbErr, EntityTrait, QueryFilter};

use types::dto::models::{User, UserWithVotings, Voting};
use types::dto::request::{PatchUserBody, CreateUserBody, PaginationQuery};
use types::dto::response::{List, ListPagination};

use entity::user::{Entity as UserEntity, Column as UserColumn, ActiveModel as UserActiveModel};
use entity::voting::{Entity as VotingEntity, Column as VotingColumn};
use entity::user_vote::{Entity as UserVoteEntity, Column as UserVoteColumn};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db: DbConn
}

impl From<DbConn> for UserRepository {
    fn from(value: DbConn) -> Self {
        Self {
            db: value,
        }
    }
}

pub struct GetUserQuery {
    pub username: Option<String>,
    pub id: Option<Uuid>,
}

impl From<Uuid> for GetUserQuery {
    fn from(value: Uuid) -> Self {
        Self {
            username: None,
            id: Some(value),
        }
    }
}

impl From<String> for GetUserQuery {
    fn from(value: String) -> Self {
        Self {
            id: None,
            username: Some(value),
        }
    }
}

#[derive(Debug, FromQueryResult)]
struct TempListUser {
    pub id: Uuid,
    pub username: String,
    pub status: Option<String>,
    pub votings_count: u32,
}

#[derive(Debug, FromQueryResult)]
struct TempUserVoting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub votes_count: u32,
    pub author_id: Uuid,
}

impl UserRepository {
    pub async fn get_user(&self, query: GetUserQuery) -> Result<Option<UserWithVotings>, DbErr> {
        if let (None, None) = (query.username.clone(), query.id) {
            return Ok(None)
        }

        let sql_query = UserEntity
            ::find();

        let sql_query = match query.username {
            None => sql_query,
            Some(username) => sql_query.filter(UserColumn::Username.eq(username))
        };

        let sql_query = match query.id {
            None => sql_query,
            Some(id) => sql_query.filter(UserColumn::Id.eq(id))
        };

        let user = sql_query
            .one(&self.db)
            .await;

        let user = match user {
            Err(error) => return Err(error),
            Ok(user) => match user {
                None => return Ok(None),
                Some(user) => user,
            }
        };

        let user_votings = VotingEntity
            ::find()
            .column_as(UserVoteColumn::Id.count(), "votes_count")
            .filter(VotingColumn::AuthorId.eq(user.id))
            .left_join(UserVoteEntity)
            .group_by(VotingColumn::Id)
            .into_model::<TempUserVoting>()
            .all(&self.db)
            .await;

        let user_votings: Vec<Voting> = match user_votings {
            Err(error) => return Err(error),
            Ok(user_votings) => user_votings
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
        };

        Ok(Some(UserWithVotings {
            id: user.id,
            status: user.status.clone(),
            username: user.username.clone(),
            votings: user_votings.clone(),
            votings_count: user_votings.len() as u64,
        }))
    }

    pub async fn get_users(&self, pagination: PaginationQuery) -> Result<List<User>, DbErr> {
        let users_count = UserEntity
            ::find()
            .count(&self.db)
            .await;

        let users_count = match users_count {
            Err(error) => return Err(error),
            Ok(users_count) => users_count,
        };

        let last_page = users_count / pagination.per_page;

        UserEntity
            ::find()
            .column_as(VotingColumn::Id.count(), "votings_count")
            .left_join(VotingEntity)
            .group_by(UserColumn::Id)
            .into_model::<TempListUser>()
            .paginate(&self.db, pagination.per_page)
            .fetch_page(pagination.page)
            .await
            .map(|users| {
                let users: Vec<User> = users
                    .iter()
                    .map(|user| User {
                        id: user.id,
                        status: user.status.clone(),
                        username: user.username.clone(),
                        votings_count: user.votings_count as u64,
                    })
                    .collect();

                users
            })
            .map(|users| List {
                items: users,
                pagination: ListPagination {
                    page: pagination.page as usize,
                    per_page: pagination.per_page as usize,
                    last_page: last_page as usize,
                }
            })
    }

    pub async fn create_user(&self, data: &CreateUserBody) -> Result<User, DbErr> {
        let hashed_password = hash(data.password.clone().to_string(), DEFAULT_COST).unwrap();

        let user = UserActiveModel {
            password: Set(hashed_password),
            username: Set(data.username.clone()),
            ..Default::default()
        };

        let user = user
            .insert(&self.db)
            .await;

        let user = match user {
            Err(error) => return Err(error),
            Ok(user) => user,
        };

        let user = User {
            id: user.id,
            status: user.status.clone(),
            username: user.username.clone(),
            votings_count: 0
        };

        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, data: &PatchUserBody) -> Result<User, DbErr> {
        let user = UserEntity
            ::find_by_id(id)
            .one(&self.db)
            .await;

        let user = match user {
            Err(error) => return Err(error),
            Ok(user) => user.unwrap(),
        };

        let mut user: UserActiveModel = user.into();

        match data.password.clone() {
            None => (),
            Some(password) => {
                let hashed_password = hash(
                    password,
                    DEFAULT_COST
                ).unwrap();
                user.password = Set(hashed_password);
            }
        };

        match data.status.clone() {
            None => (),
            Some(status) => {
                user.status = Set(Some(status))
            }
        };

        match data.username.clone() {
            None => (),
            Some(username) => {
                user.username = Set(username)
            }
        };

        let user = user
            .update(&self.db)
            .await;

        let user = match user {
            Err(error) => return Err(error),
            Ok(user) => user,
        };

        self
            .get_user(GetUserQuery::from(user.id))
            .await
            .map(|user| {
                let user = user.unwrap();

                User {
                    id: user.id,
                    status: user.status.clone(),
                    username: user.username.clone(),
                    votings_count: user.votings_count,
                }
            })
    }
}
