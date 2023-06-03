use std::env;

use crate::errors::AppError;

struct Sender {
    name: String,
    email: String
}

struct Recipient {
    name: String,
    email: String
}

pub async fn send_email() -> Result<(), AppError> {

}