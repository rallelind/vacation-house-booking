use axum::{
    routing::{get, post},
    Extension, Router, middleware
};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

use aws_sdk_s3 as s3;
use aws_sdk_sqs as sqs;
use aws_sdk_textract as textract;

mod queue;
mod controllers;
mod models;
mod errors;

use controllers::{files::{upload_file::upload_file, get_file::get_file}, smart_docu::create_smart_docu::create_smart_docu};

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

    let app = Router::new()
        .route("/", get(|| async move { "welcome to image upload api" }))
        .route("/file", get(get_file))
        .route("/file/upload", post(upload_file))
        .route("/smartdocu", post(create_smart_docu))
        .route("/smartdocu", get(|| async move { "get smart documents status by SSE" }))
        .layer(cors_layer)
        .layer(Extension(aws_s3_client))
        .layer(Extension(aws_textract_client))
        .layer(Extension(aws_sqs_client))
        .layer(middleware::f);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
