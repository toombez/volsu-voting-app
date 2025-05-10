use types::dto::models::Voting;
use yew::{function_component, html, use_context, Callback, Classes, Html, Properties, classes};

use crate::{components::voting_card::VotingCard, providers::client_provider::ClientProvider, utils::get_token};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct VotingsGridProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub votings: Vec<Voting>,
}

#[function_component]
pub fn VotingsGrid(props: &VotingsGridProps) -> Html {
    let votings = &props.votings.clone();
    let class = &props.class.clone();

    let ClientProvider {
        client
    } = use_context
        ::<ClientProvider>()
        .expect("Client provider not found");

    let on_vote_voting = {
        let client = client.clone();

        Callback::from(move |id| {
            let client = client.clone();
            let token = get_token();

            let token: String = match token {
                None => return,
                Some(token) => token,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let response = client
                    .vote(token, id)
                    .await;

                let response = match response {
                    Err(()) => return,
                    Ok(response) => response,
                };

                gloo::console::log!(format!("{:?}", response));
            });
        })
    };

    html! {
        <ul class={classes!(class.clone(), "grid grid--cols-4".to_string())}>
            {
                votings
                .iter()
                .map(|voting| html!(
                    <li key={&*voting.id.clone().to_string()}>
                        <VotingCard
                            voting={voting.clone()}
                            on_vote={on_vote_voting.clone()}
                        />
                    </li>
                ))
                .collect::<Html>()
            }
        </ul>
    }
}
