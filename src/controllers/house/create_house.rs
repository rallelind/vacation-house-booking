use axum::{Extension, Json};
use serde_json::Value;

use crate::{
    controllers::authentication::login_authorized::AuthedUser, errors::AppError,
    models::house::House, repository::mongodb_repo::MongoRepo,
};

pub async fn create_house(
    user: AuthedUser,
    Extension(db): Extension<MongoRepo>,
    Json(payload): Json<House>,
) -> Result<Json<Value>, AppError> {
    let created_house = db.create_house(payload);

    match created_house {
        Ok(new_house) => Ok(Json(serde_json::json!(new_house))),
        Err(_) => Err(AppError::InternalServerError),
    }
}
