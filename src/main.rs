use rs_cache::cache::Cache;

#[tokio::main]
async fn main() {
    let shards_count = 10;
    let cache = Cache::new(shards_count);
    cache.set(
        "Key",
        String::from("Val")
    );

    if let Some(value) = cache.get("Key") {
        println!("{}", value);
    }
}
