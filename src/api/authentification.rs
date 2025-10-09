use actix_web::web;
use bs58;
use crate::{models::request_data::AuthPayload, services::{check_signe, generate_nonce, generate_tokens}};

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