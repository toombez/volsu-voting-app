use axum::{body::Body, extract::{Request, State}, http::{self, Response}, middleware::Next};
use serde_json::{json, Value};

use crate::{app_state::AppState, types::ErrorResponse};
use entity::user::{Entity as UserEntity, Column as UserColumn};
use sea_orm::{entity::*, query::*};

use super::super::auth::utils::decode_jwt;

pub async fn auth_only_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next
) -> Result<Response<Body>, ErrorResponse<Value>> {
    let auth_header = req
        .headers_mut()
        .get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| ErrorResponse::new(json!("Empty header is not allowed"))),
        None => return Err(ErrorResponse::new(json!("Please add the JWT token to the header"))),
    };

    let mut header = auth_header?.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => return Err(ErrorResponse::new(json!("Unable to decode token"))),
    };

    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(&token_data.claims.username))
        .one(&state.connection)
        .await;

    let user = match user {
        Err(_error) => return Err(ErrorResponse::new(json!("Internal server error"))),
        Ok(user) => user,
    };

    let user = match user {
        None => return Err(ErrorResponse::new(json!("You are not an authorized user"))),
        Some(user) => user,
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)

}
