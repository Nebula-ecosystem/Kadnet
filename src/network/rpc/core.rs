#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum Rpc {
    Ping,
    Pong,
}

impl Rpc {
    pub(crate) fn as_byte(&self) -> u8 {
        match self {
            Rpc::Ping => 0,
            Rpc::Pong => 1,
        }
    }

    pub(crate) fn from_bytes(byte: &[u8]) -> Option<Self> {
        match byte[0] {
            0 => Some(Rpc::Ping),
            1 => Some(Rpc::Pong),
            _ => None,
        }
    }
}
