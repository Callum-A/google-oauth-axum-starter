use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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

fn safe_get_google_oauth_env_var(name: &str) -> Result<String, GoogleOAuthError> {
    std::env::var(name).map_err(|_e| GoogleOAuthError::MissingEnvironmentVariable {
        name: name.to_string(),
    })
}

pub async fn get_google_oauth_tokens_by_code(
    code: &String,
) -> Result<GoogleOAuthTokens, GoogleOAuthError> {
    let url = "https://oauth2.googleapis.com/token";
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("code", code);

    let client_id = safe_get_google_oauth_env_var("GOOGLE_CLIENT_ID")?;
    params.insert("client_id", &client_id);
    let client_secret = safe_get_google_oauth_env_var("GOOGLE_CLIENT_SECRET")?;
    params.insert("client_secret", &client_secret);
    let redirect_uri = safe_get_google_oauth_env_var("GOOGLE_REDIRECT_URI")?;
    params.insert("redirect_uri", &redirect_uri);
    params.insert("grant_type", "authorization_code");
    params.insert("access_type", "offline");
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|_e| GoogleOAuthError::RestRequestFailed)?
        .json::<GoogleOAuthTokens>()
        .await
        .map_err(|_e| GoogleOAuthError::DeserializeFailed)?;
    Ok(resp)
}

pub async fn get_google_oauth_token_details_by_id_token(
    id_token: &String,
) -> Result<GoogleUserDetails, GoogleOAuthError> {
    let url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        id_token
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|_e| GoogleOAuthError::RestRequestFailed)?
        .json::<GoogleUserDetails>()
        .await
        .map_err(|_e| GoogleOAuthError::DeserializeFailed)?;
    Ok(resp)
}

pub async fn perform_google_oauth(code: &String) -> Result<GoogleUserDetails, GoogleOAuthError> {
    let tokens = get_google_oauth_tokens_by_code(code).await?;
    let details = get_google_oauth_token_details_by_id_token(&tokens.id_token).await?;
    Ok(details)
}
