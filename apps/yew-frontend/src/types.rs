use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingAuthor {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub author: VotingAuthor,
    pub votes_count: usize,
}
