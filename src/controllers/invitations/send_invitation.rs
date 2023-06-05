use axum::{Extension, Json};
use serde::Deserialize;
use serde_json::{Value, json};
use std::env::var;
use reqwest::{header, blocking::Client};

use crate::{errors::AppError, repository::mongodb_repo::MongoRepo};

#[derive(Deserialize)]
struct Sender {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct Recipient {
    email: String,
}

#[derive(Deserialize)]
pub struct EmailPayload {
    sender: Sender,
    recipient: Recipient,
}

pub async fn send_invitation(
    Extension(db): Extension<MongoRepo>,
    Json(payload): Json<EmailPayload>,
) -> Result<Json<Value>, AppError> {

    let send_grid_api_key = var("SENDGRID_API_KEY").expect("issue reading sendgrid api key");

    let EmailPayload { sender, recipient } = payload;

     let body = json!(
        {
            "personalizations": [{
                "from": {
                    "email": sender.email,
                    "name": sender.name
                },
                "to": {
                    "email": recipient.email
                },
                "subject": "Invitation to join havklitvej 60",
                "content": [
                    {
                        "type": "text/html",
                        "value": "Invitation to join the vacation house havklitvej 60"
                    }
                ]
            }]
        }
     );

    let client = Client::new().post("https://api.sendgrid.com/v3/mail/send").json(&body).bearer_auth(send_grid_api_key).header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

    let response = client.send();

     match response {
        Ok(res) => Ok(Json(serde_json::json!("send email!"))),
        Err(_) => Err(AppError::InternalServerError)
     }
    

}
