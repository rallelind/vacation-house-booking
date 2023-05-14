use crate::{models::family::Family, repository::mongodb_repo::MongoRepo};
use mongodb::{
    bson::{oid::ObjectId, DateTime, doc},
    error::Error,
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct House {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub families: Vec<Family>,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookings: Option<Vec<Booking>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Booking {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub family: Family,
    pub house: House,
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
            Err(_) => Err(Error::custom("error creating house")),
        }
    }

    pub fn create_booking(&self, new_booking: Booking) -> Result<InsertOneResult, Error> {
        let created_booking = self.booking_collection.insert_one(new_booking, None);

        match created_booking {
            Ok(booking) => Ok(booking),
            Err(_) => Err(Error::custom("error creating booking")),
        }
    }

    pub fn create_booking_post(&self, new_booking_post: BookingPost) -> Result<InsertOneResult, Error> {
        let created_booking_post = self.booking_post_collection.insert_one(new_booking_post, None);
    
        match created_booking_post {
            Ok(booking_post) => Ok(booking_post),
            Err(_) => Err(Error::custom("error creating booking post"))
        }
    }

    pub fn booking_exists(
        &self,
        start_date: DateTime,
        end_date: DateTime,
    ) -> Result<u64, Error> {

        let filter = doc! {
            "$and": [
                { "start_date": { "$lt": start_date } },
                { "end_date": { "$gt": end_date } }
            ]
        };

        let count = self.booking_collection.count_documents(filter, None);

        match count {
            Ok(counted_documents) => Ok(counted_documents),
            Err(_) => Err(Error::custom("error counting documents"))
        }
    }
}
