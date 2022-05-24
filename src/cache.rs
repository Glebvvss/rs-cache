use super::shard::ShardSet;

pub struct Cache {
    shard_set: ShardSet
}

impl Cache {
    pub fn new(shards_count: u32) -> Cache {
        Cache {
            shard_set: ShardSet::new(shards_count)
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let k = key.to_string();

        let shard = self.shard_set
            .get_shard(&k)
            .unwrap();

        shard.get(&k)
    }

    pub fn set(&self, key: &str, value: String) {
        let k = key.to_string();

        if let Some(shard) = self.shard_set.get_shard(&k) {
            shard.set(k, value);
        }
    }
}