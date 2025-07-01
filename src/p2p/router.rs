use async_trait::async_trait;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::p2p::context::PeerContext;

#[async_trait]
pub trait P2PMessageHandler: Send + Sync {
    async fn handle(&self, ctx: PeerContext) -> Result<()>;
}

pub struct P2PMessageRouter {
    handlers: RwLock<HashMap<String, Arc<dyn P2PMessageHandler>>>,
}

impl P2PMessageRouter {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register<T: Into<String>>(
        &self,
        message_type: T,
        handler: impl P2PMessageHandler + 'static,
    ) {
        let mut handlers = self.handlers.write().await;
        handlers.insert(message_type.into(), Arc::new(handler));
    }

    pub async fn dispatch(&self, ctx: PeerContext) -> Result<()> {
        let handlers = self.handlers.read().await;
        let message_name = ctx.envelope.message
            .as_ref()
            .map(|m| format!("{:?}", m))
            .unwrap_or_else(|| "None".into());

        for (name, handler) in handlers.iter() {
            if message_name.contains(name) {
                return handler.handle(ctx).await;
            }
        }

        println!("⚠️ No handler found for message: {}", message_name);
        Ok(())
    }
}