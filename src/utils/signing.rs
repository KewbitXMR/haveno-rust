use anyhow::{Result, Context};
use std::fs;
use base64::{engine::general_purpose, Engine as _};
use k256::{
    ecdsa::{SigningKey},
    EncodedPoint,
};
use crate::utils::config;

/// Load secret key from config.json and return a SigningKey
pub async fn load_signing_key() -> Result<SigningKey> {
    let json = fs::read_to_string("config.json")?;
    let config: config::Config = serde_json::from_str(&json)?;
    let secret_vec = general_purpose::STANDARD
        .decode(config.secret.trim())
        .context("Failed to decode base64 secret")?;

    // Ensure it's exactly 32 bytes
    let secret_bytes: [u8; 32] = secret_vec
        .try_into()
        .map_err(|_| anyhow::anyhow!("Secret key must be exactly 32 bytes"))?;

    let signing_key = SigningKey::from_bytes(&secret_bytes.into())?;
    Ok(signing_key)
}

/// Derive compressed public key as hex
pub fn get_pubkey_hex(signing_key: &SigningKey) -> String {
    let verifying_key = signing_key.verifying_key();
    let encoded = EncodedPoint::from(verifying_key).to_bytes(); // Compressed
    hex::encode(encoded)
}