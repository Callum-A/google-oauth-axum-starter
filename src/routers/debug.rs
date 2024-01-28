use axum::{extract::State, http::StatusCode};
use sqlx::Error;

use crate::state::app::AppState;

pub async fn health_check(State(state): State<AppState>) -> (StatusCode, &'static str) {
    let row: Result<(i32,), Error> = sqlx::query_as("SELECT $1")
        .bind(1_i32)
        .fetch_one(&state.pool)
        .await;
    tracing::info!("Event=HealthCheck row={:?}", row);
    match row {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "NOK"),
    }
}
