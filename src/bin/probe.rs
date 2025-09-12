#![recursion_limit = "512"]

use anyhow::Result;
use prost::Message;
use tokio::{
    io::{AsyncReadExt},
    net::TcpListener,
};

use haveno::generated::io_haveno_protobuffer::NetworkEnvelope;

/// Read and decode a length-prefixed NetworkEnvelope
async fn recv_envelope(mut socket: tokio::net::TcpStream) -> Result<()> {
    loop {
        let mut len_buf = [0u8; 4];
        if let Err(e) = socket.read_exact(&mut len_buf).await {
            eprintln!("[✘] Failed to read length prefix: {}", e);
            break;
        }

        let len = u32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; len];
        if let Err(e) = socket.read_exact(&mut body).await {
            eprintln!("[✘] Failed to read envelope body: {}", e);
            break;
        }

        match NetworkEnvelope::decode(&*body) {
            Ok(envelope) => {
                println!("[📦] Received envelope: {:#?}", envelope);
            }
            Err(e) => {
                eprintln!("[!] Failed to decode envelope: {}", e);
                println!("Raw: {:?}", body);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let port = 3333;
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("[👂] Listening on port {} for incoming envelopes...", port);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[+] Accepted connection from: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = recv_envelope(socket).await {
                eprintln!("[!] Error handling connection from {}: {}", addr, e);
            }
        });
    }
}