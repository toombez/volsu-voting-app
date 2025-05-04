use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};
use entity::user::Entity as UserEntity;
use sea_orm::{entity::*, query::*};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
}

pub async fn get_users_list(
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let users = UserEntity::find()
        .paginate(&state.connection, pagination.per_page as u64)
        .fetch_page(pagination.page as u64)
        .await;

    let users = match users {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(users) => users,
    };

    let users: Vec<PublicUser> = users
        .iter()
        .map(|user| PublicUser {
            id: user.id.into(),
            username: user.username.clone(),
        })
        .collect();

    (
        StatusCode::OK,
        SuccessResponse::new(json!(users))
    ).into_response()
}
