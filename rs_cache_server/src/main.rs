use std::sync::Arc;
use rs_cache::Cache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let cache = Arc::new(
        Cache::default()
    );

    let cache_gc = cache.clone();
    let handle   = tokio::spawn(async move {
        cache_gc.gc_launch().await;
    });

    cache.set("Key", String::from("Val"), 1);
    if let Some(value) = cache.get("Key") {
        println!("{}", value);
    }

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            match cache.get("Key") {
                Some(value) => {
                    println!("{}", value);
                },
                None => {
                    println!("Not Found");
                }
            };
        }
    });

    let _ = tokio::join!(handle);
}
