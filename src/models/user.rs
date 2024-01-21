use crate::oauth::google::GoogleUserDetails;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid, // UUID
    pub email: String,
    pub name: String,
}

impl User {
    pub fn new(email: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            name,
        }
    }
}

impl From<GoogleUserDetails> for User {
    fn from(details: GoogleUserDetails) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: details.email,
            name: details.name,
        }
    }
}
