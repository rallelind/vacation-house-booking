use crate::repository::mongodb_repo::MongoRepo;
use mongodb::{
    bson::{oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult},
    error::Error
};
use serde::{Deserialize, Serialize};
use crate::controllers::users::update_user::PatchUser;
use crate::models::house::House;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub name: String,
    pub house: Option<House>
}

impl MongoRepo {
    pub fn create_user(&self, user_data: User) -> Result<InsertOneResult, Error> {
        let user = self
            .user_collection
            .insert_one(user_data, None)
            .ok()
            .expect("Error creating user");
        
        Ok(user)
    }

    pub fn update_user_doc(&self, patch_user: PatchUser, user_id: String) -> Result<UpdateResult, Error> {
        let user_object_id = ObjectId::parse_str(user_id).expect("error reading objectId");

        let PatchUser { email, name, avatar } = patch_user;
        
        let mut update_doc = doc!{};

        if let Some(email) = email {
            update_doc.insert("email", email);
        }

        if let Some(name) = name {
            update_doc.insert("name", name);
        }

        if let Some(avatar) = avatar {
            update_doc.insert("avatar", avatar);
        }

        let query = doc! { "_id": user_object_id };
        let update = doc! { "$set": update_doc };

        let updated_user = self.user_collection.update_one(query, update, None);

        match updated_user {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::custom("error updating the user"))
        }
    }
}
