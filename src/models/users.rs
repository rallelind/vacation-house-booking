use std::error::Error;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, DecodingKey};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub name: String
}

impl User {
    pub fn create_user(user_data) -> Result<InsertOneResult, > {

    }
}