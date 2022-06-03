use rs_cache::Cache;
use rs_cache::store::Store;
use std::sync::{Arc, RwLock};
use rs_cache::gc::{Gc, Lifes};

#[tokio::main]
async fn main() {
    let cache = Arc::new(
        Cache::default()
    );

    let cache_gc = cache.clone();
    let handle   = tokio::spawn(async move {
        cache_gc.gc_launch().await;
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
