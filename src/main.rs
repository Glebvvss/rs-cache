use rs_cache::cache::Cache;

#[tokio::main]
async fn main() {
    let cache = Cache::new(10);

    let key = String::from("Key");
    cache.set(
        key,
        String::from("Val")
    );

    let key = String::from("Key");
    if let Some(value) = cache.get(&key) {
        println!("{}", value);
    }
}
