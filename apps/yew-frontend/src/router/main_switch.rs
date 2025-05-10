use yew::{html, Html};

use crate::pages::{index::IndexPage, login::LoginPage, not_found::NotFoundPage, profile::ProfilePage, register::RegisterPage, voting::VotingPage, votings::VotingsPage};

use super::main_route::MainRoute;

pub fn main_switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Index => html! { <IndexPage /> },
        MainRoute::Profile => html! { <ProfilePage /> },
        MainRoute::NotFound => html! { <NotFoundPage /> },
        MainRoute::Login => html! { <LoginPage /> },
        MainRoute::Register => html! { <RegisterPage /> },
        MainRoute::Voting { id } => html! { <VotingPage id={id} /> },
        MainRoute::Votings => html! { <VotingsPage /> },
    }
}
