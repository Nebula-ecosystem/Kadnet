use super::entry::NodeEntry;
use super::errors::RoutingError;
use super::kbucket::InsertDecision;
use super::kbucket::KBucket;
use crate::consts::{KUSIZE, N_BUCKETS, SMALL_BUCKET_COUNT};
use crate::network::rpc::ping;

use cadentis::sync::Mutex;
use cryptal::primitives::U256;
use std::array;
use std::sync::Arc;

pub(crate) struct RoutingTable {
    local_id: U256,
    pub(crate) buckets: [Arc<Mutex<KBucket>>; N_BUCKETS],
}

impl RoutingTable {
    pub(crate) fn new_from_id(id: U256) -> Self {
        let buckets = array::from_fn(|i| {
            let size = if i <= SMALL_BUCKET_COUNT {
                1usize << i
            } else {
                KUSIZE
            };

            Arc::new(Mutex::new(KBucket::new(size)))
        });

        Self {
            local_id: id,
            buckets,
        }
    }

    pub(crate) fn find_corresponding_bucket(&self, target: U256) -> Option<usize> {
        let distance = self.local_id ^ target;

        if distance == U256::ZERO {
            return None;
        }

        Some(N_BUCKETS - 1 - distance.leading_zeros() as usize)
    }

    pub(crate) async fn insert(&mut self, entry: NodeEntry) -> Result<(), RoutingError> {
        let bucket_id = match self.find_corresponding_bucket(entry.id) {
            Some(bi) => bi,
            None => return Err(RoutingError::SelfNode),
        };

        let oldest_to_ping = {
            let mut bucket = self.buckets[bucket_id].lock().await;
            match bucket.try_insert(entry) {
                InsertDecision::PingOldest(oldest) => Some(oldest),
                InsertDecision::Inserted | InsertDecision::Refreshed => None,
            }
        };

        if let Some(oldest) = oldest_to_ping {
            let ping_result = ping(oldest.addr).await.map_err(RoutingError::NetworkError);

            if ping_result.is_err() {
                let mut bucket = self.buckets[bucket_id].lock().await;

                bucket.remove(oldest).map_err(RoutingError::BucketError)?;
                bucket.force_insert(entry);
            }
        }

        Ok(())
    }
}
