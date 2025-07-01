use k256::ecdsa::SigningKey;

use crate::generated::io_haveno_protobuffer::{
    Filter, StoragePayload, ProtectedStorageEntry, StorageEntryWrapper, AddDataMessage,
    storage_payload::Message as PayloadMessage,
    storage_entry_wrapper::Message as WrapperMessage,
};

pub async fn build_add_data_message(filter: Filter, signer: &SigningKey) -> AddDataMessage {
    let payload = StoragePayload {
        message: Some(PayloadMessage::Filter(filter.clone())),
    };

    let protected_entry = ProtectedStorageEntry {
        storage_payload: Some(payload),
        owner_pub_key_bytes: signer.verifying_key().to_sec1_bytes().to_vec(),
        sequence_number: 1,
        signature: vec![], // Optional: you can sign this too, but may be redundant
        creation_time_stamp: chrono::Utc::now().timestamp_millis(),
    };

    let wrapper = StorageEntryWrapper {
        message: Some(WrapperMessage::ProtectedStorageEntry(protected_entry)),
    };

    AddDataMessage {
        entry: Some(wrapper),
    }
}