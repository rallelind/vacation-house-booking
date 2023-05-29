use async_mongodb_session::MongodbSessionStore;
use std::env::var;

use axum::{
    routing::get,
    Extension, Router
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

mod controllers;
mod errors;
mod middleware;
mod models;
mod repository;

use controllers::{
    authentication::{
        google_auth::google_auth, login_authorized::login_authorized, logout::logout,
    },
    users::me::me,
};
use repository::mongodb_repo::MongoRepo;

use middleware::auth::{oauth_client, AuthState};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_connection_string =
        var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");

    let store = MongodbSessionStore::new(
        mongo_connection_string.as_str(),
        "house_booking",
        "sessions",
    )
    .await
    .unwrap();
    let client = oauth_client();
    let db = MongoRepo::init();
    let cors_layer = CorsLayer::permissive();

    let auth_state = AuthState { store, client };

    let app = Router::new()
        .route("/auth/logout", get(logout))
        .route("/auth/google", get(google_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/users/me", get(me))
        .layer(cors_layer)
        .layer(Extension(db))
        .with_state(auth_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
