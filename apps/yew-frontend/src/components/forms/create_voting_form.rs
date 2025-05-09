use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{html, Callback, Classes, Component, InputEvent, Properties, SubmitEvent};

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

pub struct CreateVotingForm {
    state: CreateVotingFormSubmitData,
}

impl Component for CreateVotingForm {
    type Message = CreateVotingFormMessage;
    type Properties = CreateVotingFormProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            state: CreateVotingFormSubmitData::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let props =  ctx.props();

        match msg {
            CreateVotingFormMessage::Submit => {
                let on_submit = &props.on_submit;

                on_submit.emit(self.state.clone());
            },
            CreateVotingFormMessage::SetText(text) => {
                self.state.text = text;
            },
            CreateVotingFormMessage::SetTitle(title) => {
                self.state.title = title;
            },
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_submit = ctx
            .link()
            .batch_callback(|event: SubmitEvent| {
                event.prevent_default();

                Some(CreateVotingFormMessage::Submit)
            });

        let on_title_input = ctx
            .link()
            .batch_callback(|event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlInputElement>()
                        .ok()
                    )
                    .map(|input| CreateVotingFormMessage::SetTitle(input.value()))
            });

        let on_text_input = ctx
            .link()
            .batch_callback(|event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlTextAreaElement>()
                        .ok()
                    )
                    .map(|input| CreateVotingFormMessage::SetText(input.value()))
            });

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
}
