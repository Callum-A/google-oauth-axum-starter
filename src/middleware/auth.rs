use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::state::app::AppState;

pub async fn authenticate(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers.get("Authorization");
    if auth_header.is_none() {
        tracing::info!("Event=AuthHeaderMissing");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth_header = auth_header.unwrap();
    let auth_header = auth_header.to_str().unwrap();
    let token = auth_header.replace("Bearer ", "");
    let claims = state.jwt_client.decode(&token);
    if claims.is_none() {
        tracing::info!("Event=JWTDecodeError error={:?}", claims);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let claims = claims.unwrap();
    tracing::info!("Event=AuthenticatedUser user={:?}", claims);
    request.extensions_mut().insert(claims);
    let response = next.run(request).await;
    Ok(response)
}
