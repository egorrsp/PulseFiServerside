use sqlx::PgPool;
use chrono::prelude::*;
use actix_web::web;

use crate::models::{
    request_data::ChangeNameUserRequest, 
    user_bd_struct::User
};

// Add user to the database
pub async fn set_user(
    db: web::Data<PgPool>,
    payload: String,
) -> Result<bool, sqlx::Error> {
    let record = sqlx::query(
        r#"
        INSERT INTO users (public_key)
        VALUES ($1)
        ON CONFLICT (public_key) DO NOTHING
        "#
    )
    .bind(payload)
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
    let record1 = sqlx::query_as::<_, User>(
        r#"
        SELECT public_key, username, rewards, created_at, last_seen, banned, ban_reason
        FROM users
        WHERE public_key = $1
        "#
    )
    .bind(&public_key)
    .fetch_optional(db.get_ref())
    .await?;

    let record2 = set_new_time(db, &public_key).await;

    if let Some(user) = record1 {
        match record2 {
            Ok(_) => Ok(user),
            Err(e) => Err(e)
        }
    }
    else {
        Err(sqlx::Error::RowNotFound)
    }
}

// Change username
pub async fn change_username_from_bd(
    db: web::Data<PgPool>,
    request: ChangeNameUserRequest,
) -> Result<bool, sqlx::Error> {
    let record = sqlx::query(
        r#"
        UPDATE users
        SET username = $1
        WHERE public_key = $2
        "#)
        .bind(request.newUsername)
        .bind(request.pubkey)
        .execute(db.as_ref())
        .await;

    match record {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

pub async fn set_new_time(
    db: web::Data<PgPool>,
    public_key: &String
) -> Result<bool, sqlx::Error> {

    let time_now: DateTime<Utc> = Utc::now();

    let record = sqlx::query(
        r#"
        UPDATE users
        SET last_seen = $1
        WHERE public_key = $2
        "#)
        .bind(time_now)
        .bind(public_key)
        .execute(db.as_ref())
        .await;

    match record {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}