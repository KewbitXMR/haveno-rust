use std::fs;
use std::path::Path;
use anyhow::Result;
use haveno_basic_bootstrap::generated::io_haveno_protobuffer::{
    NetworkEnvelope,
    network_envelope::Message as EnvMsg,
    GetDataResponse,
    storage_payload::Message as StorageMsg,
};
use prost::Message;
use base64::{engine::general_purpose, Engine};
use serde_json::json;

pub struct MessageProcessor {
    base_dir: String,
}

impl MessageProcessor {
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: base_dir.to_string(),
        }
    }

    pub fn process(&self, envelope: &NetworkEnvelope) -> Result<()> {
        match &envelope.message {
            Some(EnvMsg::GetDataResponse(resp)) => {
                self.handle_get_data_response(resp)?;
            }
            other => {
                println!("🟡 No handler for {:?}", other);
            }
        }
        Ok(())
    }

    fn handle_get_data_response(&self, resp: &GetDataResponse) -> Result<()> {
        let path = Path::new(&self.base_dir).join("seednode/get_data_response");
        fs::create_dir_all(&path)?;

        for (i, payload) in resp.persistable_network_payload_items.iter().enumerate() {
            if let Some(message) = &payload.message {
                let (kind, bytes) = match message {
                    StorageMsg::AccountAgeWitness(v) => ("AccountAgeWitness", v.encode_to_vec()),
                    StorageMsg::SignedWitness(v) => ("SignedWitness", v.encode_to_vec()),
                    StorageMsg::TradeStatistics3(v) => ("TradeStatistics3", v.encode_to_vec()),
                    _ => continue,
                };

                let encoded = general_purpose::STANDARD.encode(&bytes);
                let out = json!({
                    "type": kind,
                    "encoded": encoded
                });

                let filename = format!("{}_{}.json", i, kind);
                let file_path = path.join(filename);
                fs::write(file_path, serde_json::to_string_pretty(&out)?)?;
            }
        }

        Ok(())
    }
}