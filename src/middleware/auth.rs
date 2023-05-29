use async_mongodb_session::MongodbSessionStore;
use std::env::var;

use axum::extract::FromRef;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

#[derive(Clone)]
pub struct AuthState {
    pub store: MongodbSessionStore,
    pub client: BasicClient,
}

impl FromRef<AuthState> for MongodbSessionStore {
    fn from_ref(state: &AuthState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AuthState> for BasicClient {
    fn from_ref(state: &AuthState) -> Self {
        state.client.clone()
    }
}

pub fn oauth_client() -> BasicClient {
    let client_id = var("CLIENT_ID").expect("missing client id");
    let client_secret = var("CLIENT_SECRET").expect("missing client secret");
    let redirect_url = var("REDIRECT_URL").expect("missing redirect url");
    let auth_url = var("AUTH_URL").expect("missing auth url");
    let token_url = var("TOKEN_URL").expect("missing token url");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}
