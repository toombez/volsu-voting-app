use types::dto::models::Voting;
use uuid::Uuid;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties};
use yew_router::hooks::use_navigator;

use crate::router::main_route::MainRoute;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct VotingCardProps {
    pub voting: Voting,

    #[prop_or_default]
    pub on_vote: Callback<Uuid>,
}

#[function_component]
pub fn VotingCard(props: &VotingCardProps) -> Html {
    let navigator = use_navigator().unwrap();

    let on_vote_prop = &props.on_vote;
    let voting_prop = props.voting.clone();

    let id = voting_prop.id.clone();
    let title = voting_prop.title.clone();
    let votes_count = voting_prop.votes_count;

    let on_vote_button_click = {
        let on_vote_prop = on_vote_prop.clone();
        let id = id.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            on_vote_prop.emit(id);
        })
    };

    let on_more_click = {
        let navigator = navigator.clone();
        let id = id.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            navigator.push(&MainRoute::Voting { id });
        })
    };

    html! {
        <article
            class="voting"
            id={id.clone().to_string()}
        >
            <h1 class="voting__title">
                {&*title}
            </h1>

            <h2 class="voting__votes-count">
                {"Голосов: "} {votes_count}
            </h2>

            <div class="voting__buttons">
                <button class="voting__more-button button" onclick={on_more_click}>
                    {"Подробнее"}
                </button>

                <button class="voting__vote-button button"
                    onclick={on_vote_button_click}
                >
                    {"Проголосовать"}
                </button>
            </div>
        </article>
    }
}
