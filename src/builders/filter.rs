use openssl::bn::BigNum;
use openssl::dsa::Dsa;
use openssl::pkey::PKey;
use openssl::sha::sha256;
use openssl::sign::Signer;
use prost::Message;
use anyhow::{Result};
use base64::{engine::general_purpose, Engine as _};
use anyhow::{anyhow};

use crate::utils::signing::{load_private_key};
use crate::generated::io_haveno_protobuffer::{Filter};
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
/// Load the private key from config and sign the filter
pub async fn sign_filter(filter: &Filter) -> Result<(String, Vec<u8>)> {
    // Serialize Filter to bytes
    let mut buf = Vec::new();
    filter.encode(&mut buf); // Assume success

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

pub async fn build_signed_filter() -> Result<Filter> {
    let mut filter = build_filter();
    
    // Sign the filter and get the signature and public key
    let (signature_base64, pubkey) = sign_filter(&filter).await?;
    
    // Set the signature and public key in the filter
    filter.signature_as_base64 = signature_base64;
    //filter.owner_pub_key_bytes = hex::decode(pubkey_hex).expect("Invalid hex public key");
    filter.owner_pub_key_bytes = pubkey.clone();

    Ok(filter)
}