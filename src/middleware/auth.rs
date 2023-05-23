use async_mongodb_session::MongodbSessionStore;
use async_session::{Session, SessionStore};
use std::env::var;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    RequestPartsExt, Router,
};
use http::{header, request::Parts};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;

pub struct AuthState {
    pub store: MongodbSessionStore,
    pub client: BasicClient,
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
