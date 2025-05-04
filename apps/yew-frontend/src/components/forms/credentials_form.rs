use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{html, Callback, Classes, Component, Html, InputEvent, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct CredentialsFormProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub on_submit: Callback<CredentialsFormSubmitData, ()>
}

pub enum CredentialsFormMessage {
    Submit,
    SetPassword(String),
    SetUsername(String),
}

pub struct CredentialsForm {
    pub state: CredentialsFormSubmitData
}

#[derive(Debug, Default, Clone)]
pub struct CredentialsFormSubmitData {
    pub username: String,
    pub password: String,
}

impl Component for CredentialsForm {
    type Message = CredentialsFormMessage;
    type Properties = CredentialsFormProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { state: CredentialsFormSubmitData::default() }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CredentialsFormMessage::SetPassword(password) => {
                self.state.password = password;
            },
            CredentialsFormMessage::SetUsername(username) => {
                self.state.username = username;
            },
            CredentialsFormMessage::Submit => {
                let on_submit_props = &ctx.props().on_submit;
                let data = self.state.clone();

                on_submit_props.emit(data)
            }
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let username = self.state.username.clone();
        let password = self.state.password.clone();

        let on_submit = ctx
            .link()
            .batch_callback(move |event: SubmitEvent| {
                event.prevent_default();
                Some(CredentialsFormMessage::Submit)
            });

        let on_username_input = ctx
            .link()
            .batch_callback(|event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlInputElement>()
                        .ok()
                    )
                    .map(|input| CredentialsFormMessage::SetUsername(input.value()))
            });

        let on_password_input = ctx
            .link()
            .batch_callback(|event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlInputElement>()
                        .ok()
                    )
                    .map(|input| CredentialsFormMessage::SetPassword(input.value()))
            });

        html! {
            <form
                class="form"
                onsubmit={on_submit}
            >
                <div
                    class="form__field field"
                >
                    <label class="field__label">
                        {"Username"}
                    </label>

                    <br />

                    <input
                        class="field__input"
                        type="text"
                        value={username}
                        oninput={on_username_input}
                    />
                </div>

                <div
                    class="form__field field"
                >
                    <label class="field__label">
                        {"Password"}
                    </label>

                    <br />

                    <input
                        class="field__input"
                        type="password"
                        value={password}
                        oninput={on_password_input}
                    />
                </div>

                <button
                    class="form__button button"
                    type="submit"
                >
                    {"submit"}
                </button>
            </form>
        }
    }
}
