use crate::consts::ALPHA;

use cryptal::primitives::U256;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub(crate) type TransactionId = [u8; 4];

#[derive(Clone, Debug)]
pub(crate) enum Rpc {
    Ping(TransactionId),
    Pong(TransactionId),
    FindNode(TransactionId, U256),
    FoundNodes(TransactionId, Vec<(U256, SocketAddr)>),
    FindValue(TransactionId, U256),
    FoundValue(TransactionId, U256, Vec<u8>),
    StoreValue(TransactionId, Vec<u8>),
    Ok(TransactionId),
    Error(TransactionId),
}

impl Rpc {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            Rpc::Ping(tx_id) => {
                let mut bytes = vec![0];
                bytes.extend_from_slice(tx_id);
                bytes
            }
            Rpc::Pong(tx_id) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(tx_id);
                bytes
            }
            Rpc::FindNode(tx_id, target) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(tx_id);
                bytes.extend_from_slice(target.as_ref());
                bytes
            }
            Rpc::FoundNodes(tx_id, nodes) => {
                let mut bytes = vec![3];
                bytes.extend_from_slice(tx_id);
                bytes.push(nodes.len() as u8);
                for (id, addr) in nodes {
                    bytes.extend_from_slice(id.as_ref());

                    match addr {
                        SocketAddr::V4(v4) => {
                            bytes.push(0x04);
                            bytes.extend_from_slice(&v4.ip().octets());
                            bytes.extend_from_slice(&v4.port().to_be_bytes());
                        }
                        SocketAddr::V6(v6) => {
                            bytes.push(0x06);
                            bytes.extend_from_slice(&v6.ip().octets());
                            bytes.extend_from_slice(&v6.port().to_be_bytes());
                        }
                    }
                }
                bytes
            }
            Rpc::FindValue(tx_id, target) => {
                let mut bytes = vec![4];
                bytes.extend_from_slice(tx_id);
                bytes.extend_from_slice(target.as_ref());
                bytes
            }
            Rpc::FoundValue(tx_id, key, value) => {
                let mut bytes = vec![5];
                bytes.extend_from_slice(tx_id);
                bytes.extend_from_slice(key.as_ref());
                bytes.extend_from_slice(&(value.len() as u32).to_be_bytes());
                bytes.extend_from_slice(value);
                bytes
            }
            Rpc::StoreValue(tx_id, value) => {
                let mut bytes = vec![6];
                bytes.extend_from_slice(tx_id);
                bytes.extend_from_slice(&(value.len() as u32).to_be_bytes());
                bytes.extend_from_slice(value);
                bytes
            }
            Rpc::Ok(tx_id) => {
                let mut bytes = vec![7];
                bytes.extend_from_slice(tx_id);
                bytes
            }
            Rpc::Error(tx_id) => {
                let mut bytes = vec![8];
                bytes.extend_from_slice(tx_id);
                bytes
            }
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 5 {
            return None;
        }

        let mut tx_id = [0u8; 4];
        tx_id.copy_from_slice(&bytes[1..5]);

        match bytes.first()? {
            0 => Some(Rpc::Ping(tx_id)),
            1 => Some(Rpc::Pong(tx_id)),
            2 => Self::parse_id(&bytes[5..]).map(|id| Rpc::FindNode(tx_id, id)),
            3 => Self::parse_found_nodes(tx_id, bytes),
            4 => Self::parse_id(&bytes[5..]).map(|id| Rpc::FindValue(tx_id, id)),
            5 => Self::parse_found_value(tx_id, bytes),
            6 => Self::parse_store_value(tx_id, bytes),
            7 => Some(Rpc::Ok(tx_id)),
            8 => Some(Rpc::Error(tx_id)),
            _ => None,
        }
    }

    fn parse_id(bytes: &[u8]) -> Option<U256> {
        if bytes.len() < 32 {
            return None;
        }

        let mut target_bytes = [0u8; 32];
        target_bytes.copy_from_slice(&bytes[0..32]);
        let target = U256::from(target_bytes);

        Some(target)
    }

    fn parse_found_nodes(tx_id: TransactionId, bytes: &[u8]) -> Option<Self> {
        if bytes.len() <= 6 || bytes[5] == 0 || bytes[5] > ALPHA as u8 {
            return None;
        }

        let bytes_trunc = &bytes[6..];
        let mut nodes = Vec::new();
        let mut i = 0;

        while i < bytes_trunc.len() {
            if bytes_trunc[i..].len() < 32 {
                return None;
            }

            let mut id_bytes = [0u8; 32];
            id_bytes.copy_from_slice(&bytes_trunc[i..i + 32]);

            let id = U256::from(id_bytes);

            i += 32;

            let addr_type = *bytes_trunc.get(i)?;
            i += 1;

            let addr = match addr_type {
                0x04 => Self::parse_ipv4(&bytes_trunc[i..])?,
                0x06 => Self::parse_ipv6(&bytes_trunc[i..])?,
                _ => break,
            };

            nodes.push((id, addr));
        }

        Some(Rpc::FoundNodes(tx_id, nodes))
    }

    fn parse_ipv4(bytes: &[u8]) -> Option<SocketAddr> {
        if bytes.len() < 6 {
            return None;
        }

        let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
        let port = u16::from_be_bytes([bytes[4], bytes[5]]);

        Some(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    }

    fn parse_ipv6(bytes: &[u8]) -> Option<SocketAddr> {
        if bytes.len() < 18 {
            return None;
        }

        let ip = Ipv6Addr::from([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);

        let port = u16::from_be_bytes([bytes[16], bytes[17]]);

        Some(SocketAddr::V6(SocketAddrV6::new(ip, port, 0, 0)))
    }

    fn parse_found_value(tx_id: TransactionId, bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 41 {
            return None;
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes[5..37]);
        let key = U256::from(key_bytes);

        let len = u32::from_be_bytes([bytes[37], bytes[38], bytes[39], bytes[40]]) as usize;

        if bytes.len() != 41 + len {
            return None;
        }

        let value = bytes[41..].to_vec();

        Some(Rpc::FoundValue(tx_id, key, value))
    }

    fn parse_store_value(tx_id: TransactionId, bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 9 {
            return None;
        }

        let len = u32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]) as usize;

        if bytes.len() != 9 + len {
            return None;
        }

        let value = bytes[9..].to_vec();

        Some(Rpc::StoreValue(tx_id, value))
    }
}
