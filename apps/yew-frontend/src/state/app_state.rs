use types::dto::models::UserWithVotings;
use yewdux::Store;

#[derive(Default, PartialEq, Store, Clone)]
pub struct AppState {
    pub auth_user: Option<UserWithVotings>
}

impl AppState {
    pub fn is_logged_in(&self) -> bool {
        self.auth_user.is_some()
    }
}
