use types::dto::request::PaginationQuery;
use yew::{function_component, html, use_context, use_effect_with, use_state, Callback, Html, MouseEvent};

use crate::{components::votings_grid::VotingsGrid, providers::client_provider::ClientProvider};

#[function_component]
pub fn VotingsPage() -> Html {
    let page = use_state(|| 0);
    let per_page = use_state(|| 8);
    let last_page = use_state(|| 0);

    let votings = use_state(|| vec![]);

    let ClientProvider {
        client
    } = use_context
        ::<ClientProvider>()
        .expect("Client provider not found");

    use_effect_with(page.clone(), {
        let client = client.clone();
        let per_page = per_page.clone();
        let last_page = last_page.clone();
        let votings = votings.clone();

        move |page| {
            wasm_bindgen_futures::spawn_local({
                let client = client.clone();
                let page = page.clone();
                let per_page = per_page.clone();
                let last_page = last_page.clone();
                let votings = votings.clone();

                async move {
                    let response = client
                        .get_votings(&PaginationQuery {
                            page: *page,
                            per_page: *per_page,
                        })
                        .await;

                    let response = match response {
                        Err(()) => {
                            votings.set(vec![]);
                            page.set(0);
                            last_page.set(0);
                            return
                        },
                        Ok(response) => response,
                    };

                    last_page.set(response.data.pagination.last_page);
                    votings.set(response.data.items);
                }
            });
        }
    });

    let prev_page = {
        let page = page.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            page.set(*page - 1);
        })
    };

    let next_page = {
        let page = page.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            page.set(*page + 1);
        })
    };

    html!(
        <section class="section">
            <h1 class="section__heading">
                {"Страница "}{*page + 1}
            </h1>

            <VotingsGrid votings={(*votings).clone()} />

            <div>
                <button class="button"
                    disabled={*page <= 0}
                    onclick={prev_page}
                >
                    {"Предыдущая страница"}
                </button>

                <button
                    class="button"
                    disabled={(*page + 1) > *last_page as u64}
                    onclick={next_page}
                >
                    {"Следующая страница"}
                </button>
            </div>
        </section>
    )
}
