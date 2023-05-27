use axum::Json;
use serde_json::Value;

use crate::controllers::authentication::login_authorized::AuthedUser;

pub async fn me(user: AuthedUser) -> Json<Value> {
    Json(serde_json::json!(user))
}