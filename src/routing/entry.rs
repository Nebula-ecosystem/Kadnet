use crate::consts::{DISTANCE_WEIGHT_SHIFT, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::rpc::ping;

use cryptal::primitives::U256;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct NodeEntry {
    pub(crate) id: U256,
    pub(crate) addr: SocketAddr,

    pub(crate) score: U256,
    pub(crate) respond_time: Duration,
    pub(crate) distance: U256,
}

impl NodeEntry {
    pub(crate) async fn new(id: U256, addr: SocketAddr) -> Result<Self, NetworkError> {
        let respond_time = ping(addr).await?;

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

    pub(crate) fn distance(&self, target: U256) -> U256 {
        self.id ^ target
    }

    fn time_penalty(&self) -> U256 {
        let d_part = self.distance >> DISTANCE_WEIGHT_SHIFT.into();

        let t_ms = self.respond_time.as_millis();
        let t_norm = t_ms.min(T_MAX_MS as u128);

        d_part * U256::from(t_norm) / U256::from(T_MAX_MS)
    }

    pub(crate) fn compute_score(&mut self, target: U256) {
        self.score = self.distance(target) + self.time_penalty();
    }
}
