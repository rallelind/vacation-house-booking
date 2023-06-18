use axum::{Extension, Json};
use serde_json::Value;

use crate::{
    controllers::authentication::login_authorized::AuthedUser, errors::AppError,
    repository::mongodb_repo::MongoRepo,
};

pub async fn get_family(
    user: AuthedUser,
    Extension(db): Extension<MongoRepo>,
) -> Result<Json<Value>, AppError> {
    let found_family = db.get_family(user.email);

    match found_family {
        Ok(family) => {

            if let Some(fam) = family {
                Ok(Json(serde_json::json!(fam)))
            } else {
                Err(AppError::NotPartOfFamily)
            }

        },
        Err(_) => Err(AppError::InternalServerError),
    }
}
