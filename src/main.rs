use async_mongodb_session::*;
use std::env::var;

use axum::{
    middleware::from_fn,
    routing::{get, patch, post},
    Extension, Router,
};
use dotenv::dotenv;
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;
use rand_core::{OsRng, RngCore};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use aws_sdk_s3 as s3;

mod controllers;
mod errors;
mod middleware;
mod models;
mod repository;

use controllers::{
    bookings::{create_booking::create_booking, create_booking_post::create_booking_post},
    family::create_family::create_family,
    files::{get_file::get_file, upload_file::upload_file},
    house::create_house::create_house,
    smart_docu::create_smart_docu::create_smart_docu,
    users::{register::register_user, update_user::update_user},
    authentication::{google_auth::google_auth, login_authorized::login_authorized}
};
use repository::mongodb_repo::MongoRepo;

use middleware::auth::{oauth_client, AuthState};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_connection_string =
    var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");

    let store = MongodbSessionStore::new(mongo_connection_string.as_str(), "cluster0", "sessions").await.unwrap();
    let client = oauth_client();

    let auth_state = AuthState {
        store,
        client
    };

    let app = Router::new()
        .route("/auth/google", get(google_auth))
        .route("/auth/authorized", get(login_authorized))
        .with_state(auth_state);


    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
