use yew::{html, Component};

pub struct IndexPage;

impl Component for IndexPage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!("Index page")
    }
}
