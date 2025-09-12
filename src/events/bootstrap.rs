use anyhow::Result;
use crate::{
    generated::{self, io_haveno_protobuffer::{
        network_envelope::Message as EnvMsg,
        storage_payload::Message as PayloadMessage,
        NetworkEnvelope, PreliminaryGetDataRequest, StoragePayload
    }}, builders::{add_data, filter}, p2p::{handlers::add_data::AddDataMessageHandler, router::P2PMessageRouter}, utils::{
        network::{
            ack, envelope::{build_envelope, recv_envelope, send_envelope}, updated_data
        },
        signing
    }
};
use tokio_socks::tcp::Socks5Stream;

/// Entry point to run the seed bootstrap procedure
pub async fn run_seed_bootstrap() -> Result<()> {

    // Start the network listener
    let router = P2PMessageRouter::new();
    router.register("AddDataMessage", *Box::new(AddDataMessageHandler)).await;


    let onion_addr = "5i6blbmuflq4s4im6zby26a7g22oef6kyp7vbwyru6oq5e36akzo3ayd.onion:2001";
    let mut stream = Socks5Stream::connect("127.0.0.1:9050", onion_addr)
        .await?
        .into_inner();

    println!("\n🧅 Connected to {onion_addr}");

    // Build and send the PreliminaryGetDataRequest
    let request_env = NetworkEnvelope {
        message_version: "0X".into(),
        message: Some(EnvMsg::PreliminaryGetDataRequest(PreliminaryGetDataRequest {
            nonce: 4232,
            excluded_keys: vec![],
            version: "1.1.2".to_string(),
            supported_capabilities: vec![11, 14, 16],
        })),
    };

    send_envelope(&mut stream, &request_env).await?;
    println!("📤 Sent PreliminaryGetDataRequest");

    // Wait for and process GetDataResponse
    if let Some(envelope) = recv_envelope(&mut stream).await? {
        match &envelope.message {
            Some(EnvMsg::GetDataResponse(resp)) => {
                println!("📥 Received GetDataResponse:");
                println!("  Request nonce: {}", resp.request_nonce);
                println!("  Supported capabilities: {:?}", resp.supported_capabilities);
                println!("  Persistable network payloads: {:?}", resp.persistable_network_payload_items);
            }
            Some(other) => println!("⚠️ Unexpected envelope message: {:?}", other),
            None => println!("⚠️ Envelope has no message field."),
        }
    }

    // Must now GetDataUpdateRequest
    let get_updated_data = updated_data::build_get_updated_data().await?;
    let update_data_network_envelope = build_envelope(EnvMsg::GetUpdatedDataRequest(get_updated_data));
    send_envelope(&mut stream, &update_data_network_envelope).await?;
    println!("📤 Sent GetUpdatedDataRequest");

    // Must now response with AckMessage before sending anything else
    let ack_envelope = ack::build_ack().await?;
    let ack_network_envelope = build_envelope(EnvMsg::AckMessage(ack_envelope));
    send_envelope(&mut stream, &ack_network_envelope).await?;
    println!("📤 Sent AckMessage");


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