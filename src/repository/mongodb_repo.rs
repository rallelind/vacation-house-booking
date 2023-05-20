use crate::models::{
    family::Family,
    house::{Booking, BookingPost, House},
    session::Session,
    users::User,
};
use std::env::var;

use mongodb::sync::{Client, Collection};
#[derive(Clone, Debug)]
pub struct MongoRepo {
    pub user_collection: Collection<User>,
    pub house_collection: Collection<House>,
    pub booking_collection: Collection<Booking>,
    pub booking_post_collection: Collection<BookingPost>,
    pub family_collection: Collection<Family>,
    pub session_collection: Collection<Session>,
}

impl MongoRepo {
    pub fn init() -> Self {
        let mongo_connection_string =
            var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");

        let client =
            Client::with_uri_str(mongo_connection_string).expect("error connection to client");

        let db = client.database("house_booking");
        let user_collection: Collection<User> = db.collection("Users");
        let house_collection: Collection<House> = db.collection("House");
        let booking_collection: Collection<Booking> = db.collection("Booking");
        let booking_post_collection: Collection<BookingPost> = db.collection("BookingPost");
        let family_collection: Collection<Family> = db.collection("Family");
        let session_collection: Collection<Session> = db.collection("Session");

        MongoRepo {
            user_collection,
            house_collection,
            booking_collection,
            booking_post_collection,
            family_collection,
            session_collection,
        }
    }
}
