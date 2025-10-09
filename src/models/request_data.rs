use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthPayload {
    pub nonce: String,
    pub public_key: String,
    pub signature: String,
}