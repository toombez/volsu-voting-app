use yew::{html, Component, Properties};

use crate::types::Voting;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct VotingCardProps {
    pub voting: Voting
}

pub struct VotingCard {
    voting: Voting
}

impl Component for VotingCard {
    type Message = ();
    type Properties = VotingCardProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let voting = &ctx.props().voting;

        Self { voting: voting.clone() }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let id = self.voting.id.clone();
        let title = self.voting.title.clone();
        let votes_count = self.voting.votes_count;

        let author_username = self.voting.author.username.clone();

        html! {
            <article id={id.clone().to_string()}>
                <h1>
                    {&*author_username}{":"} {&*title}
                </h1>

                <button>
                    {votes_count} {" Проголосовать \"За\""}
                </button>
            </article>
        }
    }
}
