use sha2::{Sha256, Digest};

use crate::models::{
    request_data::UserResponse, 
    user_bd_struct::User
};

pub fn encode_nonce(nonce: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(nonce);
    hasher.finalize()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

pub fn serialize_uzer(user: User) -> UserResponse {
    UserResponse {
        public_key: user.public_key,
        username: user.username,
        rewards: user.rewards,
        created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        last_seen: user.last_seen.format("%Y-%m-%d %H:%M:%S").to_string(),
        banned: user.banned,
        ban_reason: user.ban_reason,
    }
}