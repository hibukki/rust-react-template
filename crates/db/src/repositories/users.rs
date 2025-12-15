use crate::DbPool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

pub async fn create_user(
    pool: &DbPool,
    email: &str,
    password_hash: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES (?, ?)
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_user_by_email(pool: &DbPool, email: &str) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, email, password_hash, created_at
        FROM users
        WHERE email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_id(pool: &DbPool, id: i64) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, email, password_hash, created_at
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
