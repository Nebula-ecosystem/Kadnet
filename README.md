# Kadnet

[![License](https://img.shields.io/badge/license-SSPL-blue.svg)](LICENSE)
![Dev Rust](https://img.shields.io/badge/Developed%20with-Rust%201.92.0-orange)
[![CI](https://github.com/Nebula-ecosystem/Kadnet/actions/workflows/ci.yml/badge.svg)](https://github.com/Nebula-ecosystem/Kadnet/actions/workflows/ci.yml)

---

**Kadnet** is the dedicated protocol for the ***Nebula*** ecosystem, providing the entry point.

---

## 📊 Project Status

- [ ] **Core DHT Logic (Kademlia Engine)**
  - [x] NodeID Definition
  - [x] XOR Metric Logic
  - [x] K-Bucket Structure
  - [x] Routing Table Management
  - [ ] Local Storage & Backup (Persist stored files to disk)

- [ ] **RPC & Wire Protocol**
  - [ ] Message Serialization
  - [ ] Transaction ID & RPC Matching
  - [ ] Core RPC Operations (`PING`/`PONG`, `FIND_NODE`, `STORE`/`FIND_VALUE`)

- [ ] **Iterative Operations**
  - [ ] Recursive Node Lookup
  - [ ] Key/Value Propagation Logic

- [ ] **Async Networking (Transport)**
  - [ ] Async UDP Socket Integration
  - [ ] Handlers for Inbound/Outbound UDP Streams
  - [ ] Peer Handshaking (Identity exchange)
  - [ ] NAT Traversal & Keep-alive

- [ ] **Security (Nebula Shield)**
  - [ ] Message Signing & Verification (Ed25519)
  - [ ] User Account Management (Secure Authentication & Identity)
  - [ ] Multi-Device Support (Secure Key & Session Handling)
  - [ ] Resistance to Sybil & Eclipse Attacks

- [ ] **Nebula Ecosystem Bridges**
  - [ ] Bootstrap Peer Discovery Service
  - [ ] Peer Reputation & Scoring System

## 🚀 Getting Started

This crate is not yet published on crates.io. Add it directly from GitHub:

``` toml
[dependencies]
kadnet = { git = "https://github.com/Nebula-ecosystem/Kadnet" }
```

---


## 🦀 Rust Version

- **Developed with**: Rust 1.92.0
- **MSRV**: Rust 1.92.0 (may increase in the future)

---

## 📄 License Philosophy

Kadnet is licensed under the **Server Side Public License (SSPL) v1**.

This license is intentionally chosen to protect the integrity of the Nebula ecosystem.  
While the project is fully open for **contribution, improvement, and transparency**,  
SSPL prevents third parties from creating competing platforms, proprietary versions,  
or commercial services derived from the project.

Nebula is designed to grow as **one unified, community-driven network**.  
By using SSPL, we ensure that:

- all improvements remain open and benefit the ecosystem,  
- the network does not fragment into multiple incompatible forks,  
- companies cannot exploit the project without contributing back,  
- contributors retain full access to the entire codebase.


In short, SSPL ensures that Kadnet — and the Nebula ecosystem built on top of it —  
remains **open to the community, but protected from fragmentation and exploitation**.

## 🤝 Contact

For questions, discussions, or contributions, feel free to reach out:

- **Discord**: enzoblain
- **Email**: [enzoblain@proton.me](mailto:enzoblain@proton.me)