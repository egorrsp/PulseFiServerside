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
pub struct RegisterUserRequest {
    pub public_key: String,
    pub username: String,
}