use cadentis::net::{TcpListener, TcpStream};
use cadentis::task;
use std::net::SocketAddr;

use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;

pub(crate) async fn listen(port: u16) -> Result<(), NetworkError> {
    let listener =
        TcpListener::bind(&format!("127.0.0.1:{port}")).map_err(|_| NetworkError::Connection)?;

    loop {
        let (stream, _addr) = listener
            .accept()
            .await
            .map_err(|_| NetworkError::Connection)?;

        task::spawn(async move {
            handle_connection(stream).await;
            Ok::<(), NetworkError>(())
        })
        .await?;
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut first = [0u8; 1];
    match stream.read(&mut first).await {
        Ok(0) | Err(_) => return,
        Ok(_) => {}
    }

    let rpc = match Rpc::from_bytes(&first[..1]) {
        Some(rpc) if [Rpc::Ping, Rpc::Pong].contains(&rpc) => rpc,
        _ => return,
    };

    match rpc {
        Rpc::Ping => {
            let _ = stream.write_all(&[Rpc::Pong.as_byte()]).await;
        }
        Rpc::Pong => {}
    }
}

pub(crate) async fn send_rpc(addr: SocketAddr, rpc: Rpc) -> Result<TcpStream, NetworkError> {
    let addr_str = addr.to_string();

    let stream = TcpStream::connect(&addr_str)
        .await
        .map_err(|_| NetworkError::Connection)?;

    stream
        .write_all(&[rpc.as_byte()])
        .await
        .map_err(|_| NetworkError::Write)?;

    Ok(stream)
}

pub(crate) async fn read_rpc(stream: TcpStream) -> Result<Rpc, NetworkError> {
    let mut buffer = [0; 4];
    let n = match stream.read(&mut buffer).await {
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
