use crate::DbPool;
use shared::types::Profile;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct ProfileRow {
    id: i64,
    user_id: i64,
    display_name: String,
    bio: Option<String>,
    updated_at: String,
}

impl From<ProfileRow> for Profile {
    fn from(row: ProfileRow) -> Self {
        Profile {
            id: row.id,
            user_id: row.user_id,
            display_name: row.display_name,
            bio: row.bio,
            updated_at: row.updated_at,
        }
    }
}

pub async fn create_profile(
    pool: &DbPool,
    user_id: i64,
    display_name: &str,
) -> Result<Profile, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO profiles (user_id, display_name)
        VALUES (?, ?)
        "#,
    )
    .bind(user_id)
    .bind(display_name)
    .execute(pool)
    .await?;

    // Fetch the created profile
    get_profile_by_user_id(pool, user_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn get_profile_by_id(pool: &DbPool, id: i64) -> Result<Option<Profile>, sqlx::Error> {
    let row: Option<ProfileRow> = sqlx::query_as(
        r#"
        SELECT id, user_id, display_name, bio, updated_at
        FROM profiles
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Into::into))
}

pub async fn get_profile_by_user_id(
    pool: &DbPool,
    user_id: i64,
) -> Result<Option<Profile>, sqlx::Error> {
    let row: Option<ProfileRow> = sqlx::query_as(
        r#"
        SELECT id, user_id, display_name, bio, updated_at
        FROM profiles
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Into::into))
}

pub async fn list_profiles(pool: &DbPool) -> Result<Vec<Profile>, sqlx::Error> {
    let rows: Vec<ProfileRow> = sqlx::query_as(
        r#"
        SELECT id, user_id, display_name, bio, updated_at
        FROM profiles
        ORDER BY updated_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn update_profile(
    pool: &DbPool,
    id: i64,
    display_name: Option<&str>,
    bio: Option<&str>,
) -> Result<Option<Profile>, sqlx::Error> {
    // Only update if we have something to update
    if display_name.is_none() && bio.is_none() {
        return get_profile_by_id(pool, id).await;
    }

    // Build dynamic update query
    let mut query = String::from("UPDATE profiles SET updated_at = datetime('now')");

    if display_name.is_some() {
        query.push_str(", display_name = ?");
    }
    if bio.is_some() {
        query.push_str(", bio = ?");
    }
    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query);

    if let Some(name) = display_name {
        q = q.bind(name);
    }
    if let Some(b) = bio {
        q = q.bind(b);
    }
    q = q.bind(id);

    q.execute(pool).await?;

    get_profile_by_id(pool, id).await
}
