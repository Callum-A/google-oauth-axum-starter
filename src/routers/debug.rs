use axum::extract::State;
use sqlx::Error;

use crate::state::app::AppState;

pub async fn health_check(State(state): State<AppState>) -> &'static str {
    let row: Result<(i32,), Error> = sqlx::query_as("SELECT $1")
        .bind(1_i32)
        .fetch_one(&state.pool)
        .await;
    match row {
        Ok(_) => "OK",
        Err(_) => "NOK",
    }
}
