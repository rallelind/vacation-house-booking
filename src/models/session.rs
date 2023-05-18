use mongodb::{bson::oid::ObjectId, results::InsertOneResult, error::Error};
use serde::{Deserialize, Serialize};

use crate::repository::mongodb_repo::MongoRepo;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    session_token: Vec<u8>,
    user_id: Option<ObjectId>
}

impl MongoRepo {

    pub fn create_session(&self, user_id: ObjectId, session_token: Vec<u8>) -> Result<InsertOneResult, Error> {
        // implement create session logic
    }

}