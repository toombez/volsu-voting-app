use axum::{response::Response, routing, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/login", routing::post(login))
        .route("/access", routing::post(access_token))
        .route("/refresh", routing::post(refresh_token))
}

async fn login() -> Response {
    Response::new("login".into())
}

async fn access_token() -> Response {
    Response::new("access".into())
}

async fn refresh_token() -> Response {
    Response::new("refresh".into())
}
