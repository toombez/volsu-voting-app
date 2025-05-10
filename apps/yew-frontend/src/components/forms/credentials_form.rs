use serde::{Deserialize, Serialize};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{classes, function_component, html, use_state, Callback, Classes, Html, InputEvent, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct CredentialsFormProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub on_submit: Callback<CredentialsFormSubmitData, ()>
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CredentialsFormSubmitData {
    pub username: String,
    pub password: String,
}

#[function_component]
pub fn CredentialsForm(props: &CredentialsFormProps) -> Html {
    let username = use_state(|| String::default());
    let password = use_state(|| String::default());

    let class_prop = props.class.clone();
    let on_submit_prop = props.on_submit.clone();

    let on_submit = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            on_submit_prop.emit(CredentialsFormSubmitData {
                username: (*username).clone(),
                password: (*password).clone(),
            })
        })
    };

    let on_username_input = {
        let username = username.clone();

        Callback
            ::from(move |event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlInputElement>()
                        .ok()
                    )
                    .map(|input| username.set(input.value()));
            })
    };

    let on_password_input = {
        let password = password.clone();

        Callback
            ::from(move |event: InputEvent| {
                event
                    .target()
                    .and_then(|target| target
                        .dyn_into::<HtmlInputElement>()
                        .ok()
                    )
                    .map(|input| password.set(input.value()));
            })
    };

    html! {
        <form
            class={classes!("form", class_prop)}
            onsubmit={on_submit}
        >
            <div
                class="form__field field"
            >
                <label
                    for="username-field"
                    class="field__label"
                >
                    {"Логин"}
                </label>

                <input
                    class="field__input"
                    type="text"
                    id="username-field"
                    value={(*username).clone()}
                    oninput={on_username_input}
                />
            </div>

            <div
                class="form__field field"
            >
                <label
                    for="password-field"
                    class="field__label"
                >
                    {"Password"}
                </label>

                <input
                    id="password-field"
                    class="field__input"
                    type="password"
                    value={(*password).clone()}
                    oninput={on_password_input}
                />
            </div>

            <button
                class="form__button button"
                type="submit"
            >
                {"Отправить"}
            </button>
        </form>
    }
}
