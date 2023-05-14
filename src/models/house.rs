use crate::{models::family::Family, repository::mongodb_repo::MongoRepo};
use mongodb::{
    bson::{oid::ObjectId, DateTime},
    results::InsertOneResult, error::Error
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct House {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub families: Vec<Family>,
    pub address: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Booking {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub start_date: DateTime,
    pub end_date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<BookingPost>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookingPost {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pictures: Vec<String>,
    pub description: String,
}

impl MongoRepo {
    pub fn create_house(&self, new_house: House) -> Result<InsertOneResult, Error> {
        let created_house = self.house_collection.insert_one(new_house, None);
    
        match created_house {
            Ok(house) => Ok(house),
            Err(_) => Err(Error::custom("error creating house"))
        }
    }
}
