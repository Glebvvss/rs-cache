use rs_cache::gc::Gc;
use std::sync::Arc;
use rs_cache::cache::{Cache, Conf};

#[tokio::main]
async fn main() {
    let cache = Arc::new(
        Cache::new(
            Conf::default()
        )
    );

    let cache_gc = cache.clone();
    let handle = tokio::spawn(async move {
        let gc = Gc::new(cache_gc);
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
