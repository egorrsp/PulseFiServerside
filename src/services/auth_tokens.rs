use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

use crate::models::request_data::Claims;

pub fn generate_tokens(pubkey: &str, jwt_secret: &str) -> (String, String) {
    let access_claims = Claims {
        sub: pubkey.to_string(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    };
    let refresh_claims = Claims {
        sub: pubkey.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    let access = encode(&Header::default(), &access_claims, &EncodingKey::from_secret(jwt_secret.as_bytes())).unwrap();
    let refresh = encode(&Header::default(), &refresh_claims, &EncodingKey::from_secret(jwt_secret.as_bytes())).unwrap();

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

pub fn refresh_tokens(refresh_token: &str, jwt_secret: &str) -> Option<(String, String)> {
    
    let decoded = decode::<Claims>(
        refresh_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).ok()?;

    let pubkey = decoded.claims.sub;

    let now = Utc::now().timestamp() as usize;
    if decoded.claims.exp < now {
        return None; // токен просрочен
    }

    let access_claims = Claims {
        sub: pubkey.clone(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    };

    let refresh_claims = Claims {
        sub: pubkey.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    let access = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).ok()?;

    let refresh = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).ok()?;

    Some((access, refresh))
}