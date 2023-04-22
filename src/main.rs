// Resources for learning:
// https://medium.com/intelliconnect-engineering/uploading-files-to-aws-s3-using-axum-a-rust-framework-c96b1c774dfc
// https://www.youtube.com/watch?v=DLmyW58egg4

use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};

use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use aws_sdk_s3 as s3;
use dotenv::dotenv;

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
        .route("/upload", post(upload_image))
        .layer(cors_layer)
        .layer(Extension(aws_s3_client));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("starting server on port: {}", addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

async fn upload_image(
    Extension(s3_client): Extension<Client>,
    mut files: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bucket = std::env::var("AWS_S3_BUCKET").unwrap_or("my-bucket-name".to_owned());
    let bucket_url = std::env::var("BUCKET_URL").unwrap_or(bucket.to_owned());

    let mut res = HashMap::new();
    while let Some(file) = files.next_field().await.unwrap() {

        let category = file.name().unwrap().to_string();
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();
        let key = format!(
            "images/{}_{}_{}",
            chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
            &category,
            &name
        );

        // send Putobject request to aws s3
        let _resp = s3_client
            .put_object()
            .bucket(&bucket)
            .key(&key)
            .body(data.into())
            .send()
            .await
            .map_err(|err| {
                dbg!(err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"err": "an error occured during image upload"})),
                )
            })?;
        dbg!(_resp);
        res.insert(
            format!("{}_{}", &name, &category),
            format!(
                "{}/{}",
                bucket_url,
                key
            ),
        );
    }
    Ok(Json(serde_json::json!(res)))
}