use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

use crate::models::request_data::Claims;

pub fn generate_tokens(pubkey: &str) -> (String, String) {
    let access_claims = Claims {
        sub: pubkey.to_string(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    };
    let refresh_claims = Claims {
        sub: pubkey.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    let secret = b"secret-q1w2e3r4t5y6u7i8-key";

    let access = encode(&Header::default(), &access_claims, &EncodingKey::from_secret(secret)).unwrap();
    let refresh = encode(&Header::default(), &refresh_claims, &EncodingKey::from_secret(secret)).unwrap();

    (access, refresh)
}

pub fn verify_token(token: &str, secret: &str) -> bool {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .is_ok()
}