use yew::{html, Html};

use crate::pages::{index::IndexPage, not_found::NotFoundPage, profile::ProfilePage};

use super::main_route::MainRoute;

pub fn main_switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Index => html! { <IndexPage /> },
        MainRoute::Profile => html! { <ProfilePage /> },
        MainRoute::NotFound => html! { <NotFoundPage /> },
    }
}
