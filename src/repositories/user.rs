use sqlx::{Pool, Postgres};

use crate::models::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn create_user(&self, user: &User) {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, name, provider)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            user.id,
            user.email,
            user.name,
            user.provider.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();
    }
}
