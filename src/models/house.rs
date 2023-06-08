use crate::{models::family::Family, repository::mongodb_repo::MongoRepo};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
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
    pub admin_needs_to_approve: bool
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
    pub approved: bool
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookingPost {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pictures: Vec<String>,
    pub description: String,
    pub booking: Booking
}

impl MongoRepo {
    pub fn create_house(&self, new_house: House) -> Result<InsertOneResult, Error> {
        let created_house = self.house_collection.insert_one(new_house, None);

        match created_house {
            Ok(house) => Ok(house),
            Err(_) => Err(Error::custom("error creating house")),
        }
    }

    pub fn create_booking(&self, mut new_booking: Booking) -> Result<InsertOneResult, Error> {

        if new_booking.house.admin_needs_to_approve {
            new_booking.approved = false
        } else {
            new_booking.approved = true
        }

        let created_booking = self.booking_collection.insert_one(new_booking, None);

        match created_booking {
            Ok(booking) => Ok(booking),
            Err(_) => Err(Error::custom("error creating booking")),
        }
    }

    pub fn create_booking_post(
        &self,
        new_booking_post: BookingPost,
    ) -> Result<InsertOneResult, Error> {
        let created_booking_post = self
            .booking_post_collection
            .insert_one(new_booking_post, None);

        match created_booking_post {
            Ok(booking_post) => Ok(booking_post),
            Err(_) => Err(Error::custom("error creating booking post")),
        }
    }

    pub fn booking_exists(&self, start_date: DateTime, end_date: DateTime) -> Result<u64, Error> {
        let filter = doc! {
            "$and": [
                { "start_date": { "$lt": start_date } },
                { "end_date": { "$gt": end_date } }
            ]
        };

        let count = self.booking_collection.count_documents(filter, None);

        match count {
            Ok(counted_documents) => Ok(counted_documents),
            Err(_) => Err(Error::custom("error counting documents")),
        }
    }

    pub fn get_house(&self, house_id: String) -> Result<Option<House>, Error> {

        let house_id_obj = ObjectId::parse_str(&house_id).expect("Invalid house_id");

        let filter = doc! {
            "_id": house_id_obj
        };

        let found_house = self.house_collection.find_one(filter, None);

        match found_house {
            Ok(document) => Ok(document),
            Err(err) => Err(err)
        }

    }

    pub fn user_part_of_house(&self, house_id: String, user_id: String) -> bool {
        let house_id_obj = ObjectId::parse_str(&house_id).expect("Invalid house_id");
        let user_id_obj = ObjectId::parse_str(&user_id).expect("Invalid user_id");

        let filter = doc! {
            "_id": house_id_obj,
            "families.members": {
                "$elemMatch": {
                    "_id": user_id_obj
                }
            }
        };

        let found_document = self.house_collection.find_one(filter, None);

        println!("{:?}", found_document);

        match found_document {
            Ok(document) => {
                if document.is_none() {
                    return false
                } else {
                    return true
                }
            },
            Err(_) => false,
        }
    }
}
