use crate::{models::users::PatchUser, models::users::User, repository::mongodb_repo::MongoRepo};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Family {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub family_name: String,
    pub members: Vec<User>,
}

impl MongoRepo {
    pub fn create_family(&self, family_data: Family) -> Result<InsertOneResult, Error> {
        let created_family = self.family_collection.insert_one(family_data.clone(), None);

        for member in family_data.members.clone() {
            let patch_user = PatchUser {
                family: Some(family_data.clone()),
            };

            let member_id_string = match member.id {
                Some(id) => id.to_hex(),
                _ => "provide a id".to_string(),
            };

            self.update_user_doc(patch_user, member_id_string).ok();
        }

        match created_family {
            Ok(new_fam) => Ok(new_fam),
            Err(_) => Err(Error::custom("error creating family")),
        }
    }

    pub fn user_part_of_family(&self, user_email: String) -> bool {
        let filter = doc! {
            "members": {
                "$elemMatch": {
                    "email": user_email
                }
            }
        };

        let found_document = self.family_collection.find_one(filter, None);

        match found_document {
            Ok(document) => return true,
            Err(err) => return false,
        }
    }
}
