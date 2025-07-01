use haveno_basic_bootstrap::{messages::filter};
use anyhow::Result;
use haveno_basic_bootstrap::utils::network::envelope::{send_envelope, recv_envelope, build_envelope, EnvMsg};
use haveno_basic_bootstrap::utils::network::add_data_message;
use tokio_socks::tcp::Socks5Stream;
use haveno_basic_bootstrap::utils::signing;
use haveno_basic_bootstrap::generated::io_haveno_protobuffer::{
    NetworkEnvelope, PreliminaryGetDataRequest
};

/// this should be trigger as part of an event onApplicationStart() but as part of a routine on a thread for other stuff and runs as daemon
#[tokio::main]
async fn main() -> Result<()> {
    let onion_addr = "5i6blbmuflq4s4im6zby26a7g22oef6kyp7vbwyru6oq5e36akzo3ayd.onion:2001";
    let mut stream = Socks5Stream::connect("127.0.0.1:9050", onion_addr)
        .await?.into_inner();
    println!("🧅 Connected to {onion_addr}");

    let request_env = NetworkEnvelope {
        message_version: "0X".into(),
        message: Some(EnvMsg::PreliminaryGetDataRequest(PreliminaryGetDataRequest {
            nonce: 4232,
            excluded_keys: vec![],
            version: "1.1.2".to_string(),
            supported_capabilities: [11,14,16].to_vec(),
        })),
    };

    send_envelope(&mut stream, &request_env).await?;
    println!("📤 Sent PreliminaryGetDataRequest");

    match recv_envelope(&mut stream).await? {
        Some(NetworkEnvelope { message: Some(EnvMsg::GetDataResponse(resp)), .. }) => {
            println!("📥 Received GetDataResponse:");
            println!("  Request nonce: {}", resp.request_nonce);
            println!("  Supported capabilities: {:?}", resp.supported_capabilities);
            println!("  Persistable network payloads: {:?}", resp.persistable_network_payload_items);
        }
        Some(other) => {
            println!("⚠️ Unexpected envelope message: {:?}", other);
        }
        None => {
            println!("⚠️ No response received in time.");
        }
    }

    let filter = filter::build_signed_filter().await?;
    let add_data_message = add_data_message::build_add_data_message(filter.clone(), &signing::load_signing_key().await?).await;
    let network_envelope = build_envelope(EnvMsg::AddDataMessage(add_data_message));
    send_envelope(&mut stream, &network_envelope).await?;

    Ok(())
}