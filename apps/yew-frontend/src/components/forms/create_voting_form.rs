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
    pub title: String,
    pub text: String,
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
        <form class="form" onsubmit={on_submit}>
            <div class="form__field field">
                <label class="field__label" for="title-field">
                    {"Заголовок"}
                </label>
                <input
                    class="field__input"
                    id="title-field"
                    oninput={on_title_input}
                    type="text"
                />
            </div>

            <div>
                <label class="field__label" for="title-text">
                    {"Описание"}
                </label>
                <textarea
                    id="title-text"
                    class="field__input"
                    oninput={on_text_input}
                ></textarea>
            </div>

            <button type="submit" class="button">
                {"Создать"}
            </button>
        </form>
    }
}
