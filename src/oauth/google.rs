use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::info;

const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const TOKEN_INFO_URL: &str = "https://oauth2.googleapis.com/tokeninfo";

#[derive(Deserialize, Debug, Serialize)]
pub struct GoogleOAuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserDetails {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub email_verified: String,
    pub at_hash: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub locale: String,
    pub iat: String,
    pub exp: String,
}

#[derive(Debug)]
pub enum GoogleOAuthError {
    MissingEnvironmentVariable { name: String },
    RestRequestFailed,
    DeserializeFailed,
}

impl fmt::Display for GoogleOAuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoogleOAuthError::MissingEnvironmentVariable { name } => {
                write!(f, "Missing environement variable: {}", name)
            }
            GoogleOAuthError::RestRequestFailed => {
                write!(f, "Rest request failed")
            }
            GoogleOAuthError::DeserializeFailed => {
                write!(f, "Deserialize failed")
            }
        }
    }
}

#[derive(Clone)]
pub struct GoogleOAuthClient {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
pub struct TokenRequestPayload {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
    access_type: String,
}

fn safe_get_google_oauth_env_var(name: &str) -> Result<String, GoogleOAuthError> {
    std::env::var(name).map_err(|_e| GoogleOAuthError::MissingEnvironmentVariable {
        name: name.to_string(),
    })
}

impl GoogleOAuthClient {
    pub fn from_env() -> Result<GoogleOAuthClient, GoogleOAuthError> {
        let client_id = safe_get_google_oauth_env_var("GOOGLE_CLIENT_ID")?;
        let client_secret = safe_get_google_oauth_env_var("GOOGLE_CLIENT_SECRET")?;
        let redirect_uri = safe_get_google_oauth_env_var("GOOGLE_REDIRECT_URI")?;
        Ok(GoogleOAuthClient {
            client_id,
            client_secret,
            redirect_uri,
            client: reqwest::Client::new(),
        })
    }

    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> GoogleOAuthClient {
        GoogleOAuthClient {
            client_id,
            client_secret,
            redirect_uri,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_google_oauth_tokens_by_code(
        &self,
        code: &str,
    ) -> Result<GoogleOAuthTokens, GoogleOAuthError> {
        let resp = self
            .client
            .post(TOKEN_URL)
            .form(&TokenRequestPayload {
                code: code.to_string(),
                client_id: self.client_id.clone(),
                client_secret: self.client_secret.clone(),
                redirect_uri: self.redirect_uri.clone(),
                grant_type: "authorization_code".to_string(),
                access_type: "offline".to_string(),
            })
            .send()
            .await
            .map_err(|_e| GoogleOAuthError::RestRequestFailed)?
            .json::<GoogleOAuthTokens>()
            .await
            .map_err(|_e| GoogleOAuthError::DeserializeFailed)?;
        Ok(resp)
    }

    pub async fn get_google_oauth_token_details_by_id_token(
        &self,
        id_token: &str,
    ) -> Result<GoogleUserDetails, GoogleOAuthError> {
        let url = format!("{}?id_token={}", TOKEN_INFO_URL, id_token);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|_e| GoogleOAuthError::RestRequestFailed)?
            .json::<GoogleUserDetails>()
            .await
            .map_err(|_e| GoogleOAuthError::DeserializeFailed)?;
        Ok(resp)
    }

    pub async fn perform_google_oauth(
        &self,
        code: &str,
    ) -> Result<GoogleUserDetails, GoogleOAuthError> {
        let tokens = self.get_google_oauth_tokens_by_code(code).await?;
        let details = self
            .get_google_oauth_token_details_by_id_token(&tokens.id_token)
            .await?;
        info!("Event=GoogleUserAuthenticated name='{}'", details.name);
        Ok(details)
    }
}
