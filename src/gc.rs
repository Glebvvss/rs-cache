use std::time::Duration;
use super::store::Store;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub struct Gc {
    store:   Arc<Store>,
    lifes:   RwLock<HashMap<String, u32>>,
}

struct Lifes {
    vec:  RwLock<Vec<(String, u32)>>,
    map:  RwLock<HashMap<String, (u32, u32)>>,
    free: RwLock<Vec<u32>>
}

impl Gc {
    pub fn new(store: Arc<Store>) -> Self {
        Gc {
            store,
            lifes: RwLock::new(HashMap::new())
        }
    }

    pub async fn launch(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(100)).await;
        }
    }
}

impl Lifes {
    fn new() -> Self {
        Lifes {
            vec:  RwLock::new(Vec::new()),
            map:  RwLock::new(HashMap::new()),
            free: RwLock::new(Vec::new())
        }
    }

    fn has(&self, key: &String) -> bool {
        let map = self.map.read().unwrap();
        if map.is_empty() {
            return false;
        }

        match map.get(key) {
            Some(_) => true,
            None    => false
        }
    }

    fn expire_at(&self, key: &String) -> Option<u32> {
        let map = self.map.read().unwrap();
        if map.is_empty() {
            return None;
        }

        match map.get(key) {
            Some((_, expiration)) => Some(expiration.clone()),
            None => None
        }
    }

    fn grab(&self, key: &String, expiration: u32) {
        let mut map  = self.map.write().unwrap();
        let mut vec  = self.vec.write().unwrap();
        let mut free = self.free.write().unwrap();
        match map.get(key) {
            Some((mut position, _)) => {
                map.insert(key.to_string(), (position, expiration));
            },
            None => {
                let position = match free.pop() {
                    Some(position) => position,
                    None           => vec.len() as u32
                };

                map.insert(key.to_string(), (position, expiration));
                vec.insert(position as usize, (key.to_string(), expiration));
            }
        };
    }

    fn release(&self, key: &String) {
        let mut map  = self.map.write().unwrap();
        let mut free = self.free.write().unwrap();
        if let Some((pos, _)) = map.get(key) {
            let mut position = pos.clone();
            map.remove(key);
            free.push(position);
        }
    }
}