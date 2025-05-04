use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use uuid::Uuid;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};

use entity::user::Entity as UserEntity;
use sea_orm::entity::*;

pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let user = UserEntity::find_by_id(id)
        .one(&state.connection)
        .await;

    let user = match user {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error")),
        ).into_response(),
        Ok(user) => user,
    };

    let user = match user {
        None => return (
            StatusCode::NOT_FOUND,
            ErrorResponse::new(json!("User not found"))
        ).into_response(),
        Some(user) => user,
    };

    return (
        StatusCode::OK,
        SuccessResponse::new(json!({
            "id": user.id,
            "username": user.username,
        }))
    ).into_response()
}
