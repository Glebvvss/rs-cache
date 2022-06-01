use super::shard::ShardSet;

pub struct Conf {
    shards_count: u32
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            shards_count: 10
        }
    }
}

pub struct Store {
    shard_set: ShardSet
}

impl Store {
    pub fn new(conf: Conf) -> Store {
        Store {
            shard_set: ShardSet::new(conf.shards_count)
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let shard = self.shard_set
            .get_shard(key)
            .unwrap();

        shard.get(key)
    }

    pub fn set(&self, key: &str, value: String) {
        if let Some(shard) = self.shard_set.get_shard(key) {
            shard.set(key, value);
        }
    }

    pub fn unset(&self, key: &str) {
        if let Some(shard) = self.shard_set.get_shard(key) {
            shard.unset(key);
        }
    }
}