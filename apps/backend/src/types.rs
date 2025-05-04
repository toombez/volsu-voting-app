use axum::{body::Body, http::Response, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

// ============================================================================
// API Responses
// ============================================================================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SuccessResponse<D: Clone + Serialize> {
    status: String,
    pub data: D,
}

impl <D: Clone + Serialize> SuccessResponse<D> {
    pub fn new(data: D) -> Self {
        Self {
            status: "Success".to_string(),
            data,
        }
    }
}

impl <D: Clone + Serialize> IntoResponse for SuccessResponse<D> {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::from(json!(self).to_string()))
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ErrorResponse<E: Clone + Serialize> {
    status: String,
    pub errors: E,
}

impl <E: Clone + Serialize> ErrorResponse<E> {
    pub fn new(errors: E) -> Self {
        Self {
            errors,
            status: "error".to_string(),
        }
    }
}

impl <E: Clone + Serialize> IntoResponse for ErrorResponse<E> {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::from(json!(self).to_string()))
    }
}

// ============================================================================
// Types for public responses
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUserVoting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub votes_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: Uuid,
    pub votings: Vec<PublicUserVoting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicVotingAuthor {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicVoting {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub author: PublicVotingAuthor,
    pub votes_count: usize,
}
