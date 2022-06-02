use std::time::Duration;
use std::sync::Arc;
use super::store::Store;
use std::collections::HashMap;

pub struct Gc {
    store: Arc<Store>,
    lifes: HashMap<String, u8>,
}

impl Gc {
    pub fn new(store: Arc<Store>) -> Self {
        Gc {
            store,
            lifes: HashMap::new()
        }
    }

    pub async fn launch(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("GC tick");
        }
    }
}