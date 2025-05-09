use types::dto::models::Voting;
use yew::{html, Component, Properties};

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
                    <button class="voting__more-button button">
                        {"Подробнее"}
                    </button>

                    <button class="voting__vote-button button">
                        {"Проголосовать"}
                    </button>
                </div>
            </article>
        }
    }
}
