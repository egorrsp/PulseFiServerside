use actix_web::web;
use std::error::Error;
use bs58;
use crate::{models::request_data::AuthPayload, services::{check_signe, generate_nonce}};

pub fn authentification_hendler(payload: &web::Json<AuthPayload>) -> Result<bool, Box<dyn Error>> {

    let public_key_vec = bs58::decode(&payload.public_key).into_vec()?;
    let public_key_bytes: [u8; 32] = public_key_vec
        .try_into()
        .map_err(|_| "Invalid public key length")?;

    let signature_vec = bs58::decode(&payload.signature).into_vec()?;
    let signature_bytes: [u8; 64] = signature_vec
        .try_into()
        .map_err(|_| "Invalid signature length")?;

    check_signe(&payload.nonce, &public_key_bytes, &signature_bytes)
}

pub fn send_nonce_hendler() -> String {
    let nonce = generate_nonce();
    nonce
}