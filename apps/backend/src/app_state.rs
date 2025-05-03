use sea_orm::DbConn;

#[derive(Debug)]
#[derive(Clone)]
pub struct AppState {
    pub connection: DbConn,
}
