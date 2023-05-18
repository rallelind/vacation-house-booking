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

        let token = Session {
            session_token,
            user_id: Some(user_id)
        };

        let new_session = self.session_collection.insert_one(token, None);

        match new_session {
            Ok(session) => Ok(session),
            Err(_) => Err(Error::custom("error creating session"))
        }
    }

}