use actix_web::web;
use bs58;
use crate::{
    models::{request_data::AuthPayload},
    services::{ check_signer, generate_nonce, generate_tokens },
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use crate::models::request_data::Claims;
use crate::db_hooks;
use crate::models::config;

pub fn send_nonce_hendler() -> String {
    let nonce = generate_nonce();
    // Middleware to put nonce into response
    nonce
}

pub fn authentification_hendler(
    payload: &web::Json<AuthPayload>,
    cfg: web::Data<config::Config>
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let public_key_vec = bs58::decode(&payload.public_key).into_vec()?;
    let public_key_bytes: [u8; 32] = public_key_vec
        .try_into()
        .map_err(|_| "Invalid public key length")?;

    let signature_vec = bs58::decode(&payload.signature).into_vec()?;
    let signature_bytes: [u8; 64] = signature_vec
        .try_into()
        .map_err(|_| "Invalid signature length")?;

    match db_hooks::check_nonce_in_cache(&payload.nonce, &payload.public_key, &cfg.redis_url) {
        Ok(true) => (),
        Err(_) | Ok(false) => return Err("Nonce not found or does not match".into()),
    }
    let valid = check_signer(&payload.nonce, &public_key_bytes, &signature_bytes)?;
    
    db_hooks::reverse_flag(&payload.nonce, &cfg.redis_url)?;

    if valid {
        Ok(generate_tokens(&payload.public_key))
    } else {
        Err("Invalid signature".into())
    }
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