use cadentis::net::{TcpListener, TcpStream};
use cadentis::sync::Mutex;
use cadentis::task;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;
use crate::network::rpc::find_node;
use crate::network::rpc::lookup::{find_value, store_value};
use crate::network::rpc::pong;
use crate::routing::RoutingTable;
use crate::storage::core::LocalStorage;

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
        Ok(Rpc::Ping) => pong(stream).await,
        Ok(Rpc::FindNode(target)) => find_node(addr, routing, target).await,
        Ok(Rpc::FindValue(target)) => find_value(addr, LocalStorage::new(), routing, target).await,
        Ok(Rpc::StoreValue(value)) => store_value(addr, LocalStorage::new(), value).await,
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

    let stream = TcpStream::connect(&addr_str)
        .await
        .map_err(|_| NetworkError::Connection)?;

    stream
        .write_all(rpc.as_bytes().as_slice())
        .await
        .map_err(|_| NetworkError::Write)?;

    Ok(Arc::new(Mutex::new(stream)))
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
