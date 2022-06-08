use super::store::Store;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

pub struct Gc {
    store: Arc<Store>,
    lifes: Arc<RwLock<Lifes>>,
    time:  SystemTime
}

impl Gc {
    pub fn new(store: Arc<Store>, lifes: Arc<RwLock<Lifes>>) -> Self {
        Gc {
            store,
            lifes,
            time: SystemTime::now()
        }
    }

    pub fn lifes(&self) -> Arc<RwLock<Lifes>> {
        self.lifes.clone()
    }

    pub fn grab(&self, key: &String, duration_secs: u32) {
        let now = match self.time.elapsed() {
            Ok(elapsed) => elapsed.as_secs(),
            Err(_) => 0
        } as u32;

        let expiration = now + duration_secs;
        let mut lifes = self.lifes.write().unwrap();
        lifes.grab(key, expiration);
    }

    pub async fn launch(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;

            let now_secs = match self.time.elapsed() {
                Ok(elapsed) => elapsed.as_secs(),
                Err(_)      => 0
            } as  u32;

            let mut expired_keys = Vec::new();
            {
                let lifes = self.lifes.read().unwrap();
                for (key, expiration) in lifes.iter() {
                    if now_secs >= *expiration {
                        expired_keys.push(key.to_string());
                    }
                }
            }

            let mut lifes = self.lifes.write().unwrap();
            for key in expired_keys {
                self.store.unset(&key);
                lifes.release(&key);
            }
        }
    }
}

pub struct Lifes {
    vec:  Vec<(String, u32)>,
    map:  HashMap<String, (u32, u32)>,
    free: Vec<u32>
}

impl Lifes {
    pub fn new() -> Self {
        Lifes {
            vec:  Vec::new(),
            map:  HashMap::new(),
            free: Vec::new()
        }
    }

    pub fn grabbed(&self, key: &String) -> bool {
        if self.map.is_empty() {
            return false;
        }

        match self.map.get(key) {
            Some(_) => true,
            None    => false
        }
    }

    pub fn expire_at(&self, key: &String) -> Option<u32> {
        if self.map.is_empty() {
            return None;
        }

        match self.map.get(key) {
            Some((_, expiration)) => Some(expiration.clone()),
            None => None
        }
    }

    pub fn grab(&mut self, key: &String, expiration: u32) {
        match self.map.get(key) {
            Some((position, _)) => {
                self.map.insert(key.to_string(), (*position, expiration));
            },
            None => {
                let position = match self.free.pop() {
                    Some(position) => position,
                    None           => self.vec.len() as u32
                };

                self.map.insert(key.to_string(), (position, expiration));
                self.vec.insert(position as usize, (key.to_string(), expiration));
            }
        };
    }

    pub fn release(&mut self, key: &String) {
        if let Some((pos, _)) = self.map.get(key) {
            let position = pos.clone();
            self.map.remove(key);
            self.free.push(position);
        }
    }

    pub fn iter(&self) -> &Vec<(String, u32)> {
        &self.vec
    }
}