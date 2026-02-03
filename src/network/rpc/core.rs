use crate::consts::ALPHA;

use cryptal::primitives::U256;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[derive(Clone, Debug)]
pub(crate) enum Rpc {
    Ping,
    Pong,
    FindNode(U256),
    FoundNodes(Vec<(U256, SocketAddr)>),
    FindValue(U256),
    FoundValue(U256, Vec<u8>),
}

impl Rpc {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            Rpc::Ping => vec![0],
            Rpc::Pong => vec![1],
            Rpc::FindNode(target) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(target.as_ref());
                bytes
            }
            Rpc::FoundNodes(nodes) => {
                let mut bytes = vec![3, nodes.len() as u8];
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
            Rpc::FindValue(target) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(target.as_ref());
                bytes
            }
            Rpc::FoundValue(key, value) => {
                let mut bytes = vec![5];
                bytes.extend_from_slice(key.as_ref());
                bytes.extend_from_slice(&(value.len() as u32).to_be_bytes());
                bytes.extend_from_slice(value);
                bytes
            }
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.first()? {
            0 => Some(Rpc::Ping),
            1 => Some(Rpc::Pong),
            2 => Self::parse_id(bytes).map(Self::FindNode),
            3 => Self::parse_found_nodes(bytes),
            4 => Self::parse_id(bytes).map(Self::FindValue),
            5 => Self::parse_found_value(bytes),
            _ => None,
        }
    }

    fn parse_id(bytes: &[u8]) -> Option<U256> {
        if bytes.len() != 33 {
            return None;
        }

        let mut target_bytes = [0u8; 32];
        target_bytes.copy_from_slice(&bytes[1..33]);
        let target = U256::from(target_bytes);

        Some(target)
    }

    fn parse_found_nodes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() <= 2 || bytes[1] == 0 || bytes[1] > ALPHA as u8 {
            return None;
        }

        let bytes_trunc = &bytes[2..];
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

        Some(Rpc::FoundNodes(nodes))
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

    fn parse_found_value(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 37 {
            return None;
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes[1..33]);
        let key = U256::from(key_bytes);

        let len = u32::from_be_bytes([bytes[33], bytes[34], bytes[35], bytes[36]]) as usize;

        if bytes.len() != 37 + len {
            return None;
        }

        let value = bytes[37..].to_vec();

        Some(Rpc::FoundValue(key, value))
    }
}
