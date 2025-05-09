use types::dto::request::CreateUserBody;
use yew::{html, Component};
use yew_router::prelude::*;

use crate::{client::Client, components::forms::credentials_form::{CredentialsForm, CredentialsFormSubmitData}, router::main_route::MainRoute};

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
                let client = Client::default();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = client.register(&CreateUserBody {
                        password: data.password.clone(),
                        username: data.username.clone(),
                    }).await;

                    match response {
                        Err(_) => return,
                        Ok(_) => {}
                    };

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
            <div class="section register-section">
                <CredentialsForm
                    on_submit={on_submit}
                    class="register-form"
                />
            </div>
        )
    }
}
