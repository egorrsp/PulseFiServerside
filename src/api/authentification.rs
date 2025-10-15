use actix_web::web;
use bs58;
use crate::{
    models::request_data::AuthPayload,
    services::{ check_signe, generate_nonce, generate_tokens },
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use crate::models::request_data::Claims;

pub fn send_nonce_hendler() -> String {
    let nonce = generate_nonce();
    nonce
}

pub fn authentification_hendler(
    payload: &web::Json<AuthPayload>
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let public_key_vec = bs58::decode(&payload.public_key).into_vec()?;
    let public_key_bytes: [u8; 32] = public_key_vec
        .try_into()
        .map_err(|_| "Invalid public key length")?;

    let signature_vec = bs58::decode(&payload.signature).into_vec()?;
    let signature_bytes: [u8; 64] = signature_vec
        .try_into()
        .map_err(|_| "Invalid signature length")?;

    let valid = check_signe(&payload.nonce, &public_key_bytes, &signature_bytes)?;

    if valid {
        Ok(generate_tokens(&payload.public_key))
    } else {
        Err("Invalid signature".into())
    }
}

const SECRET: &[u8] = b"secret-q1w2e3r4t5y6u7i8-key";

pub fn refresh_tokens(refresh_token: &str) -> Option<(String, String)> {
    
    let decoded = decode::<Claims>(
        refresh_token,
        &DecodingKey::from_secret(SECRET),
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
        &EncodingKey::from_secret(SECRET),
    ).ok()?;

    let refresh = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(SECRET),
    ).ok()?;

    Some((access, refresh))
}