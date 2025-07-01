use crate::generated::io_haveno_protobuffer::NetworkEnvelope;

/// Holds context about a received P2P message and connection
pub struct PeerContext {
    pub stream: tokio::net::TcpStream,
    pub envelope: NetworkEnvelope,
}