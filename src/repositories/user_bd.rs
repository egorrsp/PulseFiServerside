use sqlx::PgPool;
use actix_web::web;
use crate::models::{
    request_data::RegisterUserRequest, 
    user_bd_struct::User
};

// Add user to the database
pub async fn set_user(
    db: web::Data<PgPool>,
    payload: RegisterUserRequest,
) -> Result<bool, sqlx::Error> {
    let record = sqlx::query(
        r#"
        INSERT INTO users (public_key)
        VALUES ($1)
        ON CONFLICT (public_key) DO NOTHING
        "#
    )
    .bind(payload.public_key)
    .execute(db.as_ref())
    .await;

    match record {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
    
}

// Get user with public key
pub async fn get_user_by_public_key(
    db: web::Data<PgPool>,
    public_key: String,
) -> Result<User, sqlx::Error> {
    let record = sqlx::query_as::<_, User>(
        r#"
        SELECT public_key, username, rewards, created_at, lust_seen, banned, ban_reason
        FROM users
        WHERE public_key = $1
        "#
    )
    .bind(public_key)
    .fetch_one(db.get_ref())
    .await?;

    Ok(record)
}