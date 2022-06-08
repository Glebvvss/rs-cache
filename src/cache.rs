use crate::store::Store;
use crate::gc::{Gc, Lifes};
use std::sync::{Arc, RwLock};

pub struct Cache {
    store: Arc<Store>,
    gc:    Gc
}

impl Default for Cache {
    fn default() -> Self {
        let store = Arc::new(
            Store::default()
        );
    
        let gc = Gc::new(
            store.clone(),
            Arc::new(
                RwLock::new(
                    Lifes::new()
                )
            )
        );

        Cache::new(
            store,
            gc
        )
    }
}

impl Cache {
    pub fn new(store: Arc<Store>, gc: Gc) -> Self {
        Cache {
            store,
            gc
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key)
    }

    pub fn set(&self, key: &str, value: String, duration_secs: u32) {
        self.store.set(key, value);
        self.gc
            .lifes()
            .write()
            .unwrap()
            .grab(&key.to_string(), duration_secs);
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