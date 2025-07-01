use anyhow::Result;
use tokio::net::{TcpListener};
use tokio::sync::broadcast;
use std::sync::Arc;
use crate::p2p::context::PeerContext;
use crate::p2p::router::P2PMessageRouter;
use crate::utils::network::envelope::recv_envelope;
pub struct P2PListener {
    port: u16,
    shutdown_tx: broadcast::Sender<()>,
}

impl P2PListener {
    pub fn new(port: u16) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self { port, shutdown_tx }
    }

    pub async fn start(&self, router: Arc<P2PMessageRouter>) -> Result<()> {
        let port = 3333;
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;
        println!("[👂] Listening on port {port}...");

        loop {
            let (socket, _) = listener.accept().await?;
            let router = router.clone();

            tokio::spawn(async move {
                let mut socket = socket; // Make it mutable in this scope

                match recv_envelope(&mut socket).await {
                    Ok(Some(envelope)) => {
                        let ctx = PeerContext {
                            stream: socket, // You still own the full socket here
                            envelope,
                        };
                        if let Err(err) = router.dispatch(ctx).await {
                            eprintln!("❌ Router error: {err}");
                        }
                    }
                    Ok(None) => {
                        eprintln!("⚠️ Received empty envelope");
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to decode envelope: {e}");
                    }
                }
            });
        }
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }
}