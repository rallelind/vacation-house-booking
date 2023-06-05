use axum::{Extension, Json};
use reqwest::{Client, header};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env::var;

use crate::{errors::AppError, repository::mongodb_repo::MongoRepo, models::invitations::{self, Invitation}};

#[derive(Deserialize)]
pub struct Sender {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct Recipient {
    email: String,
    #[serde(rename = "familyName")]
    family_name: String
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

    let response = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(send_grid_api_key)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        )
        .send()
        .await;

    match response {
        Ok(_res) => {

            let new_invitation = Invitation {
                family: recipient.family_name,
                invited_email: recipient.email,
                id: None
            };

            let invitation_creation = db.create_invitation(new_invitation);

            match invitation_creation {
                Ok(_mail) => Ok(Json(serde_json::json!("send email!"))),
                Err(_) => Err(AppError::InternalServerError)
            }
            
        },
        Err(_) => Err(AppError::InternalServerError),
    }
}
