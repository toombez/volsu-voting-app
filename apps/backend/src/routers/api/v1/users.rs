use axum::{response::Response, routing, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/register", routing::post(register))
        .route("/", routing::patch(update).get(users))
        .route("/me", routing::get(me))
        .route("/{id}", routing::get(user_by_id))
}

async fn register() -> Response {
    Response::new("create user".into())
}

async fn me() -> Response {
    Response::new("me".into())
}

async fn update() -> Response {
    Response::new("update".into())
}

async fn user_by_id() -> Response {
    Response::new("user by id".into())
}

async fn users() -> Response {
    Response::new("users".into())
}
