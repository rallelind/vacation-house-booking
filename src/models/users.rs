use crate::repository::mongodb_repo::MongoRepo;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    Collection,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub name: String,
}

impl MongoRepo {
    pub fn create_user(&self, user_data: User) -> Result<InsertOneResult, Error> {
        let user = self
            .user_collection
            .insert_one(user_data, None)
            .ok()
            .expect("Error creating user");
        
        Ok(user)
    }
}
