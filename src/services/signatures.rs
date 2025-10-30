use ed25519_dalek::{Signature, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use std::error::Error;

pub fn check_signer(
    nonce: &str,
    public_key_bytes: &[u8; 32],
    signature_bytes: &[u8; 64],
) -> Result<bool, Box<dyn Error>> {
    let verifying_key = VerifyingKey::from_bytes(public_key_bytes)?;
    let signature = Signature::from_bytes(signature_bytes);

    Ok(verifying_key.verify_strict(nonce.as_bytes(), &signature).is_ok())
}

pub fn generate_nonce() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}