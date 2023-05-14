use crate::{models::users::User, repository::mongodb_repo::MongoRepo, controllers::users::update_user::PatchUser};
use mongodb::{bson::oid::ObjectId, error::Error, results::InsertOneResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Family {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub famile_name: String,
    pub members: Vec<ObjectId>,
}

impl MongoRepo {
    pub fn create_family(&self, family_data: Family) -> Result<InsertOneResult, Error> {
        let created_family = self
            .family_collection
            .insert_one(family_data, None)
            .ok()
            .expect("Error creating family");

        for member in family_data.members {
            let patch_user = PatchUser {
                family: Some(family_data),
                email: None,
                name: None,
                avatar: None
            };

            self.update_user_doc(patch_user, member.to_string());
        }

        Ok(created_family)
    }
}
