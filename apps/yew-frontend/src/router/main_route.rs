use uuid::Uuid;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Index,

    #[at("/profile")]
    Profile,

    #[at("/login")]
    Login,

    #[at("/register")]
    Register,

    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/votings")]
    Votings,

    #[at("/voting/:id")]
    Voting {
        id: Uuid,
    },

    #[at("/create_voting")]
    CreateVoting,
}
