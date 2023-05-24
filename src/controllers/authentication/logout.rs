use async_mongodb_session::MongodbSessionStore;
use async_session::SessionStore;
use axum::{extract::State, headers::Cookie, response::{IntoResponse, Redirect}, TypedHeader};

use crate::controllers::authentication::login_authorized::COOKIE_NAME;

pub async fn logout(
    State(store): State<MongodbSessionStore>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> impl IntoResponse {

    let cookie = cookies.get(COOKIE_NAME).unwrap();

    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        None => return Redirect::to("/")
    };

    store.destroy_session(session).await.unwrap();

    Redirect::to("/")

}
