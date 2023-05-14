use axum::{Json, Extension};
use serde::Deserialize;
use serde_json::Value;

use crate::{errors::AppError, repository::mongodb_repo::MongoRepo, models::family::Family};

#[derive(Deserialize)]
struct FamilyPayload {
    users: Vec<String>
}

pub async fn create_family(Json(FamilyPayload): Json<FamilyPayload>, Extension(db): Extension<MongoRepo>) -> Result<Json<Value>, AppError> {
    
    let new_family = Family {
        
    }

}