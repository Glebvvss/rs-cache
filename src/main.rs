use std::sync::Arc;
use rs_cache::gc::Gc;
use rs_cache::store::Store;

#[tokio::main]
async fn main() {
    let store = Arc::new(
        Store::default()
    );

    let store_gc = store.clone();
    let gc = Gc::new(store_gc);

    let handle = tokio::spawn(async move {
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
