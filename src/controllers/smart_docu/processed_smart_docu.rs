use axum::{extract::Json, http::StatusCode};
use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize)]
pub struct ProcessedSmartDocuBody {
    content: String, // todo figure out best way create a structure for the text content
}

pub async fn processed_smart_docu(
    Json(payload): Json<ProcessedSmartDocuBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    Ok(Json(
        serde_json::json!(payload.content),
    ))
}
