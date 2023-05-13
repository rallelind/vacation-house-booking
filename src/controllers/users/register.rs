use axum::extract::Json;
use axum::Extension;
use serde::Deserialize;
use serde_json::Value;

use crate::errors::AppError;
use crate::MongoRepo;
use crate::models::users::User;

#[derive(Deserialize)]
pub struct RegisterBody {
    username: String,
    password: String,
    confirm_password: String,
    email: String,
    avatar: String,
}

pub async fn register_user(
    Json(payload): Json<RegisterBody>,
    Extension(db): Extension<MongoRepo>,
) -> Result<Json<Value>, AppError> {

    let RegisterBody { username, password, confirm_password, email, avatar } = payload;

    if confirm_password != password {
        return Err(AppError::PasswordMismatch);
    }

    let user_data = User {
        id: None,
        name: username.clone(),
        password: password.clone(),
        email: email.clone(),
        avatar: avatar.clone()
    };

    let registered_user = db.create_user(user_data);
    
    match registered_user {
        Ok(user) => Ok(Json(serde_json::json!(user))),
        Err(_) => Err(AppError::InternalServerError)
    }

}
