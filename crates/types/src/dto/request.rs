use serde::{Deserialize, Serialize};
#[cfg(feature = "validator")]
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
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct CreateUserBody {
    #[cfg_attr(feature = "validator", validate(length(min = 3, message = "Username must contain at least 3 characters")))]
    pub username: String,
    #[cfg_attr(feature = "validator", validate(length(min = 12, message = "Password must contain at least 12 characters")))]
    pub password: String,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct PatchUserBody {
    #[cfg_attr(feature = "validator", validate(length(min = 3, message = "Username must contain at least 3 characters")))]
    pub username: Option<String>,
    #[cfg_attr(feature = "validator", validate(length(min = 12, message = "Password must contain at least 12 characters")))]
    pub password: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
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
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct CreateVotingBody {
    #[cfg_attr(feature = "validator", validate(length(min = 1, message = "Voting title must contain at least 1 characters")))]
    pub title: String,
    #[cfg_attr(feature = "validator", validate(length(min = 1, message = "Voting text must contain at least 1 characters")))]
    pub text: String,
}
