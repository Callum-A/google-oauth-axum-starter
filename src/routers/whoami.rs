use axum::{http::StatusCode, Extension, Json};

use crate::services::jwt::JWTClaims;

pub async fn whoami(Extension(user): Extension<JWTClaims>) -> (StatusCode, Json<JWTClaims>) {
    tracing::info!("Event=WhoAmI user={:?}", user);
    (StatusCode::OK, Json(user))
}
