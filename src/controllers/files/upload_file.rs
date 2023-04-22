use std::collections::HashMap;
use axum::{
    extract::Multipart,
    http::StatusCode,
    Extension, Json,
};

use aws_sdk_s3 as s3;
use s3::Client;

pub async fn upload_file(
    Extension(s3_client): Extension<Client>,
    mut files: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bucket = std::env::var("AWS_S3_BUCKET").unwrap_or("my-bucket-name".to_owned());
    let bucket_url = std::env::var("BUCKET_URL").unwrap_or(bucket.to_owned());

    let mut res = HashMap::new();
    while let Some(file) = files.next_field().await.unwrap() {

        let category = file.name().unwrap().to_string();
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();
        let key = format!(
            "images/{}_{}_{}",
            chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
            &category,
            &name
        );

        // send Putobject request to aws s3
        let _resp = s3_client
            .put_object()
            .bucket(&bucket)
            .key(&key)
            .body(data.into())
            .send()
            .await
            .map_err(|err| {
                dbg!(err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"err": "an error occured during image upload"})),
                )
            })?;
        dbg!(_resp);
        res.insert(
            format!("{}_{}", &name, &category),
            format!(
                "{}/{}",
                bucket_url,
                key
            ),
        );
    }
    Ok(Json(serde_json::json!(res)))
}