use std::time::Duration;
use std::sync::Arc;
use super::cache::Cache;

pub struct Gc {
    cache: Arc<Cache>
}

impl Gc {
    pub fn new(cache: Arc<Cache>) -> Self {
        Gc {
            cache
        }
    }

    pub async fn launch(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("GC tick");
        }
    }
}