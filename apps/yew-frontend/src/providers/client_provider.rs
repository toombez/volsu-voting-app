use crate::client::Client;

#[derive(Clone, PartialEq)]
pub struct ClientProvider {
    pub client: Client,
}
