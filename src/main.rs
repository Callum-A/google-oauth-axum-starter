pub mod models;
pub mod oauth;

use crate::oauth::google::{perform_google_oauth, GoogleUserDetails};
use axum::{extract::Query, routing::get, Json, Router};
use dotenv::dotenv;
use serde::Deserialize;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());
    let app = Router::new().route("/api/v1/users/oauth/google", get(google_oauth));
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

async fn google_oauth(oauth: Query<GoogleOAuth>) -> Json<GoogleUserDetails> {
    let details = perform_google_oauth(&oauth.code).await.unwrap();
    Json(details)
}
