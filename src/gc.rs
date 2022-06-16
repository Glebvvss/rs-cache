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
    pub fn new(store: Arc<Store>) -> Self {
        Gc {
            store,
            lifes: Arc::new(
                RwLock::new(
                    Lifes::new()
                )
            ),
            time: SystemTime::now()
        }
    }

    pub fn grab(&self, key: &str, duration_secs: u32) {
        let now = match self.time.elapsed() {
            Ok(elapsed) => elapsed.as_secs(),
            Err(_) => 0
        } as u32;

        let expiration = now + duration_secs;
        let mut lifes = self.lifes.write().unwrap();
        lifes.grab(key, expiration);
    }

    pub fn release(&self, key: &str) {
        let mut lifes = self.lifes.write().unwrap();
        lifes.release(key);
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

struct Lifes {
    vec:  Vec<(String, u32)>,
    map:  HashMap<String, (u32, u32)>,
    free: Vec<u32>
}

impl Lifes {
    fn new() -> Self {
        Lifes {
            vec:  Vec::new(),
            map:  HashMap::new(),
            free: Vec::new()
        }
    }

    #[allow(mutable_borrow_reservation_conflict)]
    fn grab(&mut self, key_str: &str, expiration: u32) {
        let key = &key_str.to_string();

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

    fn release(&mut self, key_str: &str) {
        let key = &key_str.to_string();

        if let Some((pos, _)) = self.map.get(key) {
            let position = pos.clone();
            self.map.remove(key);
            self.free.push(position);
        }
    }

    fn iter(&self) -> &Vec<(String, u32)> {
        &self.vec
    }
}

#[cfg(test)]
impl Lifes {    
    fn inner_map(&self) -> &HashMap<String, (u32, u32)> {
        &self.map
    }

    fn inner_free(&self) -> &Vec<u32> {
        &self.free
    }
}

#[cfg(test)]
mod tests {
    use super::Lifes;
    use std::time::Duration;

    #[test]
    fn lifes_grab_release() {
        let key        = "Key";
        let expiration = Duration::from_secs(120).as_secs() as u32;
        let mut lifes  = Lifes::new();

        lifes.grab(&key, expiration.clone());
        for life in lifes.iter() {
            assert_eq!(life, &(key.to_string(), expiration.clone()));
        }

        lifes.release(&key);
        assert_eq!(lifes.iter().len(), 1);
        assert_eq!(lifes.inner_free()[0], 0);
        assert_eq!(lifes.inner_map().is_empty(), true);
    }
}