use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub name: String
}