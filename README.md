# Kadnet

[![License](https://img.shields.io/badge/license-SSPL-blue.svg)](LICENSE)
![Dev Rust](https://img.shields.io/badge/Developed%20with-Rust%201.92.0-orange)
[![CI](https://github.com/Nebula-ecosystem/Kadnet/actions/workflows/ci.yml/badge.svg)](https://github.com/Nebula-ecosystem/Kadnet/actions/workflows/ci.yml)

---

**Kadnet** is the dedicated protocol for the ***Nebula*** ecosystem, providing the entry point.

---

## 📊 Project Status

## 1. Fondations DHT (Kademlia)

- [x] NodeID Definition
- [x] XOR Distance Metric
- [x] K-Bucket Structure & Rules
- [x] Routing Table Logic

## 2. RPC Protocol

- [x] Message Format & Serialization
- [ ] Transaction IDs & Timeout Handling
- [x] `PING` / `PONG`
- [x] `FIND_NODE`
- [x] `STORE`
- [x] `FIND_VALUE`

## 3. Stockage Local

- [ ] In-Memory Key/Value Store
- [ ] Disk Persistence
- [ ] Backup & Restore
- [ ] Storage Quotas & Eviction Policy

## 4. Lookup & Iterative Algorithms

- [ ] Iterative Node Lookup
- [ ] Iterative Value Lookup
- [ ] Store Propagation Strategy
- [ ] Bucket Refresh

## 5. Networking & Transport

- [ ] Async UDP Socket
- [ ] Send / Receive Pipeline
- [ ] Inbound / Outbound Dispatch
- [ ] Peer Liveness Detection
- [ ] NAT Traversal & Keep-alive

## 6. Peer Identity & Sessions

- [ ] Node Identity Exchange
- [ ] Capability Negotiation
- [ ] Session Tracking

## 7. Security (Nebula Shield)

- [ ] Message Signing & Verification (Ed25519)
- [ ] Secure Node Identity
- [ ] User Account Binding
- [ ] Multi-Device Key Management
- [ ] Sybil Resistance
- [ ] Eclipse Attack Mitigation

## 8. Nebula Network Services

- [ ] Bootstrap Nodes
- [ ] Peer Reputation & Scoring
- [ ] Network Metrics
- [ ] Relay / Gateway Nodes


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