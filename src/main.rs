use axum::{
    routing::{get, post},
    Extension, Router,
};

use tower_http::cors::CorsLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

mod queue;
mod controllers;
use controllers::files::{upload_file::upload_file, get_file::get_file};

use aws_sdk_s3 as s3;
use s3::Client;

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

    let aws_s3_client = Client::new(&aws_configuration);

    let app = Router::new()
        .route("/", get(|| async move { "welcome to image upload api" }))
        .route("/file/upload", post(upload_file))
        .route("/file", get(get_file))
        .layer(cors_layer)
        .layer(Extension(aws_s3_client));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
