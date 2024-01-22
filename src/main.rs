pub mod models;
pub mod oauth;

use crate::models::user::User;
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use serde::Deserialize;
use tracing_subscriber::layer::SubscriberExt;

#[derive(Clone)]
struct AppState {
    google_oauth_client: oauth::google::GoogleOAuthClient,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());

    let google_oauth_client = oauth::google::GoogleOAuthClient::from_env().unwrap();
    let app_state = AppState {
        google_oauth_client,
    };
    let app = Router::new()
        .route("/api/v1/users/oauth/google", get(google_oauth))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct GoogleOAuth {
    code: String,
}

async fn google_oauth(oauth: Query<GoogleOAuth>, State(state): State<AppState>) -> Json<User> {
    let details = state
        .google_oauth_client
        .perform_google_oauth(&oauth.code)
        .await
        .unwrap();
    let user = User::from(details);
    tracing::info!("Event=CreatedUser user={:?}", user);
    Json(user)
}
