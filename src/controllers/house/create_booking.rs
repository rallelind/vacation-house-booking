use axum::{Extension, Json};
use serde_json::Value;

use crate::{
    controllers::authentication::login_authorized::AuthedUser, errors::AppError,
    models::house::Booking, repository::mongodb_repo::MongoRepo,
};

pub async fn create_booking(
    _user: AuthedUser,
    Extension(db): Extension<MongoRepo>,
    Json(payload): Json<Booking>,
) -> Result<Json<Value>, AppError> {
    let booking_already_exists = db.booking_exists(payload.start_date, payload.end_date);

    if booking_already_exists.ok().unwrap() > 0 {
        return Err(AppError::BookingAlreadyExists);
    }

    let create_booking = db.create_booking(payload);

    match create_booking {
        Ok(booking) => Ok(Json(serde_json::json!(booking))),
        Err(_) => Err(AppError::InternalServerError),
    }
}
