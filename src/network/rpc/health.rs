use super::Rpc;
use super::id::new_tid;
use crate::network::errors::NetworkError;
use crate::network::tcp::{read_rpc, send_rpc, send_with_retry};

use cadentis::net::TcpStream;
use cadentis::sync::Mutex;
use cadentis::time::instrumented;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn ping(addr: SocketAddr) -> Result<Duration, NetworkError> {
    let (res, duration) = instrumented(async {
        let tx_id = new_tid();
        let stream = send_rpc(addr, Rpc::Ping(tx_id)).await?;
        read_rpc(stream).await
    })
    .await;

    res.map(|_| duration)
}

pub(crate) async fn pong(stream: Arc<Mutex<TcpStream>>, tx_id: [u8; 4]) {
    let _ = send_with_retry(stream, Rpc::Pong(tx_id)).await;
}
