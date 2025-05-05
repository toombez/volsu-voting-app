use yew::prelude::*;
use yew_router::prelude::*;

use yew_frontend::{layout::header::Header, router::{main_route::MainRoute, main_switch::main_switch}};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />

            <main>
                <Switch<MainRoute> render={main_switch} />
            </main>
        </BrowserRouter>
    }
}
