use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Clone)]
pub struct JWTClient {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl From<&User> for JWTClaims {
    fn from(user: &User) -> Self {
        Self {
            user_id: user.id.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTClaims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

impl JWTClient {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn encode(&self, claims: JWTClaims) -> String {
        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key).unwrap()
    }

    pub fn decode(&self, token: &str) -> Option<JWTClaims> {
        let claims = jsonwebtoken::decode::<JWTClaims>(
            token,
            &self.decoding_key,
            &jsonwebtoken::Validation::default(),
        )
        .map(|data| data.claims);

        match claims {
            Ok(claims) => Some(claims),
            Err(e) => {
                tracing::error!("Event=JWTDecodeError error={:?}", e);
                None
            }
        }
    }
}
