use uuid::Uuid;
use yew::{function_component, html, use_context, use_state, Callback, Html, MouseEvent, Properties};
use yew_router::hooks::use_navigator;

use crate::{providers::client_provider::ClientProvider, router::main_route::MainRoute, utils::get_token};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct VotingPageProps {
    pub id: Uuid,
}

#[function_component]
pub fn VotingPage(props: &VotingPageProps) -> Html {
    let id = props.id.clone();
    let navigator = use_navigator().unwrap();

    let voting = use_state(|| None);

    let ClientProvider {
        client
    } = use_context::<ClientProvider>()
        .unwrap();

    wasm_bindgen_futures::spawn_local({
        let client = client.clone();
        let id = id.clone();
        let navigator = navigator.clone();
        let voting = voting.clone();

        async move {
            let response = client
                .get_voting(id)
                .await;

            let response = match response {
                Err(()) => return navigator.push(&MainRoute::NotFound),
                Ok(response) => response
            };

            voting.set(Some(response.data));
        }
    });

    let on_vote_click = {
        let client = client.clone();
        let id = id.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let token = get_token();

            let token = match token {
                None => return,
                Some(token) => token,
            };

            wasm_bindgen_futures::spawn_local({
                let token = token.clone();
                let id = id.clone();
                let client = client.clone();

                async move {
                    let _ = client
                        .vote(token, id)
                        .await;
                }
            });
        })
    };

    html! {
        match (*voting).clone() {
            None => html!(<div>{"Не найдено"}</div>),
            Some(voting) => html! {
                <section class="section">
                    <h1 class="section__heading">
                        {voting.clone().title}
                    </h1>

                    <h2>
                        {voting.clone().created_at.to_string()}
                    </h2>

                    <h3>
                        {"Голосов: "}{voting.clone().votes_count}
                    </h3>

                    <p>
                        {voting.clone().text}
                    </p>

                    <button onclick={on_vote_click}>
                        {"Проголосовать"}
                    </button>
                </section>
            }
        }
    }
}
