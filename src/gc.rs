use std::time::Duration;
use std::sync::Arc;
use super::store::Store;

pub struct Gc {
    store: Arc<Store>
}

impl Gc {
    pub fn new(store: Arc<Store>) -> Self {
        Gc {
            store
        }
    }

    pub async fn launch(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("GC tick");
        }
    }
}