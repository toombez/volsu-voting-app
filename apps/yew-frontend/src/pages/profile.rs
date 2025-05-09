use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use yew::{html, Component};
use yew_router::prelude::*;

use crate::router::main_route::MainRoute;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub data: User,
    pub status: String,
}

#[derive(Debug)]
pub enum ProfilePageMessage {
    FetchUser(User),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfilePage {
    user: Option<User>
}

impl Component for ProfilePage {
    type Message = ProfilePageMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link().clone();

        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap();

        let cookie = document.cookie().unwrap();

        let token = wasm_cookies
            ::cookies
            ::get(&cookie, "token")
            .unwrap()
            .unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let response = Request
                ::get("http://127.0.0.1:3000/api/v1/users/me")
                .header("Authorization", format!("Bearer {}", token).as_str())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json::<GetUserResponse>()
                .await
                .unwrap()
            ;

            link.send_message(ProfilePageMessage::FetchUser(response.data));
        });

        Self {
            user: None,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfilePageMessage::FetchUser(user) => {
                self.user = Some(user);
            }
        };

        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            match &self.user {
                None => html!{
                    <div>
                        <Link<MainRoute> to={MainRoute::Login}>
                            {"Войти"}
                        </Link<MainRoute>>
                        <Link<MainRoute> to={MainRoute::Register}>
                            {"Зарегестрироваться"}
                        </Link<MainRoute>>
                    </div>
                },
                Some(user) => html!{
                    <div>
                        <h1>{&*user.username}</h1>
                    </div>
                }
            }
        }
    }
}
