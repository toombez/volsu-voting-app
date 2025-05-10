use axum::{body::Body, extract::{Request, State}, http::{self, Response, StatusCode}, middleware::Next, response::IntoResponse, Json};
use types::api_response::{ErrorApiResponse, ErrorApiResponseData};

use crate::app_state::AppState;
use entity::user::{Entity as UserEntity, Column as UserColumn};
use sea_orm::{entity::*, query::*};

use super::super::auth::utils::decode_jwt;

pub async fn auth_only_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next
) -> Result<Response<Body>, impl IntoResponse> {
    let auth_header = req
        .headers_mut()
        .get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| Json(ErrorApiResponse::from(ErrorApiResponseData::new(
                "EMPTY_HEADER",
                Some("Empty header is not allowed".to_string()),
            )))),
        None => return Err(Json(ErrorApiResponse::from(ErrorApiResponseData::new(
            "EMPTY_AUTHORIZATION_HEADER",
            Some("Please add the JWT token to the header".to_string())
        )))),
    };

    let mut header = auth_header?.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => return Err(Json(ErrorApiResponse::from(ErrorApiResponseData::new(
            "UNABLE_TO_DECODE_TOKEN",
            Some("Unable to decode token".to_string())
        )))),
    };

    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(&token_data.claims.username))
        .one(&state.connection)
        .await;

    let user = match user {
        Err(_error) => return Err(Json(ErrorApiResponse::from(ErrorApiResponseData::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            Some("Internal server error".to_string()),
        )))),
        Ok(user) => user,
    };

    let user = match user {
        None => return Err(Json(ErrorApiResponse::from(ErrorApiResponseData::new(
            StatusCode::UNAUTHORIZED.as_str(),
            Some("You are not an authorized user".to_string()),
        )))),
        Some(user) => user,
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)

}
