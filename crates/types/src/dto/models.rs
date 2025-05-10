use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub status: Option<String>,
    pub votings_count: u64,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct Voting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub votes_count: usize,
    pub author_id: Uuid,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct UserWithVotings {
    pub id: Uuid,
    pub username: String,
    pub status: Option<String>,
    pub votings: Vec<Voting>,
    pub votings_count: u64,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct VotingWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub votes_count: usize,
    pub author_id: Uuid,
    pub author: User,
}

impl From<UserWithVotings> for User {
    fn from(value: UserWithVotings) -> Self {
        Self {
            id: value.id,
            status: value.status.clone(),
            username: value.username.clone(),
            votings_count: value.votings_count,
        }
    }
}

impl From<VotingWithAuthor> for Voting {
    fn from(value: VotingWithAuthor) -> Self {
        Self {
            author_id: value.author_id,
            created_at: value.created_at,
            id: value.id,
            text: value.text.clone(),
            title: value.title.clone(),
            votes_count: value.votes_count,
        }
    }
}
