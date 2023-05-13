use crate::models::users::User;
use mongodb::{
    bson::{oid::ObjectId}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Family {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub famile_name: String,
    pub members: Vec<User>
}