use types::dto::request::LoginUserBody;
use yew::{function_component, html, use_context, Callback, Html};
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::{components::forms::credentials_form::{CredentialsForm, CredentialsFormSubmitData}, providers::client_provider::ClientProvider, router::main_route::MainRoute, state::app_state::AppState, utils::store_token};

#[function_component]
pub fn LoginPage() -> Html {
    let client_provider = use_context
        ::<ClientProvider>()
        .expect("Client context not found");

    let client = client_provider.client;

    let (_, dispatch) = use_store::<AppState>();

    let navigator = use_navigator().unwrap();

    let on_submit = {
        let client = client.clone();
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();

        Callback
            ::from(move |data: CredentialsFormSubmitData| {
                let client = client.clone();
                let dispatch = dispatch.clone();
                let navigator = navigator.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = client
                        .login(&LoginUserBody {
                            password: data.password.clone(),
                            username: data.username.clone(),
                        })
                        .await;

                    let response = match response {
                        Err(_) => return,
                        Ok(response) => response,
                    };

                    let token = response.data.token;

                    store_token(&token);

                    let response = client
                        .me(token)
                        .await;

                    let response = match response {
                        Err(()) => return,
                        Ok(user) => user,
                    };

                    dispatch.reduce_mut(|state| state.auth_user = Some(response.data));

                    navigator.push(&MainRoute::Profile);
                });
            })
    };

    html! {
        <div class="login-section section">
            <CredentialsForm
                on_submit={on_submit}
            />
        </div>
    }
}
