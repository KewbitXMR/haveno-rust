use base64::{engine::general_purpose, Engine};
use anyhow::{anyhow, Result};
use openssl::{bn::BigNum, dsa::Dsa, pkey::PKey, sha::sha256, sign::Signer};
use prost::Message;

use crate::{generated::io_haveno_protobuffer::{
    storage_entry_wrapper::Message as WrapperMessage, storage_payload::Message as PayloadMessage, AddDataMessage, Filter, ProtectedStorageEntry, StorageEntryWrapper, StoragePayload
}, utils::signing::{decode_signature_base64, load_private_key}};

pub async fn build_add_data_message(payload: PayloadMessage) -> Result<AddDataMessage> {
    // Wrap the enum payload into StoragePayload
    let storage_payload = StoragePayload {
        message: Some(payload.clone()),
    };

    // Sign and get signature + public key
    let (signature_base64, pubkey) = sign_storage_payload(storage_payload.clone()).await?;

    let payload_signature_bytes = decode_signature_base64(&signature_base64).await?;

    let protected_entry = ProtectedStorageEntry {
        storage_payload: Some(storage_payload),
        owner_pub_key_bytes: pubkey.clone(),
        sequence_number: 1,
        signature: payload_signature_bytes,
        creation_time_stamp: chrono::Utc::now().timestamp_millis(),
    };

    let wrapper = StorageEntryWrapper {
        message: Some(WrapperMessage::ProtectedStorageEntry(protected_entry)),
    };

    Ok(AddDataMessage {
        entry: Some(wrapper),
    })
}

pub async fn sign_storage_payload(storage_payload: StoragePayload) -> Result<(String, Vec<u8>)> {
    // Serialize Filter to bytes
    let mut buf = Vec::new();
    storage_payload.encode(&mut buf); // Assume success

    // SHA-256 digest
    let digest = sha256(&buf);

    // Load raw 32-byte private key (hex format from config)
    let hex_key = load_private_key().await?;
    let priv_bytes: [u8; 32] = hex::decode(hex_key.trim())
        .map_err(|e| anyhow!("Invalid hex key: {}", e))?
        .try_into()
        .map_err(|_| anyhow!("Expected 32-byte key"))?;

    // Generate DSA domain params (must match Java side exactly in production)
    let dsa = Dsa::generate(2048)?; // (p, q, g)

    // Inject raw private key into DSA structure
    let priv_bn = BigNum::from_slice(&priv_bytes)?;
    let dsa = Dsa::from_private_components(
        dsa.p().to_owned()?,
        dsa.q().to_owned()?,
        dsa.g().to_owned()?,
        priv_bn,
        dsa.pub_key().to_owned()?,
    )?;

    // Create PKey from DSA
    let pkey = PKey::from_dsa(dsa)?;

    // Sign the digest manually using SHA256withDSA
    let mut signer = Signer::new_without_digest(&pkey)?;
    signer.update(&digest)?;
    let signature = signer.sign_to_vec()?;

    // Export public key as DER (X.509 format for Java)
    let pubkey_der = pkey.public_key_to_der()?;

    Ok((general_purpose::STANDARD.encode(signature), pubkey_der))
}

pub async fn build_signed_add_data_message(storage_payload: StoragePayload) -> Result<AddDataMessage> {
    // Extract the enum variant from the storage payload
    let payload_msg = match storage_payload.message.clone() {
        Some(msg) => msg,
        None => return Err(anyhow!("StoragePayload has no message")),
    };

    // Re-use your existing builder function
    let add_data_message = build_add_data_message(payload_msg).await?;

    Ok(add_data_message)
}