use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Index,

    #[at("/profile")]
    Profile,

    #[not_found]
    #[at("/404")]
    NotFound,
}
