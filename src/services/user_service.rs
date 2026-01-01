use sqlx::PgPool;

use crate::models::user::User;

pub async fn create_user(
    db: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email, created_at
        "#,
        email,
        password_hash
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}
