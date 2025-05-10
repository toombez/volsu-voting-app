use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub id: Uuid,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserProvider {
    pub user: Option<User>
}
