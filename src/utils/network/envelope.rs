use bytes::BytesMut;
use prost::Message;
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{timeout, Duration},
};
use crate::generated::io_haveno_protobuffer::{network_envelope, NetworkEnvelope};

pub type EnvMsg = crate::generated::io_haveno_protobuffer::network_envelope::Message;

pub async fn send_envelope(stream: &mut TcpStream, env: &NetworkEnvelope) -> Result<()> {
    let mut body = Vec::new();
    env.encode(&mut body)?;

    // Encode the varint length prefix using BytesMut
    let mut frame = BytesMut::with_capacity(10 + body.len());
    prost::encoding::encode_varint(body.len() as u64, &mut frame);

    // Append the message body
    frame.extend_from_slice(&body);

    stream.write_all(&frame).await?;
    Ok(())
}

pub async fn recv_envelope(stream: &mut TcpStream) -> Result<Option<NetworkEnvelope>> {
    let mut varint_buf = BytesMut::with_capacity(10);
    let timeout_duration = Duration::from_secs(30);

    // Read varint length byte-by-byte
    loop {
        let mut byte = [0u8; 1];
        timeout(timeout_duration, stream.read_exact(&mut byte)).await??;
        varint_buf.extend_from_slice(&byte);
        if byte[0] & 0x80 == 0 {
            break;
        }
    }

    let mut cursor = &varint_buf[..];
    let len = prost::encoding::decode_varint(&mut cursor)? as usize;

    // Read message body
    let mut msg_buf = vec![0u8; len];
    timeout(timeout_duration, stream.read_exact(&mut msg_buf)).await??;

    let envelope = NetworkEnvelope::decode(&*msg_buf)?;
    Ok(Some(envelope))
}

pub fn build_envelope(message: network_envelope::Message) -> NetworkEnvelope {
    NetworkEnvelope {
        message_version: "0X".into(), // customize if needed
        message: Some(message),
    }.into()
}