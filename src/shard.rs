use std::sync::RwLock;
use std::collections::HashMap;
use super::common::check_sum_from_str;

pub(crate) struct ShardSet {
    shards: Vec<Shard>
}

impl ShardSet {
    pub(crate) fn new(shards_count: u32) -> ShardSet {
        let mut shards = Vec::new();
        for _ in 0..shards_count {
            shards.push(Shard::new());
        }

        let shards = shards; 
        ShardSet {
            shards
        }
    }

    pub(crate) fn get_shard(&self, key: &str) -> Option<&Shard> {
        self.shards.get(
            self.get_shard_key(key)
        )
    }

    fn get_shard_key(&self, key: &str) -> usize {
        check_sum_from_str(key) % self.shards.len()
    }
}

pub(crate) struct Shard {
    inner: RwLock<HashMap<String, String>>
}

impl Shard {
    pub(crate) fn new() -> Self {
        Shard {
            inner: RwLock::new(HashMap::new())
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<String> {
        let k = key.to_string();
        let inner = self.inner.read().unwrap();
        match inner.get(&k) {
            Some(value) => Some(value.clone()),
            None        => None
        }
    }

    pub(crate) fn set(&self, key: &str, value: String) {
        let k = key.to_string();
        let mut inner = self.inner.write().unwrap();
        inner.insert(k, value);
    }
}