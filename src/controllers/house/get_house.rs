use axum::{extract::Path, Extension, Json};
use serde_json::Value;

use crate::{errors::AppError, repository::mongodb_repo::MongoRepo};

pub async fn get_house(
    Extension(db): Extension<MongoRepo>,
    Path((house_id, _user_id)): Path<(String, String)>,
) -> Result<Json<Value>, AppError> {

    let found_house = db.get_house(house_id);

    match found_house {
        Ok(house) => Ok(Json(serde_json::json!(house))),
        Err(_) => Err(AppError::InternalServerError)
    }

}
