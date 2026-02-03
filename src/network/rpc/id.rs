use crate::utils::rng::random_fill;
use cadentis::time::sleep;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{OnceLock, RwLock};
use std::time::{Duration, Instant};

type TransactionId = [u8; 4];

static TRANSACTION_IDS: OnceLock<RwLock<HashMap<TransactionId, Instant>>> = OnceLock::new();
pub(crate) static CLEANUP_STARTED: OnceLock<()> = OnceLock::new();

fn ids() -> &'static RwLock<HashMap<TransactionId, Instant>> {
    TRANSACTION_IDS.get_or_init(|| RwLock::new(HashMap::new()))
}

fn generate_id() -> TransactionId {
    let mut id = [0u8; 4];
    random_fill(&mut id);
    id
}

pub(crate) fn new_tid() -> TransactionId {
    let mut map = ids().write().unwrap();

    loop {
        let id = generate_id();

        if let Entry::Vacant(e) = map.entry(id) {
            e.insert(Instant::now());
            return id;
        }
    }
}

pub(crate) fn remove_tid(id: &TransactionId) -> bool {
    ids().write().unwrap().remove(id).is_some()
}

fn cleanup_expired() {
    let mut map = ids().write().unwrap();
    let now = Instant::now();
    let timeout = Duration::from_secs(60);

    map.retain(|_, created_at| now.duration_since(*created_at) < timeout);
}

pub(crate) async fn start_cleanup_loop() {
    loop {
        sleep(Duration::from_secs(10)).await;
        cleanup_expired();
    }
}
