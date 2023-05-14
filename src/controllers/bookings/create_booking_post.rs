use axum::{Extension, Json};
use serde_json::{Value, json};

use crate::{errors::AppError, models::house::BookingPost, repository::mongodb_repo::MongoRepo};

pub async fn create_booking_post(
    Extension(db): Extension<MongoRepo>,
    Json(payload): Json<BookingPost>,
) -> Result<Json<Value>, AppError> {

    let created_booking_post = db.create_booking_post(payload);

    match created_booking_post {
        Ok(booking_post) => Ok(Json(json!(booking_post))),
        Err(_) => Err(AppError::InternalServerError)
    }

}
