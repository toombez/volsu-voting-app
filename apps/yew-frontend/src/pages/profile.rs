use yew::{function_component, html, Html};
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{components::votings_grid::VotingsGrid, router::main_route::MainRoute, state::app_state::AppState};

#[function_component]
pub fn ProfilePage() -> Html {
    let (state, _) = use_store::<AppState>();

    let user = &state.auth_user;

    html! {
        match user {
            None => html!{<Redirect<MainRoute> to={MainRoute::Login} />},
            Some(user) => html!{
                <section class="section">
                    <h1 class="section__heading">{&*user.username}</h1>
                    {
                        match &user.status {
                            None => html!(""),
                            Some(status) => html!(status)
                        }
                    }

                    <VotingsGrid votings={user.votings.clone()} />
                </section>
            }
        }
    }
}
