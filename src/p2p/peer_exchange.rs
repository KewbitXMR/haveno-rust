use std::{collections::HashMap, fs, path::PathBuf, time::Duration};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio::time::timeout;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Peer {
    address: String,
    validated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PeerState {
    peers: HashMap<String, Peer>,
}

const STATE_FILE: &str = "peers.json";
const TIMEOUT_SECONDS: u64 = 5;

#[tokio::main]
async fn main() {
    let mut state = load_peer_state().unwrap_or_else(|| PeerState {
        peers: HashMap::new(),
    });

    if state.peers.is_empty() {
        eprintln!("❗ No peers found. Please add at least one peer to `peers.json` and try again.");
        std::process::exit(1);
    }

    let client = Client::new();
    let mut new_peers = HashMap::new();
    let mut validated_peers = HashMap::new();

    for (addr, peer) in &state.peers {
        match query_peer(&client, addr).await {
            Ok(response_peers) => {
                let mut validated_peer = peer.clone();
                validated_peer.validated = true;
                validated_peers.insert(addr.clone(), validated_peer);

                for new_addr in response_peers {
                    if !state.peers.contains_key(&new_addr) && !new_peers.contains_key(&new_addr) {
                        new_peers.insert(new_addr.clone(), Peer {
                            address: new_addr,
                            validated: false,
                        });
                    }
                }
            }
            Err(_) => {
                // Do not include in validated list (implicitly removes it)
            }
        }
    }

    // Combine validated peers with new unvalidated ones
    validated_peers.extend(new_peers);

    let updated_state = PeerState {
        peers: validated_peers,
    };

    save_peer_state(&updated_state).await;
    println!("✅ Peer list updated. Total peers: {}", updated_state.peers.len());
}

async fn query_peer(client: &Client, addr: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = format!("http://{}/peers", addr);
    let result = timeout(Duration::from_secs(TIMEOUT_SECONDS), client.get(url).send()).await;
    match result {
        Ok(Ok(resp)) => {
            if resp.status().is_success() {
                let peers: Vec<String> = resp.json().await.unwrap_or_default();
                Ok(peers)
            } else {
                Err(reqwest::Error::new(reqwest::StatusCode::BAD_REQUEST, "Bad status"))
            }
        }
        _ => Err(reqwest::Error::new(reqwest::StatusCode::REQUEST_TIMEOUT, "Timeout")),
    }
}

fn load_peer_state() -> Option<PeerState> {
    let path = PathBuf::from(STATE_FILE);
    if !path.exists() {
        return None;
    }
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

async fn save_peer_state(state: &PeerState) {
    let json = serde_json::to_string_pretty(state).unwrap();
    let mut file = File::create(STATE_FILE).await.unwrap();
    file.write_all(json.as_bytes()).await.unwrap();
}