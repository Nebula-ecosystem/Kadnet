use crate::consts::{INTERVAL_MS, RPC_MAX_RETRY, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;
use crate::network::rpc::find_node;
use crate::network::rpc::id::remove_tid;
use crate::network::rpc::lookup::{find_value, store_value};
use crate::network::rpc::pong;
use crate::routing::RoutingTable;
use crate::storage::core::LocalStorage;

use cadentis::net::{TcpListener, TcpStream};
use cadentis::sync::Mutex;
use cadentis::task;
use cadentis::time::timeout;
use cadentis::tools::retry;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn listen(
    port: u16,
    routing: Arc<Mutex<RoutingTable>>,
) -> Result<(), NetworkError> {
    let listener =
        TcpListener::bind(&format!("127.0.0.1:{port}")).map_err(|_| NetworkError::Connection)?;

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .map_err(|_| NetworkError::Connection)?;

        let stream = Arc::new(Mutex::new(stream));
        let r = routing.clone();

        task::spawn(async move {
            handle_connection(addr, stream, r).await;
            Ok::<(), NetworkError>(())
        })
        .await?;
    }
}

async fn handle_connection(
    addr: SocketAddr,
    stream: Arc<Mutex<TcpStream>>,
    routing: Arc<Mutex<RoutingTable>>,
) {
    match read_rpc(stream.clone()).await {
        Ok(Rpc::Ping(tx_id)) => pong(stream, tx_id).await,
        Ok(Rpc::FindNode(tx_id, target)) => find_node(addr, routing, tx_id, target).await,
        Ok(Rpc::FindValue(tx_id, target)) => {
            find_value(addr, LocalStorage::new(), routing, tx_id, target).await
        }
        Ok(Rpc::StoreValue(tx_id, value)) => {
            store_value(addr, LocalStorage::new(), tx_id, value).await
        }
        Ok(Rpc::Ok(tx_id)) => {
            remove_tid(&tx_id);
        }
        Ok(Rpc::Error(tx_id)) => {
            remove_tid(&tx_id);
        }
        _ => {
            // Ignore
        }
    }
}

pub(crate) async fn send_rpc(
    addr: SocketAddr,
    rpc: Rpc,
) -> Result<Arc<Mutex<TcpStream>>, NetworkError> {
    let addr_str = addr.to_string();

    retry(RPC_MAX_RETRY, move || {
        let addr_str = addr_str.clone();
        let rpc = rpc.clone();

        async move {
            timeout(Duration::from_millis(T_MAX_MS), async {
                let stream = TcpStream::connect(&addr_str)
                    .await
                    .map_err(|_| NetworkError::Connection)?;

                stream
                    .write_all(rpc.as_bytes().as_slice())
                    .await
                    .map_err(|_| NetworkError::Write)?;

                Ok(Arc::new(Mutex::new(stream)))
            })
            .await
            .map_err(|_| NetworkError::Timeout)?
        }
    })
    .set_interval(Duration::from_millis(INTERVAL_MS))
    .await
}

pub(crate) async fn send_with_retry(
    stream: Arc<Mutex<TcpStream>>,
    rpc: Rpc,
) -> Result<(), NetworkError> {
    let rpc_data = rpc.as_bytes().to_vec(); // Clone the data to avoid borrowing issues

    retry(RPC_MAX_RETRY, {
        let rpc_data = rpc_data.clone();
        let stream = Arc::clone(&stream);

        move || {
            let stream_clone = Arc::clone(&stream);
            let rpc_data = rpc_data.clone();

            async move {
                let s = stream_clone.lock().await;
                s.write_all(&rpc_data)
                    .await
                    .map_err(|_| NetworkError::Write)
            }
        }
    })
    .set_interval(Duration::from_millis(INTERVAL_MS))
    .await
}

pub(crate) async fn read_rpc(stream: Arc<Mutex<TcpStream>>) -> Result<Rpc, NetworkError> {
    let mut buffer = [0; 1024];
    let stream_guard = stream.lock().await;

    let n = match stream_guard.read(&mut buffer).await {
        Ok(n) => {
            if n == 0 {
                return Err(NetworkError::Connection);
            }
            n
        }
        Err(_) => {
            return Err(NetworkError::Read);
        }
    };

    match Rpc::from_bytes(&buffer[..n]) {
        Some(r) => Ok(r),
        None => Err(NetworkError::CouldNotParseRPC),
    }
}
