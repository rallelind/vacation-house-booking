use axum::{extract::Json, http::StatusCode};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct RegisterBody {
    username: String,
    password: String,
    confirm_password: String,
}

pub async fn register_user(Json(payload): Json<RegisterBody>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    
}