pub mod models;
pub mod oauth;
pub mod repositories;
pub mod routers;
pub mod state;

use crate::{models::user::User, oauth::google::GoogleOAuthClient, state::app::AppState};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use repositories::user::UserRepository;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    // Setup env
    dotenv().ok();

    // Setup tracing and logging
    tracing_subscriber::fmt().init();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());

    // Setup DB pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // App state (pool.clone is cheap)
    let google_oauth_client = GoogleOAuthClient::from_env().unwrap();
    let app_state = AppState {
        google_oauth_client,
        user_repository: UserRepository::new(pool.clone()),
        pool,
    };

    // Start server
    let app = Router::new()
        .route("/api/v1/health_check", get(routers::debug::health_check))
        .route(
            "/api/v1/users/oauth/google",
            get(routers::oauth::google_oauth),
        )
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
