use crate::network::rpc::Rpc;
use crate::network::tcp::send_rpc;
use crate::routing::RoutingTable;
use crate::storage::core::LocalStorage;

use cadentis::sync::Mutex;
use cryptal::primitives::U256;
use std::net::SocketAddr;
use std::sync::Arc;

pub(crate) async fn find_node(
    addr: SocketAddr,
    routing: Arc<Mutex<RoutingTable>>,
    tx_id: [u8; 4],
    target: U256,
) {
    let routing_guard = routing.lock().await;
    let closests: Vec<(U256, SocketAddr)> = routing_guard
        .get_closests(target)
        .await
        .iter()
        .map(|ne| (ne.id, ne.addr))
        .collect();

    let rpc = Rpc::FoundNodes(tx_id, closests);

    let _ = send_rpc(addr, rpc).await;
}

pub(crate) async fn find_value(
    addr: SocketAddr,
    local_storage: LocalStorage,
    routing: Arc<Mutex<RoutingTable>>,
    tx_id: [u8; 4],
    target: U256,
) {
    if let Some(value) = local_storage.contains(target) {
        let rpc = Rpc::FoundValue(tx_id, target, value);
        let _ = send_rpc(addr, rpc).await;
    } else {
        find_node(addr, routing, tx_id, target).await;
    }
}

pub(crate) async fn store_value(
    addr: SocketAddr,
    local_storage: LocalStorage,
    tx_id: [u8; 4],
    value: Vec<u8>,
) {
    let rpc = match local_storage.store(value) {
        Ok(_) => Rpc::Ok(tx_id),
        Err(_) => Rpc::Error(tx_id),
    };

    let _ = send_rpc(addr, rpc).await;
}
