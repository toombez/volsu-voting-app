use gloo::net::http::Request;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_frontend::{client::Client, components::forms::credentials_form::CredentialsForm, state::app_state::AppState};
use yew_router::prelude::*;

use yew_frontend::{layout::header::Header, pages::profile::GetUserResponse, providers::user_provider::User, router::{main_route::MainRoute, main_switch::main_switch}};
use yewdux::Dispatch;

use yew_frontend::providers::client_provider::ClientProvider;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cx = yewdux::Context::new();
        let dispatch = Dispatch::<AppState>::new(&cx);

        let user = dispatch.get().auth_user.clone();

        let client_provider = ClientProvider {
            client: Client::default(),
        };

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
}
