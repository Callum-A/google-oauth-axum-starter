use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{models::user::User, services::jwt::JWTClaims, state::app::AppState};

#[derive(Deserialize)]
pub struct GoogleOAuth {
    code: String,
}

#[derive(Serialize)]
pub struct OAuthResponse {
    access_token: String,
}

pub async fn google_oauth(
    oauth: Query<GoogleOAuth>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Option<OAuthResponse>>) {
    let details = state
        .google_oauth_client
        .perform_google_oauth(&oauth.code)
        .await;

    if let Err(e) = details {
        tracing::error!("Event=GoogleOAuthError error={:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    let details = details.unwrap();
    let google_user = User::from(details);
    let opt_user = state
        .user_repository
        .find_by_email(&google_user.email)
        .await;

    let user = match opt_user {
        Some(user) => user,
        None => {
            tracing::info!("Event=CreatedUser user={:?}", google_user);
            state.user_repository.create_user(&google_user).await;
            google_user
        }
    };

    let token = state.jwt_client.encode(JWTClaims::from(&user));
    tracing::info!("Event=AuthenticatingUser user={:?}", user);
    (
        StatusCode::OK,
        Json(Some(OAuthResponse {
            access_token: token,
        })),
    )
}
