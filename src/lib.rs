pub mod store;
pub mod gc;

pub(crate) mod common;
pub(crate) mod shard;

use crate::store::Store;
use crate::gc::{Gc, Lifes};
use std::sync::{Arc, RwLock};

pub struct Cache {
    store: Arc<Store>,
    gc:    Arc<Gc>
}

impl Cache {
    pub fn new(store: Arc<Store>, gc: Arc<Gc>) -> Self {
        Cache {
            store,
            gc
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key)
    }

    pub fn set(&self, key: &str, value: String) {
        self.store.set(key, value);
        self.gc
            .lifes()
            .write()
            .unwrap()
            .grab(&key.to_string(), 128);
    }

    pub fn unset(&self, key: &str) {
        self.store.unset(key);
        self.gc
            .lifes()
            .write()
            .unwrap()
            .release(&key.to_string());
    }

    pub async fn gc_launch(&self) {
        self.gc.launch().await;
    }
}