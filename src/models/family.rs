use crate::{models::users::User, repository::mongodb_repo::MongoRepo, controllers::users::update_user::PatchUser};
use mongodb::{bson::{oid::ObjectId}, error::Error, results::InsertOneResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Family {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub famile_name: String,
    pub members: Vec<User>,
}

impl MongoRepo {
    pub fn create_family(&self, family_data: Family) -> Result<InsertOneResult, Error> {
        let created_family = self
            .family_collection
            .insert_one(family_data.clone(), None);
            
        for member in family_data.members.clone() {
            let patch_user = PatchUser {
                family: Some(family_data.clone()),
                email: None,
                name: None,
                avatar: None
            };

            if member.id.is_none() {
                return Err(Error::custom("no user id provided"))
            }

            let member_id_string = match member.id {
                Some(id) => id.to_hex(),
                _ => "provide a id".to_string()
            };

            self.update_user_doc(patch_user, member_id_string).ok();
        }

        match created_family {
            Ok(new_fam) => Ok(new_fam),
            Err(_) => Err(Error::custom("error creating family"))
        }
    }
}
