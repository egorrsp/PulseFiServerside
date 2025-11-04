use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub public_key: String,
    pub username: Option<String>,
    pub rewards: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub banned: bool,
    pub ban_reason: Option<String>,
}