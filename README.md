# Haveno

**Haveno** is a decentralized peer-to-peer (P2P) trading framework and application built with a strong focus on privacy, extensibility, and interoperability. Inspired by projects like [Haveno](https://haveno.app) and [Bisq](https://bisq.network), The Haveno App is designed for multi-perpose full-node use of agregating liquidity from several areas of the market and is not just a platform—it's a movement towards sovereign, borderless, and censorship-resistant exchange of value. This package is a low level interface for the gRPC client/server interface but is also slowly becoming a rust native node starting with Haveno compatibility then potentially expanding the protocol.

---

## 🪡 Vision

To unify decentralized liquidity pools across the cryptocurrency ecosystem, enabling seamless, anonymous, and trust-minimized trading between multiple networks and protocols including Monero, Ethereum, and beyond.

## 👊 Built With

- **Rust** — lightning-fast, memory-safe, and fearless concurrency.
- **Protobuf** — for cross-DEX protocol interoperability.
- **Tor & I2P** — enforced anonymity by default.
- **Monero** — as a base currency and privacy anchor.
- **gRPC** — for efficient P2P and service communication.
- **Tokio** + **async** — for scalable networking and I/O.
- **OpenSSL / ed25519 / DSA** — for cryptographic signing, peer authentication, and secure messaging.

---

## 📚 Features

### ✨ Decentralized and Anonymous
- **No central servers**. Operates via a self-discovering peer-to-peer mesh (requires seednodes at apex).
- **Tor-first networking** with I2P support coming soon.
- **Encrypted gRPC streams** and anonymous handshakes.

### ⚖️ AI-Powered Arbitration
- Optional **Large Language Model (LLM)** arbitration system.
- Open source arbitration logic trained on past trade history.
- Fair dispute resolution without exposing identities.

### 🌐 Cross-DEX Integration (WIP)
- Interoperable protobuf message format inspired by Haveno/Bisq.
- Plans to support:
  - **Bisq** (via filter + envelope compatibility)
  - **Haveno** (natively compatible)
  - **Serai** (via future module bridge)
  - **ETH/ERC20** (atomic swap adapters)

### 🔗 Modular Design
- Can be used as a base for custom DEXs.
- Extend with custom trading logic, consensus, or identity.
- Hook into the routing layer or message handler chain.

### 🥐 Atomic Swaps Engine (Coming Soon)
- Native Rust implementation of atomic swap logic for:
  - BTC/XMR
  - ETH/XMR
  - USDT/XMR
- With pluggable scripting logic for more chains.

### 🏱 Liquidity Aggregation (Alpha)
- SmartDEX is a **liquidity meta-layer**.
- Aggregates trade offers from:
  - Haveno seed nodes
  - Bisq filter broadcasts
  - ETH-based DEXs via oracles (future)

### 🌐 Onion-first Discovery
- Bootstrap via known `.onion` addresses.
- Passive and active peer discovery built-in.
- Anti-sybil measures in design (via PoW-based trust weighting).

---

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable)
- Tor or Tor proxy
- Protobuf compiler (`protoc`)

### Build
```bash
cargo build --release
```

### Run
```bash
./target/release/haveno --config config.json
```

### Configuration
`config.json` example:
```json
{
  "node_host": "localhost",
  "node_port": 2001,
  "secret": "<your 32-byte hex private key>",
  "bootstrap_onion": "your.bootstrap.node.onion:port"
}
```

---

## 🏁 Architecture

- `p2p/`
  - gRPC + protobuf message handling
  - Envelope parsing, signing, and relaying

- `core/`
  - Trading logic and liquidity matching
  - Offer creation, validation, and filtering

- `crypto/`
  - Key handling, signing, and peer ID derivation

- `network/`
  - Tor/I2P networking stack
  - Connection lifecycle and peer discovery

- `arbitration/`
  - LLM-based dispute resolution (via local or remote model inference)

- `cli/`
  - Interactive command-line interface for testing and node ops

---

## 🚫 Privacy First

Haveno Multiplatform makes no compromises:
- No IP logging
- No analytics
- End-to-end encryption
- All traffic routed via Tor/I2P

---

## 🪧 Use Cases

- Cross-border peer-to-peer crypto trading
- Censorship-resistant market creation
- White-label decentralized exchanges
- Off-grid trading between merchants in high-surveillance regions
- An open framework to test financial protocols privately

---

## 🎁 Future Plans

- [ ] I2P integration
- [ ] Zero-knowledge proof integration for offer matching
- [ ] Federated relays for offer caching
- [ ] Web-based dashboard (Tor hidden service only)
- [ ] Mobile light clients (Android only initially)
- [ ] Plugin-based consensus filters
- [ ] NFT and exotic asset support

---

## ✌️ License

The Haveno Multiplatform App is released under the **Apache 2.0** license to preserve the freedoms of the network while deterring centralizing abuse.

> Note: If you wish to use this framework for proprietary purposes, please reach out to the author.

---

## 🧱 Credits

Built with passion by **[Kewbit](https://kewbit.org)**, from the experience gained building [Haveno App](https://haveno.app).

Inspired by:
- Haveno
- Bisq
- Serai
- Monero Community
- The cypherpunk ethos

---

## 🚜 Contributing

Pull requests welcome! Please:
- Follow Rust best practices
- Keep the codebase async-safe
- Sign commits where possible
- Test with both onion and clearnet peers

Let’s build something unstoppable.

