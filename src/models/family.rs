use crate::{models::users::User, repository::mongodb_repo::MongoRepo, controllers::users::update_user::PatchUser};
use mongodb::{bson::{oid::ObjectId, extjson::de::Error}, results::InsertOneResult};
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
            .insert_one(family_data.clone(), None)
            .ok()
            .expect("Error creating family");

        for member in family_data.members.clone() {
            let patch_user = PatchUser {
                family: Some(family_data.clone()),
                email: None,
                name: None,
                avatar: None
            };

            println!("{:?}", member);

            let member_id_string = member.id.unwrap().to_hex();
            println!("{}", member_id_string);
            self.update_user_doc(patch_user, member_id_string).ok();
        }

        Ok(created_family)
    }
}
