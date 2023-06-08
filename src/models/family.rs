use crate::{models::users::PatchUser, models::users::User, repository::mongodb_repo::MongoRepo};
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    error::Error,
    results::{InsertOneResult, UpdateResult},
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

    pub fn add_user_to_family(&self, user_email: String, family_name: String) -> Result<UpdateResult, Error> {

        let user_part_of_family = self.user_part_of_family(user_email.clone());

        if user_part_of_family {
            return Err(Error::custom("the user is already part of the family")); 
        }
        
        let filter = doc! {
            "email": user_email
        };

        let found_user = self.user_collection.find_one(filter, None);

        match found_user {
            Ok(user) => {
                if let Some(new_user) = user {
                    let query = doc! { "family_name": family_name };
                    let update = doc! { "$push": { "members": to_bson(&new_user).unwrap() } };

                    let added_user = self.family_collection.update_one(query, update, None);

                    match added_user {
                        Ok(added_user_to_family) => Ok(added_user_to_family),
                        Err(_) => Err(Error::custom("error adding user to family")),
                    }
                } else {
                    return Err(Error::custom("error adding user to family"));
                }
            },
            Err(_) => Err(Error::custom("error adding user to family")),
        }

    }
}
