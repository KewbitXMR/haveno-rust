use async_trait::async_trait;
use anyhow::Result;
use crate::p2p::router::P2PMessageHandler;
use crate::p2p::context::PeerContext;
use crate::generated::io_haveno_protobuffer::network_envelope::Message as EnvMsg;


pub struct AddDataMessageHandler;

#[async_trait]
impl P2PMessageHandler for AddDataMessageHandler {
    async fn handle(&self,ctx: PeerContext) -> Result<()> {
        if let Some(EnvMsg::AddDataMessage(msg)) = ctx.envelope.message {
            println!("📥 [AddDataMessageHandler] Received AddDataMessage: {:#?}", msg);

            // 🔁 Optional: I don't think this message needs a response just remove from persisted storage

        } else {
            println!("⚠️ [AddDataMessageHandler] Unexpected message type.");
        }

        Ok(())
    }
}