use axum::extract::{Json, State};
use axum::response::IntoResponse;
use axum::http::StatusCode;

use bcrypt::verify;
use sea_orm::{EntityTrait, QueryFilter};
use types::api_response::{ErrorApiResponse, ErrorApiResponseData};
use types::dto::response::{LoginUserPayload, LoginUserPayloadData};
use types::dto::models::User;
use types::dto::request::LoginUserBody;

use crate::app_state::AppState;
use crate::repository::user_repository::GetUserQuery;
use crate::utils::get_internal_error_response;
use crate::routers::api::v1::auth::utils::encode_jwt;

use entity::user::{Entity as UserEntity, Column as UserColumn};
use sea_orm::entity::*;

pub async fn login_user(
    State(state): State<AppState>,
    Json(body): Json<LoginUserBody>,
) -> impl IntoResponse {
    let user = UserEntity
        ::find()
        .filter(UserColumn::Username.eq(body.username))
        .one(&state.connection)
        .await;

    let user = match user {
        Err(_error) => return get_internal_error_response(),
        Ok(user) => match user {
            None => return (
                StatusCode::NOT_FOUND,
                Json(ErrorApiResponse::from(ErrorApiResponseData::new(
                    StatusCode::NOT_FOUND.as_str(),
                    Some("User not found".to_string())
                )))
            ).into_response(),
            Some(user) => user,
        }
    };

    let verify_result = verify(
        body.password,
        &user.password
    );

    match verify_result {
        Err(_error) => return get_internal_error_response(),
        Ok(is_password_match) => if !is_password_match {
            return (
                StatusCode::FORBIDDEN,
                Json(ErrorApiResponse::from(ErrorApiResponseData::new(
                    StatusCode::FORBIDDEN.as_str(),
                    None
                )))
            ).into_response()
        }
    };

    let token = encode_jwt(user.username.clone(), user.password.clone());

    let token = match token {
        Err(_error) => return get_internal_error_response(),
        Ok(token) => token,
    };

    let user = state
        .user_repository
        .get_user(GetUserQuery::from(user.username.clone()))
        .await;

    let user = match user {
        Err(_error) => return get_internal_error_response(),
        Ok(user) => user.unwrap(),
    };

    (
        StatusCode::OK,
        Json(LoginUserPayload::new(&LoginUserPayloadData {
            token,
            user: User {
                id: user.id,
                status: user.status.clone(),
                username: user.username.clone(),
                votings_count: user.votings_count,
            },
        }))
    ).into_response()
}
