use serde::{Deserialize, Serialize};
use crate::api_response::SuccessApiResponse;
use crate::dto::models::{User, UserWithVotings, Voting, VotingWithAuthor};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct ListPagination {
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
    pub pagination: ListPagination,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct LoginUserPayloadData {
    pub user: User,
    pub token: String,
}

pub type GetUserPayload = SuccessApiResponse<UserWithVotings>;
pub type GetUsersListPayload = SuccessApiResponse<List<User>>;
pub type GetMePayload = SuccessApiResponse<UserWithVotings>;
pub type CreateUserPayload = SuccessApiResponse<User>;
pub type PatchUserPayload = SuccessApiResponse<User>;
pub type LoginUserPayload = SuccessApiResponse<LoginUserPayloadData>;

pub type GetVotingPayload = SuccessApiResponse<VotingWithAuthor>;
pub type GetVotingsListPayload = SuccessApiResponse<List<Voting>>;
pub type CreateVotingPayload = SuccessApiResponse<Voting>;

pub type VotePayload = SuccessApiResponse<VotingWithAuthor>;
