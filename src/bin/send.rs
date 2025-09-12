use haveno::generated::io_haveno_protobuffer::StoragePayload;
use haveno::builders::{add_data, filter};
use haveno::utils::network::envelope::build_envelope;
use anyhow::Result;
use haveno::utils::network::envelope::{send_envelope, recv_envelope, EnvMsg};
use tokio_socks::tcp::Socks5Stream;
use haveno::generated::io_haveno_protobuffer::{
    NetworkEnvelope, PreliminaryGetDataRequest, storage_payload::Message as PayloadMessage,
};

/// this should be trigger as part of an event onApplicationStart() but as part of a routine on a thread for other stuff and runs as daemon
#[tokio::main]
async fn main() -> Result<()> {
    let onion_addr = "cylylutdwlnc4ml3isimvlmfotsgizmpyl2lz65hpvretxfbpwktt2qd.onion:80";
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

    // Now build and send filter object
    let filter = filter::build_signed_filter().await?;

    // Wrap it in the PayloadMessage enum
    let payload = PayloadMessage::Filter(filter);

    // Build AddDataMessage from it
    let signed_add_data_message = add_data::build_signed_add_data_message(StoragePayload {
        message: Some(payload),
    }).await?;

    // Wrap it in a NetworkEnvelope
    let network_envelope = build_envelope(EnvMsg::AddDataMessage(signed_add_data_message));

    // Send
    send_envelope(&mut stream, &network_envelope).await?;
    println!("📤 Sent AddDataMessage with Filter");

    Ok(())
}