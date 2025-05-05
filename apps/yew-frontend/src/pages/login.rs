use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew::{html, Component};
use yew_router::prelude::RouterScopeExt;

use crate::{components::forms::credentials_form::{CredentialsForm, CredentialsFormSubmitData}, router::main_route::MainRoute};

pub enum LoginPageMessage {
    TryLogin(CredentialsFormSubmitData),
}
pub struct LoginPage;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginApiResponseData {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginApiResponse {
    pub data: LoginApiResponseData
}

impl Component for LoginPage {
    type Message = LoginPageMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let navigator = ctx.link().navigator().unwrap();

        match msg {
            LoginPageMessage::TryLogin(data) => {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request
                        ::post("http://127.0.0.1:3000/api/v1/auth/login")
                        .header("Content-Type", "application/json")
                        .json(&data)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                    ;

                    let status = response.status();

                    if status != 200 {
                        return
                    }

                    let response = response
                        .json::<LoginApiResponse>()
                        .await
                        .unwrap()
                    ;

                    let token = response.data.token;
                    let token_cookie = wasm_cookies::cookies::set(
                        "token",
                        &token,
                        &wasm_cookies::CookieOptions::default()
                    );

                    let document = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlDocument>()
                        .unwrap();

                    let _ = document.set_cookie(&token_cookie);

                    navigator.push(&MainRoute::Profile);
                });
            }
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_submit = ctx
            .link()
            .batch_callback(|data: CredentialsFormSubmitData| {
                Some(LoginPageMessage::TryLogin(data))
            });

        html! {
            <div>
                <CredentialsForm
                    on_submit={on_submit}
                />
            </div>
        }
    }
}
