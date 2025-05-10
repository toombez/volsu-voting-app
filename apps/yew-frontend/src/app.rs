use std::ops::DerefMut;

use yew::prelude::*;
use yew_frontend::{client::Client, state::app_state::AppState, utils::get_token};
use yew_router::prelude::*;

use yew_frontend::{layout::header::Header, router::{main_route::MainRoute, main_switch::main_switch}};

use yew_frontend::providers::client_provider::ClientProvider;
use yewdux::use_store;

#[function_component]
pub fn App() -> Html {
    let client = Client::default();

    let client_provider = ClientProvider {
        client: client.clone(),
    };

    let (_, dispatch) = use_store::<AppState>();

    use_effect({
        let client = client.clone();
        let dispatch = dispatch.clone();

        move || {
            let token = get_token();

            match token {
                Some(token) => {
                    wasm_bindgen_futures::spawn_local({
                        let client = client.clone();
                        let dispatch = dispatch.clone();

                        async move {
                            let response = client
                                .me(token)
                                .await;

                            let response = match response {
                                Err(()) => return,
                                Ok(user) => user,
                            };

                            dispatch.reduce_mut(|state| state.auth_user = Some(response.data));
                        }
                    });

                    || {}
                },
                None => || {},
            };
        }
    });

    html! {
        <>
            <ContextProvider<ClientProvider> context={client_provider.clone()}>
                <BrowserRouter>
                    <Header />

                    <main>
                        <Switch<MainRoute> render={main_switch}  />
                    </main>
                </BrowserRouter>
            </ContextProvider<ClientProvider>>
        </>
    }
}
