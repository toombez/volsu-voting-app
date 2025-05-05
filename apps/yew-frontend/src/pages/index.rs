use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::{html, Component, Html, MouseEvent};

use crate::{components::forms::create_voting_form::CreateVotingForm, types::Voting};


pub enum IndexPageMessage {
    ToggleCreateVotingForm(bool),
    GetVotings(Vec<Voting>),
}

pub struct IndexPage {
    pub votings: Vec<Voting>,
    pub is_create_voting: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVotingsListResponse {
    pub data: Vec<Voting>,
    pub status: String,
}

impl Component for IndexPage {
    type Message = IndexPageMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link().clone();

        wasm_bindgen_futures::spawn_local(async move {
            let response = Request
                ::get("http://localhost:3000/api/v1/votings?page=0&per_page=10")
                .send()
                .await
                .unwrap()
                .json::<GetVotingsListResponse>()
                .await
                .unwrap()
            ;

            link.send_message(IndexPageMessage::GetVotings(response.data));
        });

        Self {
            votings: vec![],
            is_create_voting: false,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexPageMessage::ToggleCreateVotingForm(is_open) => {
                self.is_create_voting = is_open;
            },
            IndexPageMessage::GetVotings(votings) => {
                self.votings = votings;
            }
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_open_create_voting = ctx
            .link()
            .batch_callback(|_event: MouseEvent| {
                Some(IndexPageMessage::ToggleCreateVotingForm(true))
            });

        gloo::console::log!(format!("{:?}", self.votings));

        html! {
            <>
                <button
                    onclick={on_open_create_voting}
                >
                    {"Создать голосование"}
                </button>

                if self.is_create_voting {
                    <CreateVotingForm />
                }

                <ul>
                    { self
                        .votings
                        .iter()
                        .map(|voting| html!(<li>{&*voting.title.clone()}</li>))
                        .collect::<Html>()
                    }
                </ul>
            </>
        }
    }
}
