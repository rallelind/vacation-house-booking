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
use aws_sdk_sqs as sqs;
use aws_sdk_textract as textract;

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
};
use repository::mongodb_repo::MongoRepo;

use middleware::auth::{oauth_client, AuthState};


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "aws-s3-file-upload-api-rust=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors_layer = CorsLayer::permissive();

    let aws_configuration = aws_config::load_from_env().await;

    let aws_s3_client = s3::Client::new(&aws_configuration);

    let db = MongoRepo::init();

    let mongo_connection_string =
    var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");

    let store = MongodbSessionStore::new(mongo_connection_string.as_str(), "cluster0", "sessions").await.unwrap();
    let client = oauth_client();

    let auth_state = AuthState {
        store,
        client
    };

    let app = Router::new()
        .route("/", get(|| async move { "welcome to image upload api" }))
        .route("/file", get(get_file))
        .route("/file/upload", post(upload_file))
        .route("/smartdocu", post(create_smart_docu))
        .route("/user", post(register_user))
        .route("/user/:user_id", patch(update_user))
        .route("/family", post(create_family))
        .route("/house", post(create_house))
        .route("/house/booking", post(create_booking))
        .route("/house/booking/post", post(create_booking_post))
        .layer(cors_layer)
        .layer(Extension(db))
        .layer(Extension(aws_s3_client));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
