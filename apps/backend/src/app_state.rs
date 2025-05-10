use sea_orm::DbConn;

use crate::repository::{user_repository::UserRepository, voting_repository::VotingRepository};

#[derive(Debug)]
#[derive(Clone)]
pub struct AppState {
    pub connection: DbConn,
    pub user_repository: UserRepository,
    pub voting_repository: VotingRepository,
}
