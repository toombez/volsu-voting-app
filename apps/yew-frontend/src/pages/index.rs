use types::dto::request::PaginationQuery;
use yew::{function_component, html, suspense::use_future, use_context, use_state, Html};

use crate::{components::voting_card::VotingCard, providers::client_provider::ClientProvider, router::main_route::MainRoute};

use yew_router::prelude::*;

#[function_component]
pub fn IndexPage() -> Html {
    let votings = use_state(|| vec![]);
    let client_provider = use_context
        ::<ClientProvider>()
        .expect("Client context not found");
    let client = client_provider.client;

    let _ = use_future({
        let client = client.clone();
        let votings = votings.clone();

        async move || {
            let votings_response = client.get_votings(&PaginationQuery {
                page: 0,
                per_page: 4,
            })
            .await;

            let votings_response = match votings_response {
                Err(()) => return,
                Ok(votings_response) => votings_response,
            };

            votings.set(votings_response.data.items)
        }
    });

    html! {
        <section class="new-votings-section section">
            <h1 class="section__heading">
                {"Новые голосования"}
            </h1>

            <ul class="section__grid grid grid--cols-4">
                {
                    votings
                    .iter()
                    .map(|voting| html!(
                        <li>
                            <VotingCard
                                voting={voting.clone()}
                            />
                        </li>
                    ))
                    .collect::<Html>()
                }
            </ul>

            <div class="new-votings-section__buttons">
                <Link<MainRoute> to={MainRoute::Votings} classes="button">
                    {"Ко всем голосованиям"}
                </Link<MainRoute>>
                <button class="button">
                    {"Создать голосование"}
                </button>
            </div>
        </section>
    }
}
