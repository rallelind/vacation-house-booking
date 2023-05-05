use axum::{extract::Json, http::StatusCode, Extension};
use aws_sdk_textract::{Client, types::{Document, S3Object}};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct SmartDocuBody {
    file_path: String,
}

pub async fn create_smart_docu(
    Extension(textract_client): Extension<Client>,
    Json(payload): Json<SmartDocuBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    let s3_object = S3Object::builder()
        .bucket("learningio")
        .name(payload.file_path)
        .build();

    let res = textract_client
        .detect_document_text()
        .document(Document::builder().s3_object(s3_object).build())
        .send()
        .await
        .expect("error finding file");

    for text in res.blocks().expect("error reading block") {
        println!("{:?}", text.clone());
    }

    Ok(Json(
        serde_json::json!({"success": "processing document now, we will notify when document has been processed"}),
    ))
}
