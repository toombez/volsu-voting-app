use gloo_net::http::Request;
use yew::{html, Component};
use yew_router::prelude::*;

use crate::{components::forms::credentials_form::{CredentialsForm, CredentialsFormSubmitData}, router::main_route::MainRoute};


pub enum RegisterPageMessage {
    TryRegister(CredentialsFormSubmitData),
}

pub struct RegisterPage;


impl Component for RegisterPage {
    type Message = RegisterPageMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let navigator = ctx.link().navigator().unwrap();

        match msg {
            RegisterPageMessage::TryRegister(data) => {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request
                        ::post("http://127.0.0.1:3000/api/v1/users/register")
                        .header("Content-Type", "application/json")
                        .json(&data)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                    ;

                    let status = response.status();

                    if status != 201 {
                        return
                    }

                    navigator.push(&MainRoute::Login);
                });
            }
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_submit = ctx
            .link()
            .batch_callback(|data: CredentialsFormSubmitData| {
                Some(RegisterPageMessage::TryRegister(data))
            });

        html!(
            <div>
                <CredentialsForm
                    on_submit={on_submit}
                />
            </div>
        )
    }
}
