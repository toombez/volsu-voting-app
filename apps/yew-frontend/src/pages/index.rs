use serde::{Deserialize, Serialize};
use types::dto::{models::Voting, request::PaginationQuery};
use yew::{html, use_context, Callback, Component, Html, MouseEvent};

use crate::{components::{forms::create_voting_form::{CreateVotingForm, CreateVotingFormSubmitData}, voting_card::VotingCard}, providers::client_provider::ClientProvider, router::main_route::MainRoute};

use yew_router::prelude::*;

pub enum IndexPageMessage {
    GetVotings(Vec<Voting>),
}

pub struct IndexPage {
    pub votings: Vec<Voting>
}

impl Component for IndexPage {
    type Message = IndexPageMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link().clone();

        let (client_provider, _) = ctx
            .link()
            .context::<ClientProvider>(Callback::noop())
            .expect("Client provider not found");

        wasm_bindgen_futures::spawn_local(async move {
            let votings = client_provider
                .client
                .get_votings(&PaginationQuery {
                    page: 0,
                    per_page: 4,
                })
                .await
                .unwrap();

            link.send_message(IndexPageMessage::GetVotings(votings.data.items));
        });

        Self {
            votings: vec![],
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexPageMessage::GetVotings(votings) => {
                self.votings = votings;
            },
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <section class="new-votings-section section">
                <h1 class="section__heading">
                    {"Новые голосования"}
                </h1>

                <ul class="section__grid grid grid--cols-4">
                    { self
                        .votings
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
}
