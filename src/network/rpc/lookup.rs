use crate::consts::{PING_MAX_RETRY, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;
use crate::network::tcp::send_rpc;
use crate::routing::RoutingTable;

use cadentis::sync::Mutex;
use cadentis::time::timeout;
use cadentis::tools::retry;
use cryptal::primitives::U256;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn find_node(addr: SocketAddr, routing: Arc<Mutex<RoutingTable>>, target: U256) {
    let routing_guard = routing.lock().await;
    let closests = routing_guard
        .get_closests(target)
        .await
        .iter()
        .map(|ne| (ne.id, ne.addr))
        .collect();

    let rpc = Rpc::FoundNodes(closests);

    let _ = retry(PING_MAX_RETRY, move || {
        let rpc = rpc.clone();
        async move {
            let rpc = rpc.clone();

            timeout(Duration::from_millis(T_MAX_MS), async {
                let rpc = rpc.clone();
                send_rpc(addr, rpc).await
            })
            .await
            .map_err(|_| NetworkError::Timeout)?
        }
    })
    .set_interval(Duration::from_millis(200))
    .await;
}
