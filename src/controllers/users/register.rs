use axum::{extract::Json};
use serde::Deserialize;
use serde_json::Value;

use crate::errors::AppError;

#[derive(Deserialize)]
pub struct RegisterBody {
    username: String,
    password: String,
    confirm_password: String,
}

pub async fn register_user(Json(payload): Json<RegisterBody>) -> Result<Json<Value>, AppError> {

}