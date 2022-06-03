use std::sync::{Arc, RwLock};
use rs_cache::store::Store;
use rs_cache::gc::{Gc, Lifes};

#[tokio::main]
async fn main() {
    let lifes = Arc::new(RwLock::new(Lifes::new()));
    let store = Arc::new(
        Store::default()
    );

    let store_gc = store.clone();
    let lifes_gc = lifes.clone();
    let gc = Gc::new(store_gc, lifes_gc);
    let handle = tokio::spawn(async move {
        gc.launch().await;
    });

    store.set("Key", String::from("Val"));
    lifes.write()
         .unwrap()
         .grab(&"Key".to_string(), 128);

    if let Some(value) = store.get("Key") {
        println!("{}", value);
    }

    store.unset("Key");
    lifes.write()
         .unwrap()
         .release(&"Key".to_string());

    match store.get("Key") {
        Some(_) => println!("Found"),
        None    => println!("Not exists")
    };

    let _ = tokio::join!(handle);
}
