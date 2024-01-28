use sqlx::PgPool;

use crate::{
    oauth::google::GoogleOAuthClient, repositories::user::UserRepository, services::jwt::JWTClient,
};

#[derive(Clone)]
pub struct AppState {
    pub google_oauth_client: GoogleOAuthClient,
    pub jwt_client: JWTClient,
    pub pool: PgPool,
    pub user_repository: UserRepository,
}
