use crate::network;
use crate::network::errors::NetworkError;

use cryptal::primitives::U256;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Clone, Copy)]
pub(crate) struct NodeEntry {
    pub(crate) id: U256,
    pub(crate) addr: SocketAddr,

    #[allow(dead_code)]
    pub(crate) score: U256,
    pub(crate) respond_time: Duration,
    #[allow(dead_code)]
    pub(crate) distance: U256,
}

impl NodeEntry {
    pub(crate) async fn new(id: U256, addr: SocketAddr) -> Result<Self, NetworkError> {
        let respond_time = network::rpc::ping(addr).await?;

        Ok(Self {
            id,
            addr,
            score: U256::ZERO,
            respond_time,
            distance: U256::ZERO,
        })
    }

    pub(crate) fn update_respond_time(&mut self, duration: Duration) {
        self.respond_time = duration;
    }
}
