use yew::{function_component, html, use_state, Html};
use yew_router::prelude::*;
use crate::router::main_route::MainRoute;

#[function_component]
pub fn Header() -> Html {
    let is_logged_in = use_state(|| false);

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

                            <button>
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
