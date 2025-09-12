use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier, SECRET_KEY_LENGTH};
use sha3::{Digest, Sha3_256};
use rand::rngs::OsRng;
use data_encoding::BASE32_NOPAD;

pub struct OnionIdentity {
    pub keypair: Keypair,
}

pub fn generate_keypair() -> Keypair {
    let mut csprng = OsRng;
    Keypair::generate(&mut csprng)
}

pub fn get_onion_address(public_key: &PublicKey) -> String {
    let pub_bytes = public_key.to_bytes();

    // Checksum = H(".onion checksum" || pubkey || version)[:2]
    let mut hasher = Sha3_256::new();
    hasher.update(b".onion checksum");
    hasher.update(&pub_bytes);
    hasher.update(&[0x03]); // version byte
    let hash = hasher.finalize();
    let checksum = &hash[0..2];

    // onion_address = BASE32(pubkey || checksum || version)
    let mut onion_bytes = vec![];
    onion_bytes.extend_from_slice(&pub_bytes);
    onion_bytes.extend_from_slice(checksum);
    onion_bytes.push(0x03); // version

    BASE32_NOPAD.encode(&onion_bytes).to_lowercase() + ".onion"
}

pub fn sign_nonce(secret_key_bytes: &[u8; 32], nonce: &[u8]) -> Vec<u8> {
    let secret = SecretKey::from_bytes(secret_key_bytes).expect("Invalid secret");
    let public = PublicKey::from(&secret);
    let keypair = Keypair { secret, public };
    keypair.sign(nonce).to_bytes().to_vec()
}

pub fn verify_signature(onion_address: &str, nonce: &[u8], signature_bytes: &[u8]) -> bool {
    // Extract pubkey from onion address
    let addr = onion_address.trim_end_matches(".onion").to_uppercase();
    let decoded = BASE32_NOPAD.decode(addr.as_bytes()).expect("Invalid base32");

    if decoded.len() < 32 {
        return false;
    }

    let pubkey_bytes = &decoded[0..32];
    let public_key = PublicKey::from_bytes(pubkey_bytes).expect("Invalid public key");
    let signature = Signature::from_bytes(signature_bytes).expect("Invalid signature");

    public_key.verify(nonce, &signature).is_ok()
}