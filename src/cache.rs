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

pub struct Cache {
    shard_set: ShardSet
}

impl Cache {
    pub fn new(conf: Conf) -> Cache {
        Cache {
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
}