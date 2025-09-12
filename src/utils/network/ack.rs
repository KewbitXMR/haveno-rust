use anyhow::Error;

use crate::generated::io_haveno_protobuffer::{AckMessage, NodeAddress};
use uuid::Uuid;

pub async fn build_ack()-> Result<AckMessage, Error> {
    // Get node address from config
    let json = std::fs::read_to_string("config.json")?;
    let config: crate::utils::config::Config = serde_json::from_str(&json)?;
    
    let uid = Uuid::new_v4().to_string();

    let ack_message = AckMessage {
            uid: uid,
            sender_node_address: Some(NodeAddress {
                host_name: config.node_host.to_string(),
                port: config.node_port.abs(),
            }).clone(),
            success: true,
            source_type: String::default(),
            source_msg_class_name: String::default(),
            source_uid: String::default(),
            source_id: String::default(),
            error_message: String::default(),
            updated_multisig_hex: String::default(),
        };

    // Set the message type to ACK
    Ok (ack_message)
}