use crate::models::family::Family;
use mongodb::{
    bson::{oid::ObjectId, DateTime}
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
    pub posts: Option<Vec<BookingPost>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookingPost {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pictures: Vec<String>,
    pub description: String,
}