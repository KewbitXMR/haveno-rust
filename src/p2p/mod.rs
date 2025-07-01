mod listener;
pub mod router;
pub mod handlers;
mod context;

use anyhow::Result;
use std::sync::Arc;
use crate::p2p::router::P2PMessageRouter;
use crate::p2p::listener::P2PListener;
use crate::p2p::handlers::add_data;

/// Initialize and run the P2P layer.
pub async fn setup() -> Result<()> {
    let router = Arc::new(P2PMessageRouter::new());
    let listener = P2PListener::new(3333);

    // ✅ Register handlers
    router.register("AddDataMessage", add_data::AddDataMessageHandler).await;

    // 🚀 Start listener in a background task
    tokio::spawn({
        let router = router.clone();
        async move {
            if let Err(e) = listener.start(router).await {
                eprintln!("🔥 P2P Listener error: {e}");
            }
        }
    });

    Ok(())

}