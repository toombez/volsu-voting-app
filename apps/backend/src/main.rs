use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use backend::routers;
use serde::{Deserialize, Serialize};

use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    let connection = sea_orm
        ::Database
        ::connect("sqlite://./voting_app.sqlite?mode=rwc")
        .await
        .unwrap();

    let _ = Migrator::up(&connection, None).await;

    let app = Router::new()
        .merge(routers::create_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
