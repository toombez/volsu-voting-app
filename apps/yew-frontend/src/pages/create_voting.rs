use types::dto::request::CreateVotingBody;
use yew::{function_component, html, use_context, Callback, Html};

use crate::{components::forms::create_voting_form::{CreateVotingForm, CreateVotingFormSubmitData}, providers::client_provider::ClientProvider, utils::get_token};

#[function_component]
pub fn CreateVotingPage() -> Html {
    let client_provider = use_context::<ClientProvider>().unwrap();
    let client = client_provider.client.clone();

    let on_submit = {
        let client = client.clone();

        Callback::from(move |data: CreateVotingFormSubmitData| {
            let client = client.clone();

            let token = get_token();

            let token = match token {
                None => return,
                Some(token) => token
            };

            wasm_bindgen_futures::spawn_local(async move {
                client.create_voting(token, &CreateVotingBody {
                    text: data.text,
                    title: data.title,
                }).await;
            });
        })
    };

    html! {
        <>
            <CreateVotingForm on_submit={on_submit} />
        </>
    }
}
