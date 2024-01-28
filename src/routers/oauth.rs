use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{models::user::User, state::app::AppState};

#[derive(Deserialize)]
pub struct GoogleOAuth {
    code: String,
}

pub async fn google_oauth(
    oauth: Query<GoogleOAuth>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Option<User>>) {
    let details = state
        .google_oauth_client
        .perform_google_oauth(&oauth.code)
        .await;

    if let Err(e) = details {
        tracing::error!("Event=GoogleOAuthError error={:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    let details = details.unwrap();
    let user = User::from(details);
    let existing_user = state.user_repository.find_by_email(&user.email).await;

    if let Some(user) = existing_user {
        tracing::info!("Event=ExistingUser user={:?}", user);
        return (StatusCode::OK, Json(Some(user)));
    }

    state.user_repository.create_user(&user).await;
    tracing::info!("Event=CreatedUser user={:?}", user);
    (StatusCode::OK, Json(Some(user)))
}
