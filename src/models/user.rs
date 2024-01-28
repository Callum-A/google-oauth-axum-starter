use crate::oauth::google::GoogleUserDetails;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum Provider {
    Google,
}

impl From<String> for Provider {
    fn from(s: String) -> Self {
        match s.as_str() {
            "google" => Self::Google,
            _ => panic!("Unknown provider"),
        }
    }
}

impl Provider {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Google => "google",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String, // UUID
    pub email: String,
    pub name: String,
    pub provider: Provider,
}

impl From<GoogleUserDetails> for User {
    fn from(details: GoogleUserDetails) -> Self {
        Self {
            id: Uuid::new_v4().into(),
            email: details.email,
            name: details.name,
            provider: Provider::Google,
        }
    }
}
