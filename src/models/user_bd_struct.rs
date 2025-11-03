use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub public_key: String,
    pub username: Option<String>,
    pub rewards: Option<Vec<String>>,
    pub created_at: String,
    pub lust_seen: String,
    pub banned: bool,
    pub ban_reason: Option<String>,
}