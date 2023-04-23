use aws_sdk_sqs::Client;
use axum::{http::StatusCode, Extension, Json};

use crate::queue::{send, SQSMessage};

pub async fn create_smart_docu(
    Extension(queue_client): Extension<Client>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let queue_url = std::env::var("AWS_SQS_URL").expect("aws sqs url should be provided");

    let message = &SQSMessage {
        body: "hello world".to_owned(),
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
