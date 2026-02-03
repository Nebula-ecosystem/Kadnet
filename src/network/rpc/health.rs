use super::Rpc;
use crate::consts::{PING_MAX_RETRY, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::tcp::{read_rpc, send_rpc};

use cadentis::net::TcpStream;
use cadentis::sync::Mutex;
use cadentis::time::{instrumented, timeout};
use cadentis::tools::retry;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn ping(addr: SocketAddr) -> Result<Duration, NetworkError> {
    let (res, duration) = instrumented(
        retry(PING_MAX_RETRY, move || async move {
            timeout(Duration::from_millis(T_MAX_MS), async {
                let stream = send_rpc(addr, Rpc::Ping).await?;
                read_rpc(stream).await
            })
            .await
            .map_err(|_| NetworkError::Timeout)?
        })
        .set_interval(Duration::from_millis(200)),
    )
    .await;

    res.map(|_| duration)
}

pub(crate) async fn pong(stream: Arc<Mutex<TcpStream>>) {
    let _ = retry(PING_MAX_RETRY, {
        move || {
            let stream = Arc::clone(&stream);
            async move {
                let s = stream.lock().await;
                s.write_all(Rpc::Pong.as_bytes().as_slice()).await
            }
        }
    })
    .await;
}
