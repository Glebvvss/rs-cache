use rs_cache::gc::Gc;
use std::sync::Arc;
use rs_cache::store::{Store, Conf};

#[tokio::main]
async fn main() {
    let store = Arc::new(
        Store::new(
            Conf::default()
        )
    );

    let store_gc = store.clone();
    let handle = tokio::spawn(async move {
        let gc = Gc::new(store_gc);
        gc.launch().await;
    });

    store.set("Key", String::from("Val"));
    if let Some(value) = store.get("Key") {
        println!("{}", value);
    }

    store.unset("Key");
    match store.get("Key") {
        Some(_) => println!("Found"),
        None    => println!("Not exists")
    };

    let _ = tokio::join!(handle);
}
