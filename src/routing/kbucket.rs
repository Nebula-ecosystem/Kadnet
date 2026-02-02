use super::entry::NodeEntry;
use super::errors::BucketError;
use crate::network::rpc::ping;

use cadentis::sync::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

pub(crate) enum InsertDecision {
    Inserted,
    PingOldest(NodeEntry),
    Refreshed,
}

pub(crate) struct KBucket {
    entries: VecDeque<NodeEntry>,
    capacity: usize,
}

impl KBucket {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        self.entries.len() == self.capacity
    }

    pub(crate) fn try_insert(&mut self, entry: NodeEntry) -> InsertDecision {
        if let Some(position) = self.entries.iter().position(|ne| ne.id == entry.id) {
            let ne = self.entries.remove(position).unwrap();
            self.entries.push_back(ne);

            return InsertDecision::Refreshed;
        }

        if !self.is_full() {
            self.entries.push_back(entry);
            InsertDecision::Inserted
        } else {
            InsertDecision::PingOldest(*self.entries.front().unwrap())
        }
    }

    pub(crate) fn force_insert(&mut self, entry: NodeEntry) {
        self.entries.push_back(entry);
    }

    pub(crate) fn remove(&mut self, entry: NodeEntry) -> Result<(), BucketError> {
        if let Some(position) = self.entries.iter().position(|ne| ne.id == entry.id) {
            self.entries.remove(position).unwrap();

            Ok(())
        } else {
            Err(BucketError::NodeNotFound)
        }
    }

    pub(crate) fn snapshot(&self) -> Vec<NodeEntry> {
        self.entries.iter().cloned().collect()
    }

    pub(crate) fn update_entry_respond_time(&mut self, entry: &NodeEntry, duration: Duration) {
        if let Some(entry) = self.entries.iter_mut().find(|ne| ne.id == entry.id) {
            entry.update_respond_time(duration);
        }
    }
}

pub(crate) async fn ping_entries(kbucket: Arc<Mutex<KBucket>>) {
    let mut entries = {
        let bucket = kbucket.lock().await;
        bucket.snapshot()
    };

    for entry in entries.iter_mut() {
        let res = ping(entry.addr).await;
        let mut bucket = kbucket.lock().await;

        match res {
            Ok(respond_time) => bucket.update_entry_respond_time(entry, respond_time),
            Err(_) => {
                let _ = bucket.remove(*entry);
            }
        }
    }
}
