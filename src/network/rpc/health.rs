use super::Rpc;
use crate::consts::{PING_MAX_RETRY, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::tcp::{read_rpc, send_rpc};

use cadentis::time::{instrumented, timeout};
use cadentis::tools::retry;
use std::net::SocketAddr;
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
