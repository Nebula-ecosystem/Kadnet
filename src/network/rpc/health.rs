use super::Rpc;
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
        let stream = send_rpc(addr, Rpc::Ping).await?;
        read_rpc(stream).await
    })
    .await;

    res.map(|_| duration)
}

pub(crate) async fn pong(stream: Arc<Mutex<TcpStream>>) {
    let _ = send_with_retry(stream, Rpc::Pong).await;
}
