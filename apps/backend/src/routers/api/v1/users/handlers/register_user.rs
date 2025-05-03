use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::types::{ErrorResponse, SuccessResponse};

use axum::extract::{Json, State};
use axum::http::StatusCode;
use validator::Validate;

use entity::user::{Entity as UserEntity, ActiveModel as UserActiveModel, Column as UserColumn, Model as UserModel};
use sea_orm::{entity::*, query::*};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Validate)]
pub struct RegisterUserPayload {
    #[validate(length(min = 3, message = "Username must contain at least 3 characters"))]
    username: String,
    #[validate(length(min = 12, message = "Password must contain at least 12 characters"))]
    password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserPayload>,
) -> impl IntoResponse {
    let validation_result = payload
        .validate();

    match validation_result {
        Ok(_) => {},
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                ErrorResponse::new(
                    json!(error.field_errors().clone())
                ),
            ).into_response()
        }
    };

    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(payload.username.clone()))
        .one(&state.connection)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(_error) => {
            println!("{:?}", _error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("Internal server error")
            ).into_response()
        }
    };

    match user {
        None => {},
        Some(_user) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("User already exists")
            ).into_response()
        }
    };

    let user = UserActiveModel {
        password: Set(payload.password.clone().to_string()),
        username: Set(payload.username.clone().to_string()),
        id: Set(Uuid::new_v4()),
        ..Default::default()
    }
        .insert(&state.connection)
        .await;

    let user: UserModel = match user {
        Ok(user) => user,
        Err(_error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("Internal server error"),
            ).into_response()
        }
    };

    (
        StatusCode::CREATED,
        SuccessResponse::new(json!({
            "id": user.id
        }))
    ).into_response()
}
