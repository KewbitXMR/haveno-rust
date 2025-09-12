use crate::generated::io_haveno_protobuffer::{NodeAddress, GetUpdatedDataRequest};

pub async fn build_get_updated_data() -> Result<GetUpdatedDataRequest, anyhow::Error> {

    // Get node address from config
    let json = std::fs::read_to_string("config.json")?;
    let config: crate::utils::config::Config = serde_json::from_str(&json)?;

    Ok (
        GetUpdatedDataRequest{
            sender_node_address: Some(NodeAddress {
                port: config.node_port.abs(),
                host_name: config.node_host.to_string(),
            }),
            nonce: 3424,
            excluded_keys: vec![],
            version: config.version.to_string(),
        }
    )
}