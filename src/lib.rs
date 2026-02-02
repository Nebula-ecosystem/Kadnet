pub(crate) mod network;
pub mod node;
pub(crate) mod routing;

pub(crate) mod consts {
    use cryptal::primitives::U256;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    pub(crate) static KUSIZE: usize = 32;
    pub(crate) static SMALL_BUCKET_COUNT: usize = 4;

    pub(crate) static N_BUCKETS: usize = 256;
    pub(crate) static PING_MAX_RETRY: usize = 3;
    pub(crate) static T_MAX_MS: u64 = 800;

    pub(crate) static BOOTSTRAPS_ADDRESS: [(U256, SocketAddr); 1] = [(
        U256::ZERO,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5734),
    )];
}
