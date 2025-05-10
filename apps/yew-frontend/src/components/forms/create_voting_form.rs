use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{function_component, html, use_state, Callback, Classes, Html, InputEvent, Properties, SubmitEvent};

pub enum CreateVotingFormMessage {
    Submit,
    SetTitle(String),
    SetText(String),
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct CreateVotingFormProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub on_submit: Callback<CreateVotingFormSubmitData>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVotingFormSubmitData {
    title: String,
    text: String,
}

#[function_component]
pub fn CreateVotingForm(props: &CreateVotingFormProps) -> Html {
    let title = use_state(|| String::default());
    let text = use_state(|| String::default());

    let on_submit_prop = props.on_submit.clone();

    let on_submit = {
        let title = title.clone();
        let text = text.clone();

        Callback
            ::from(move |event: SubmitEvent| {
                event.prevent_default();

                on_submit_prop.emit(CreateVotingFormSubmitData {
                    title: (*title).clone(),
                    text: (*text).clone(),
                })
            })
    };

    let on_title_input = {
        let title = title.clone();

        Callback
            ::from(move |event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlInputElement>()
                        .ok()
                    )
                    .map(|input| title.set(input.value()));
            })
    };

    let on_text_input = {
        let text = text.clone();

        Callback
            ::from(move |event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlTextAreaElement>()
                        .ok()
                    )
                    .map(|input| text.set(input.value()));
            })
    };


    html! {
        <form onsubmit={on_submit}>
            <div>
                <input
                    oninput={on_title_input}
                    type="text"
                />
            </div>

            <div>
                <textarea
                    oninput={on_text_input}
                ></textarea>
            </div>

            <button type="submit">
                {"Создать"}
            </button>
        </form>
    }
}
