use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Extension;

use entity::user::Model as UserModel;
use serde_json::json;

use crate::types::SuccessResponse;

pub async fn me(
    Extension(user): Extension<UserModel>,
) -> impl IntoResponse {
    return (
        StatusCode::OK,
        SuccessResponse::new(json!({
            "username": user.username,
            "id": user.id,
        }))
    )
}
