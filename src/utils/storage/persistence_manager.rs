use anyhow::{Result, Context};
use prost::Message;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::any::type_name;

pub enum Format {
    Json,
    Binary,
}

/// Convert a type name like `io.haveno.protobuffer.Filter` into a valid filename
fn type_to_filename<T: ?Sized>() -> String {
    let full_type = type_name::<T>();
    full_type
        .replace("::", ".")
        .replace('<', "_")
        .replace('>', "_")
}

/// Save a Protobuf message to disk in either JSON or binary
pub fn save_message<M: Message + serde::Serialize>(
    msg: &M,
    format: Format,
) -> Result<()> {
    let name = type_to_filename::<M>();
    let dir = PathBuf::from("persisted");
    fs::create_dir_all(&dir)?;

    let path = dir.join(match format {
        Format::Json => format!("{name}.json"),
        Format::Binary => format!("{name}"),
    });

    let mut file = File::create(&path)?;
    match format {
        Format::Json => {
            let json = serde_json::to_string_pretty(msg)?;
            file.write_all(json.as_bytes())?;
        }
        Format::Binary => {
            let mut buf = Vec::new();
            msg.encode(&mut buf)?;
            file.write_all(&buf)?;
        }
    }

    println!("💾 Saved to: {}", path.display());
    Ok(())
}

/// Load a message from disk
pub fn load_message<M: Message + Default + serde::de::DeserializeOwned>(
    format: Format,
) -> Result<M> {
    let name = type_to_filename::<M>();
    let path = PathBuf::from("persisted").join(match format {
        Format::Json => format!("{name}.json"),
        Format::Binary => format!("{name}"),
    });

    let mut file = File::open(&path).context("File not found")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let message = match format {
        Format::Json => serde_json::from_slice(&buf)?,
        Format::Binary => M::decode(&*buf)?,
    };

    Ok(message)
}