use crate::models::users::User;
use std::env::var;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
pub struct MongoRepo {
    pub user_collection: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        let mongo_connection_string =
            var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");
        
        let client = Client::with_uri_str(mongo_connection_string).expect("error connection to client");

        let db = client.database("house_booking");
        let users_collection: Collection<User> = db.collection("Users");
        MongoRepo { user_collection }
    }
}
