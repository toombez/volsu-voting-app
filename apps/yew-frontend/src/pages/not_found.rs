use yew::{html, Component};

pub struct NotFoundPage;

impl Component for NotFoundPage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!("Not found")
    }
}
