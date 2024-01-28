pub mod middleware;
pub mod models;
pub mod oauth;
pub mod repositories;
pub mod routers;
pub mod services;
pub mod state;

use crate::{oauth::google::GoogleOAuthClient, services::jwt::JWTClient, state::app::AppState};
use axum::{routing::get, Router};
use dotenv::dotenv;
use repositories::user::UserRepository;
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
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let jwt_client = JWTClient::new(&jwt_secret);
    let app_state = AppState {
        google_oauth_client,
        jwt_client,
        user_repository: UserRepository::new(pool.clone()),
        pool,
    };

    // Routes
    let protected_routes = Router::new()
        .route("/api/v1/whoami", get(routers::whoami::whoami))
        .with_state(app_state.clone())
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth::authenticate,
        ));

    let unprotected_routes = Router::new()
        .route(
            "/api/v1/users/oauth/google",
            get(routers::oauth::google_oauth),
        )
        .route("/api/v1/health_check", get(routers::debug::health_check))
        .with_state(app_state.clone());

    // Merge the two to give us our full app router
    let app = Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes);

    // Start Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    tracing::info!(
        "Event=ServerStart local_addr={:?}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
