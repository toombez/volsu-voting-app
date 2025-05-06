use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api_response::SuccessApiResponse;

// ============================================================================
// Shared types
// ============================================================================

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct Pagination {
    pub last_page: usize,
    pub page: usize,
    pub per_page: usize,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct List<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

// ============================================================================
// Base model types
// ============================================================================

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
pub struct VotingWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub votes_count: usize,
    pub author_id: Uuid,
    pub author: User,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub status: String,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct UserWithVotings {
    pub id: Uuid,
    pub username: String,
    pub status: String,
    pub votings: Vec<Voting>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct UsersList {
    pub users: Vec<User>,
    pub pagination: Pagination,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct Me {
    pub user: User,
    pub token: String,
}

// ============================================================================
// Request body types
// ============================================================================

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct CreateUserRequestBody {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct PatchUserRequestBody {
    pub username: Option<String>,
    pub password: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct CreateVotingRequestBody {
    pub title: String,
    pub text: String,
}

// ============================================================================
// Response payload types
// ============================================================================

pub type GetUserResponsePayload = SuccessApiResponse<UserWithVotings>;
pub type GetUsersListResponsePayload = SuccessApiResponse<List<User>>;
pub type GetMeResponsePayload = SuccessApiResponse<Me>;
pub type CreateUserResponsePayload = SuccessApiResponse<User>;
pub type PatchUserResponsePayload = SuccessApiResponse<User>;

pub type GetVotingResponsePayload = SuccessApiResponse<VotingWithAuthor>;
pub type GetVotingsListResponsePayload = SuccessApiResponse<List<VotingWithAuthor>>;
pub type CreateVotingResponsePayload = SuccessApiResponse<Voting>;
