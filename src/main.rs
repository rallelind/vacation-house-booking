// Resources for learning: 
// https://medium.com/intelliconnect-engineering/uploading-files-to-aws-s3-using-axum-a-rust-framework-c96b1c774dfc
// https://www.youtube.com/watch?v=DLmyW58egg4

use axum::{
    extract::MultiPart,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router
};

use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use aws_sdk_s3 as s3;

fn main() {
    println!("Hello, world!");
}
