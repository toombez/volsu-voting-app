use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{app_state::AppState, routers::api::v1::auth::utils::encode_jwt, types::{ErrorResponse, SuccessResponse}};

use entity::user::{Entity as UserEntity, Column as UserColumn};
use sea_orm::{entity::*, query::*};
use bcrypt::verify;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUserPayload {
    pub username: String,
    pub password: String,
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserPayload>,
) -> impl IntoResponse {
    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(payload.username.clone()))
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

    let verify_result = verify(payload.password, &user.password);

    match verify_result {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            SuccessResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(is_password_match) => {
            if !is_password_match {
                return (
                    StatusCode::FORBIDDEN,
                    SuccessResponse::new(json!("Bad credentials"))
                ).into_response()
            }
        }
    };

    let token = encode_jwt(user.username, user.password);

    let token = match token {
        Err(_error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(json!("Internal server error"))
        ).into_response(),
        Ok(token) => token
    };

    (
        StatusCode::OK,
        SuccessResponse::new(json!({
            "token": token
        }))
    ).into_response()
}
