use yew::{html, Component};

pub struct ProfilePage;

impl Component for ProfilePage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!("profile page")
    }
}
