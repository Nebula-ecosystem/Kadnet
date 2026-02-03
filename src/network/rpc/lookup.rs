use crate::consts::{INTERVAL_MS, RPC_MAX_RETRY, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;
use crate::network::tcp::send_rpc;
use crate::routing::RoutingTable;
use crate::storage::core::LocalStorage;

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

    let _ = retry(RPC_MAX_RETRY, move || {
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
    .set_interval(Duration::from_millis(INTERVAL_MS))
    .await;
}

pub(crate) async fn find_value(
    addr: SocketAddr,
    local_storage: LocalStorage,
    routing: Arc<Mutex<RoutingTable>>,
    target: U256,
) {
    if let Some(value) = local_storage.contains(target) {
        let rpc = Rpc::FoundValue(target, value);

        let _ = retry(RPC_MAX_RETRY, move || {
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
        .set_interval(Duration::from_millis(INTERVAL_MS))
        .await;
    } else {
        find_node(addr, routing, target).await
    }
}
