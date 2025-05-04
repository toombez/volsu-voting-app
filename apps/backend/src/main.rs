use axum::Router;
use backend::{app_state::AppState, routers};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    let connection = sea_orm
        ::Database
        ::connect("sqlite://./voting_app.sqlite?mode=rwc")
        .await
        .unwrap();

    let _ = Migrator::up(&connection, None).await;

    let app_state = AppState { connection };

    let app = Router::new()
        .merge(routers::create_router(app_state.clone()))
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
