use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{models::user::User, state::app::AppState};

#[derive(Deserialize)]
pub struct GoogleOAuth {
    code: String,
}

pub async fn google_oauth(oauth: Query<GoogleOAuth>, State(state): State<AppState>) -> Json<User> {
    let details = state
        .google_oauth_client
        .perform_google_oauth(&oauth.code)
        .await
        .unwrap();
    let user = User::from(details);
    let existing_user = state.user_repository.find_by_email(&user.email).await;

    if let Some(user) = existing_user {
        tracing::info!("Event=ExistingUser user={:?}", user);
        return Json(user);
    }

    state.user_repository.create_user(&user).await;
    tracing::info!("Event=CreatedUser user={:?}", user);
    Json(user)
}
