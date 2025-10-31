use sha2::{Sha256, Digest};

pub fn encode_nonce(nonce: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(nonce);
    hasher.finalize()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}