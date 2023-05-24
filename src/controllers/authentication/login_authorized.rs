use async_mongodb_session::MongodbSessionStore;
use async_session::{Session, SessionStore};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    http::{header::SET_COOKIE, HeaderMap},
};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: String,
    picture: Option<String>,
    name: String,
}

static COOKIE_NAME: &str = "SESSION";

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(store): State<MongodbSessionStore>,
    State(oauth_client): State<BasicClient>,
) -> impl IntoResponse {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let user_data = client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    let cookie = store.store_session(session).await.unwrap().unwrap();

    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to("/"))
}
