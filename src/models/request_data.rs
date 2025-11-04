use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthPayload {
    pub nonce: String,
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForNonce {
    pub pubkey: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct _ChangeNameUserRequest {
    pub public_key: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUserRequest {
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub public_key: String,
    pub username: Option<String>,
    pub rewards: Option<Vec<String>>,
    pub created_at: String,
    pub last_seen: String,
    pub banned: bool,
    pub ban_reason: Option<String>
}