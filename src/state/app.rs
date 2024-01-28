use sqlx::PgPool;

use crate::{oauth::google::GoogleOAuthClient, repositories::user::UserRepository};

#[derive(Clone)]
pub struct AppState {
    pub google_oauth_client: GoogleOAuthClient,
    pub pool: PgPool,
    pub user_repository: UserRepository,
}
