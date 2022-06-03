use std::sync::{Arc, RwLock};
use rs_cache::store::Store;
use rs_cache::gc::{Gc, Lifes};

pub struct Cache {
    store: Arc<Store>,
    gc:    Arc<Gc>
}

impl Cache {
    fn new(store: Arc<Store>, gc: Arc<Gc>) -> Self {
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
}

#[tokio::main]
async fn main() {
    let store = Arc::new(
        Store::default()
    );

    let gc = Arc::new(
        Gc::new(
            store.clone(),
            Arc::new(
                RwLock::new(
                    Lifes::new()
                )
            )
        )
    );

    let cache = Cache::new(store.clone(), gc.clone());

    let handle = tokio::spawn(async move {
        gc.launch().await;
    });

    cache.set("Key", String::from("Val"));
    if let Some(value) = cache.get("Key") {
        println!("{}", value);
    }

    cache.unset("Key");
    match cache.get("Key") {
        Some(_) => println!("Found"),
        None    => println!("Not exists")
    };

    let _ = tokio::join!(handle);
}
