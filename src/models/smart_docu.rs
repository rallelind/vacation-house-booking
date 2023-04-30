use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartDocu {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pages: Vec<Page>,
    pub page_amount: i32,
}