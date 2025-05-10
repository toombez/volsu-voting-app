use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{app_state::AppState, types::{ErrorResponse, SuccessResponse}};

use entity::user::{Entity as UserEntity, ActiveModel as UserActiveModel, Column as UserColumn, Model as UserModel};
use sea_orm::{entity::*, query::*};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Validate)]
pub struct UpdateUserPayload {
    #[validate(length(min = 3, message = "Username must contain at least 3 characters"))]
    username: Option<String>,
    #[validate(length(min = 12, message = "Password must contain at least 12 characters"))]
    password: Option<String>,
}

pub async fn update_user(
    Extension(user): Extension<UserModel>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserPayload>
) -> impl IntoResponse {
    let validation_result = payload.validate();

    match validation_result {
        Err(errors) => return (
            StatusCode::BAD_REQUEST,
            ErrorResponse::new(json!(errors)),
        ).into_response(),
        Ok(_) => {}
    };

    let mut user: UserActiveModel = user.into();

    match payload.password {
        None => {},
        Some(password) => {
            user.password = Set(bcrypt::hash(password, DEFAULT_COST).unwrap());
        }
    };

    match payload.username {
        None => {},
        Some(username) => {
            let user_with_username = UserEntity::find()
                .filter(UserColumn::Username.eq(username.clone()))
                .one(&state.connection)
                .await;

            let user_with_username = match user_with_username {
                Err(_error) => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(json!("Internal server error"))
                ).into_response(),
                Ok(user) => user,
            };

            match user_with_username {
                None => {},
                Some(_user) => return (
                    StatusCode::FORBIDDEN,
                    ErrorResponse::new(json!("User with username already exists"))
                ).into_response()
            };

            user.username = Set(username);
        }
    }

    let update_result = user
        .update(&state.connection)
        .await;

    let _user = match update_result {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(user) => user,
    };

    (
        StatusCode::OK,
        SuccessResponse::new(json!("Succesfully updated"))
    ).into_response()
}
