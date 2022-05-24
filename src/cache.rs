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

    pub fn get(&self, key: &String) -> Option<String> {
        let shard = self.shard_set
            .get_shard(key)
            .unwrap();

        shard.get(key)
    }

    pub fn set(&self, key: String, value: String) {
        if let Some(shard) = self.shard_set.get_shard(&key) {
            shard.set(key, value);
        }
    }
}