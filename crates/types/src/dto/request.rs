use serde::{Deserialize, Serialize};
use validator::Validate;

// ============================================================================
// List request query
// ============================================================================

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct PaginationQuery {
    pub page: u64,
    pub per_page: u64,
}

// ============================================================================
// User request body
// ============================================================================

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Validate)]
pub struct CreateUserBody {
    #[validate(length(min = 3, message = "Username must contain at least 3 characters"))]
    pub username: String,
    #[validate(length(min = 12, message = "Password must contain at least 12 characters"))]
    pub password: String,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Validate)]
pub struct PatchUserBody {
    #[validate(length(min = 3, message = "Username must contain at least 3 characters"))]
    pub username: Option<String>,
    #[validate(length(min = 12, message = "Password must contain at least 12 characters"))]
    pub password: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Validate)]
pub struct LoginUserBody {
    pub username: String,
    pub password: String
}

// ============================================================================
// Voting request body
// ============================================================================

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Validate)]
pub struct CreateVotingBody {
    #[validate(length(min = 1, message = "Voting title must contain at least 1 characters"))]
    pub title: String,
    #[validate(length(min = 1, message = "Voting text must contain at least 1 characters"))]
    pub text: String,
}
