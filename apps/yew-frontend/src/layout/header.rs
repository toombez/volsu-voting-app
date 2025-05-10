use yew::{function_component, html, use_effect_with, use_state, Callback, Html, MouseEvent};
use yew_router::prelude::*;
use yewdux::use_store;
use crate::{router::main_route::MainRoute, state::app_state::AppState, utils::{clear_token, get_token}};

#[function_component]
pub fn Header() -> Html {
    let is_logged_in = use_state(|| false);

    let (state, dispatch) = use_store::<AppState>();

    {
        let is_logged_in = is_logged_in.clone();

        use_effect_with(state, move |state| {
            is_logged_in.set(state.is_logged_in());
        });
    }

    let log_out = {
        let dispatch = dispatch.clone();

        Callback
            ::from(move |event: MouseEvent| {
                event.prevent_default();

                clear_token();
                gloo::console::log!(format!("Logged {:?}", get_token()));

                dispatch.reduce_mut(|state| state.auth_user = None);
            })
    };

    return html! {
        <header class="header">
            <section class="section">
                <nav class="header__navigation navigation">
                    <Link<MainRoute>
                        classes="navigation__link link"
                        to={MainRoute::Index}
                    >
                        { "Главная" }
                    </Link<MainRoute>>

                    if *is_logged_in {
                        <div class="header__profile-navigation">
                            <Link<MainRoute>
                                classes="navigation__link link"
                                to={MainRoute::Profile}
                            >
                                {"Профиль"}
                            </Link<MainRoute>>

                            <button onclick={log_out}>
                                {"Выйти"}
                            </button>
                        </div>
                    } else {
                        <div class="header__user-navigation">
                            <Link<MainRoute>
                                classes="navigation__link link"
                                to={MainRoute::Login}
                            >
                                {"Войти"}
                            </Link<MainRoute>>

                            <Link<MainRoute>
                                classes="navigation__link link"
                                to={MainRoute::Register}
                            >
                                {"Зарегистрироваться"}
                            </Link<MainRoute>>
                        </div>
                    }
                </nav>
            </section>
        </header>
    }
}
