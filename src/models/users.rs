use crate::controllers::authentication::login_authorized::AuthedUser;
use crate::models::{family::Family, house::House};
use crate::repository::mongodb_repo::MongoRepo;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house: Option<House>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
}

#[derive(Deserialize)]
pub struct PatchUser {
    pub family: Option<Family>
}

impl MongoRepo {
    pub fn create_user(&self, data: &AuthedUser) -> Result<InsertOneResult, Error> {
        let AuthedUser { email, name, picture, .. } = data; 

        let filter = doc! {
            "email": email
        };

        if let Some(_) = self.user_collection.find_one(filter, None)? {
            return Err(Error::custom("User already exists"));
        }
        
        let user_data = User {
            name: name.to_string(),
            email: email.to_string(), 
            avatar: Some(picture.to_string()),
            id: None,
            family: None,
            house: None
        };

        let user = self.user_collection.insert_one(user_data, None);

        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::custom("error updating the user")),
        }
    }

    pub fn update_user_doc(
        &self,
        patch_user: PatchUser,
        user_id: String,
    ) -> Result<UpdateResult, Error> {
        let converted_object = ObjectId::parse_str(user_id);

        let user_object_id = match converted_object {
            Ok(user_object) => Ok(user_object),
            Err(e) => Err(e),
        };

        let PatchUser {
            family,
        } = patch_user;

        let mut update_doc = doc! {};

        if let Some(family) = family {
            update_doc.insert("family", to_bson(&family).unwrap());
        }

        let query = doc! { "_id": user_object_id.ok() };
        let update = doc! { "$set": update_doc };

        let updated_user = self.user_collection.update_one(query, update, None);

        match updated_user {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::custom("error updating the user")),
        }
    }

    pub fn get_user(&self, email: String) -> Result<Option<User>, Error> {
        let filter = doc! { "email": email };

        let user = self.user_collection.find_one(filter, None);

        match user {
            Ok(doc) => Ok(doc),
            Err(_) => Err(Error::custom("error getting user")),
        }
    }
}
