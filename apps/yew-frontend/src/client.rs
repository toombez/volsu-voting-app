use gloo::net::http::Request;
use types::dto::request::{CreateUserBody, CreateVotingBody, LoginUserBody, PaginationQuery};
use types::dto::response::{CreateUserPayload, CreateVotingPayload, GetMePayload, GetUserPayload, GetUsersListPayload, GetVotingPayload, GetVotingsListPayload, LoginUserPayload, VotePayload};
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self { base_url: "http://localhost:3000/api/v1".to_string() }
    }
}

impl Client {
    fn join_route(paths: Vec<&str>) -> String {
        paths.join("/")
    }

    pub async fn register(&self, data: &CreateUserBody) -> Result<CreateUserPayload, ()> {
        let response = Request
            ::post(&Self::join_route(vec![&self.base_url, "users", "register"]))
            .json(data)
            .unwrap()
            .send()
            .await
            .unwrap();

        if response.status() != 201 {
            return Err(())
        }

        let response = response
            .json::<CreateUserPayload>()
            .await
            .unwrap();

        Ok(response)
    }

    pub async fn login(&self, data: &LoginUserBody) -> Result<LoginUserPayload, ()> {
        let response = Request
            ::post(&Self::join_route(vec![&self.base_url, "auth", "login"]))
            .json(&data)
            .unwrap()
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            return Err(())
        }

        let response = response
            .json::<LoginUserPayload>()
            .await
            .unwrap();

        Ok(response)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<GetUserPayload, ()> {
        Err(())
    }

    pub async fn get_users(&self, pagination: &PaginationQuery) -> Result<GetUsersListPayload, ()> {
        Err(())
    }

    pub async fn me(&self) -> Result<GetMePayload, ()> {
        Err(())
    }

    pub async fn create_voting(&self, data: &CreateVotingBody) -> Result<CreateVotingPayload, ()> {
        Err(())
    }

    pub async fn vote(&self, id: Uuid) -> Result<VotePayload, ()> {
        Err(())
    }

    pub async fn get_votings(&self, pagination: &PaginationQuery) -> Result<GetVotingsListPayload, ()> {
        let response = Request
            ::get(&Self::join_route(vec![&self.base_url, "votings"]))
            .query([
                ("page", pagination.page.to_string().as_str()),
                ("per_page", pagination.per_page.to_string().as_str())
            ])
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            return Err(())
        }

        let response = response
            .json::<GetVotingsListPayload>()
            .await
            .unwrap();

        Ok(response)
    }

    pub async fn get_voting(&self, id: Uuid) -> Result<GetVotingPayload, ()> {
        Err(())
    }
}
