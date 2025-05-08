use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use types::api_response::{ErrorApiResponse, ErrorApiResponseData};

pub fn get_internal_error_response() -> Response {
    let payload = ErrorApiResponse::from(ErrorApiResponseData
        ::from(StatusCode::INTERNAL_SERVER_ERROR.as_str())
    );

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(payload)
    ).into_response()
}
