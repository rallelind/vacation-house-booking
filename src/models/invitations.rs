use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::InsertOneResult,
};use serde::{Deserialize, Serialize};

use crate::{models::family::Family, repository::mongodb_repo::MongoRepo};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invitation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub family: Family,
    #[serde(rename = "invitedEmail")]
    pub invited_email: String
}

impl MongoRepo {
    pub fn create_invitation(&self, new_invitation: Invitation) -> Result<InsertOneResult, Error> {

        let created_invitation = self.invitations_collection.insert_one(new_invitation, None);

        match created_invitation {
            Ok(invitation) => Ok(invitation),
            Err(_) => Err(Error::custom("error creating invitation"))
        }

    }
}