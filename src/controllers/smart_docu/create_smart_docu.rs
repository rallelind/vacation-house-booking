use aws_sdk_sqs::Client;
use axum::{extract::Json, http::StatusCode, Extension};
use serde::Deserialize;

use crate::queue::{send, SQSMessage};

#[derive(Deserialize)]
pub struct SmartDocuBody {
    file_path: String,
}

pub async fn create_smart_docu(
    Extension(queue_client): Extension<Client>,
    Json(payload): Json<SmartDocuBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let queue_url = std::env::var("AWS_SQS_URL").expect("aws sqs url should be provided");

    let message = &SQSMessage {
        body: payload.file_path.to_owned(),
        group: "ocr_service".to_owned(),
    };

    send(&queue_client, &queue_url, message)
        .await
        .map_err(|err| {
            dbg!(err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "an error occured during document processing"})),
            )
        })?;

    Ok(Json(
        serde_json::json!({"success": "processing document now, we will notify when document has been processed"}),
    ))
}
