use base64::engine::general_purpose;
use base64::Engine;
use k256::ecdsa::signature::SignerMut;
use k256::ecdsa::Signature;
use prost::Message;
use sha2::{Digest, Sha256};
use anyhow::Result;

use crate::utils::signing::{get_pubkey_hex, load_signing_key};
use crate::generated::io_haveno_protobuffer::Filter;

fn build_filter() -> Filter {
    Filter {
        node_addresses_banned_from_trading: vec![],
        banned_offer_ids: vec![],
        banned_payment_accounts: vec![],
        signature_as_base64: "".to_string(),
        owner_pub_key_bytes: vec![],
        extra_data: Default::default(),
        banned_currencies: vec![],
        banned_payment_methods: vec![],
        arbitrators: vec![],
        seed_nodes: vec![],
        price_relay_nodes: vec![],
        prevent_public_xmr_network: false,
        xmr_nodes: vec![],
        disable_trade_below_version: "".to_string(),
        mediators: vec![],
        refund_agents: vec![],
        banned_signer_pub_keys: vec![],
        xmr_fee_receiver_addresses: vec![],
        creation_date: chrono::Utc::now().timestamp(),
        signer_pub_key_as_hex: "".to_string(),
        banned_privileged_dev_pub_keys: vec![],
        disable_auto_conf: false,
        banned_auto_conf_explorers: vec![],
        node_addresses_banned_from_network: vec![],
        disable_api: false,
        disable_mempool_validation: false,
    }
}

/// Generate a base64 signature over the SHA-256 of the serialized Filter
async fn sign_filter(filter: &Filter) -> Result<(String, String)> {
    let mut buf = Vec::new();
    filter.encode(&mut buf)?;

    let hash = Sha256::digest(&buf);
    let mut signing_key = load_signing_key().await?;
    let signature: Signature = signing_key.sign(&hash);

    let signature_base64 = general_purpose::STANDARD.encode(signature.to_der());
    let pubkey_hex = get_pubkey_hex(&signing_key);
    Ok((signature_base64, pubkey_hex))
}

pub async fn build_signed_filter() -> Result<Filter, anyhow::Error> {
    let mut filter = build_filter();
    
    // Sign the filter and get the signature and public key
    let (signature_base64, pubkey_hex) = sign_filter(&filter).await?;
    
    // Set the signature and public key in the filter
    filter.signature_as_base64 = signature_base64;
    filter.owner_pub_key_bytes = hex::decode(pubkey_hex).expect("Invalid hex public key");
    
    Ok(filter)
}