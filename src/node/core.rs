use super::errors::NodeError;
use crate::consts::BOOTSTRAPS_ADDRESS;
use crate::network::errors::NetworkError;
use crate::network::tcp::listen;
use crate::routing::RoutingTable;
use crate::routing::entry::NodeEntry;
use crate::routing::id::generate_id;
use crate::routing::kbucket::ping_entries;

use cadentis::sync::Mutex;
use cadentis::time::sleep;
use cadentis::{join, task};
use cryptal::keys::ed25519;
use std::sync::Arc;
use std::time::Duration;

pub struct Node {
    pub(crate) listenning_port: u16,
    pub(crate) routing: Arc<Mutex<RoutingTable>>,
}

impl Node {
    pub(crate) fn new(listenning_port: u16, version: usize) -> Result<Self, NodeError> {
        let (pk, _sk) = ed25519::generate_keypair();
        let public_key = pk.to_bytes();

        let id = generate_id(public_key, version).map_err(NodeError::IdError)?;

        Ok(Self {
            listenning_port,
            routing: Arc::new(Mutex::new(RoutingTable::new_from_id(id))),
        })
    }

    pub async fn start(&mut self) -> Result<(), NetworkError> {
        let port = self.listenning_port;
        let routing = self.routing.clone();

        for (id, socket_addr) in BOOTSTRAPS_ADDRESS.iter().filter(|(_, socket_addr)| {
            !socket_addr.ip().is_loopback() || socket_addr.port() != self.listenning_port
        }) {
            let mut routing_gard = self.routing.lock().await;

            let boostrap_entry = NodeEntry::new(*id, *socket_addr).await?;

            routing_gard.insert(boostrap_entry).await.unwrap();
        }

        let listener = async move {
            loop {
                if let Err(e) = listen(port, self.routing.clone()).await {
                    println!("{e:?}");
                }
            }
        };

        let refresher = async move {
            loop {
                let buckets = {
                    let rt = routing.lock().await;
                    rt.buckets.clone()
                };

                for kbucket in buckets {
                    task::spawn(async move { ping_entries(kbucket).await });
                }

                sleep(Duration::from_secs(30)).await;
            }
        };

        join!(listener, refresher);

        Ok(())
    }

    pub fn join() -> Result<(), NodeError> {
        Ok(())
    }

    pub fn stop() -> Result<(), NodeError> {
        Ok(())
    }
}
