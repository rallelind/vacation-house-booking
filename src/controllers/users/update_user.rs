use axum::{Extension, Json, extract::Path};
use serde_json::Value;

use crate::{errors::AppError, repository::mongodb_repo::MongoRepo, models::users::PatchUser};


pub async fn update_user(
    Extension(db): Extension<MongoRepo>,
    Path(user_id): Path<String>,
    Json(payload): Json<PatchUser>
) -> Result<Json<Value>, AppError> {

    let updated_user = db.update_user_doc(payload, user_id);

    match updated_user {
        Ok(updated_user) => Ok(Json(serde_json::json!(updated_user))),
        Err(_) => Err(AppError::InternalServerError) 
    }

}
