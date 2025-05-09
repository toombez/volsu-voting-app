use wasm_bindgen::JsCast;
use yew::{html, Component, MouseEvent};
use yew_router::prelude::*;
use crate::router::main_route::MainRoute;

pub enum HeaderMessage {
    Logout,
}

pub struct Header;

impl Component for Header {
    type Message = HeaderMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
        ;

        match msg {
            HeaderMessage::Logout => {
                let deleted_cookie = wasm_cookies::cookies::delete("token");
                let _ = document.set_cookie(&deleted_cookie);
            },
        };

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
        ;

        let cookie = document.cookie().unwrap();
        let token = wasm_cookies::cookies::get(&cookie, "token");

        let is_logged_in = match token {
            None => false,
            Some(_token) => true,
        };

        let logout = ctx
            .link()
            .batch_callback(|event: MouseEvent| {
                event.prevent_default();

                Some(HeaderMessage::Logout)
            });

        html! {
            <header class="header">
                <section class="section">
                    <nav class="header__navigation navigation">
                        <Link<MainRoute>
                            classes="navigation__link link"
                            to={MainRoute::Index}
                        >
                            { "Главная" }
                        </Link<MainRoute>>

                        if is_logged_in {
                            <div class="header__profile-navigation">
                                <Link<MainRoute>
                                    classes="navigation__link link"
                                    to={MainRoute::Profile}
                                >
                                    {"Профиль"}
                                </Link<MainRoute>>

                                <button
                                    onclick={logout}
                                >
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
}
