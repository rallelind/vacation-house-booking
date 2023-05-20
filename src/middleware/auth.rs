use std::str::FromStr;
use rand_core::RngCore;

use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension
};

use mongodb::bson::oid::ObjectId;

use crate::{models::users::User, repository::mongodb_repo::MongoRepo, Random};

const USER_COOKIE_NAME: &str = "user_token";

#[derive(Clone, Copy)]
pub struct SessionToken(u128);

impl FromStr for SessionToken {
    type Err = <u128 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl SessionToken {
    pub fn generate_new(random: Random) -> Self {
        let mut u128_pool = [0u8; 16];
        random.lock().unwrap().fill_bytes(&mut u128_pool);
        Self(u128::from_le_bytes(u128_pool))
    }

    pub fn into_cookie_value(self) -> String {
        self.0.to_string()
    }

    pub fn into_database_value(self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}
pub struct AuthState(Option<(SessionToken, Option<User>, MongoRepo)>);

impl AuthState {
    pub fn logged_in(&self) -> bool {
        self.0.is_some()
    }

    pub async fn get_user(&mut self, email: String) -> Option<&User> {
        let (session_token, store, database) = self.0.as_mut()?;

        if store.is_none() {
            //fill store
            let user = database.get_user(email).ok().unwrap();

            if user.is_some() {
                *store = user;
            }
        }

        store.as_ref()
    }
}

pub fn new_session(database: MongoRepo, random: Random, user_id: ObjectId) -> SessionToken {
    let session_token = SessionToken::generate_new(random);

    let database_session_value = session_token.into_database_value();

    let _result = database.create_session(user_id, database_session_value);

    session_token
}

pub async fn auth<B>(Extension(database): Extension<MongoRepo>, mut req: Request<B>, next: Next<B>) -> Response {
    let session_token = req
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .find_map(|cookie| {
            (cookie.name() == USER_COOKIE_NAME).then(move || cookie.value().to_owned())
        })
        .and_then(|cookie_value| cookie_value.parse::<SessionToken>().ok());

    req.extensions_mut()
        .insert(AuthState(session_token.map(|v| (v, None, database))));

    next.run(req).await
}
