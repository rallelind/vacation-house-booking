use crate::{models::family::Family, repository::mongodb_repo::MongoRepo};
use mongodb::{
    bson::{oid::ObjectId, DateTime}, results::InsertOneResult
};
use serde::{Deserialize, Serialize, de::value::Error};

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
    pub posts: Option<Vec<BookingPost>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookingPost {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pictures: Vec<String>,
    pub description: String,
}

