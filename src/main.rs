use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post, patch},
    middleware::from_fn,
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
mod models;
mod queue;
mod repository;
mod middleware;

use controllers::{
    files::{get_file::get_file, upload_file::upload_file},
    smart_docu::create_smart_docu::create_smart_docu,
    users::{register::register_user, update_user::update_user},
    family::{create_family::create_family},
    house::create_house::create_house,
    bookings::{create_booking::create_booking, create_booking_post::create_booking_post}
};
use repository::mongodb_repo::MongoRepo;

use middleware::auth::auth;

pub type Random = Arc<Mutex<ChaCha8Rng>>;

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
    let aws_sqs_client = sqs::Client::new(&aws_configuration);
    let aws_textract_client = textract::Client::new(&aws_configuration);

    
    let db = MongoRepo::init();

    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    
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
        .layer(Extension(aws_s3_client))
        .layer(Extension(aws_textract_client))
        .layer(Extension(aws_sqs_client));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
