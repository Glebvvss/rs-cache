use rs_cache::cache::{Cache, Conf};

#[tokio::main]
async fn main() {
    let cache = Cache::new(
        Conf::default()
    );

    cache.set("Key", String::from("Val"));
    if let Some(value) = cache.get("Key") {
        println!("{}", value);
    }
}
