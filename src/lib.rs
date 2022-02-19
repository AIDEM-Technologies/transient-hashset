mod tests;

use hashbrown::HashMap;
use std::{hash::Hash, sync::Arc};
use tokio::{sync::*, time::{Duration, Instant}};

pub struct Perfect<T: Eq + Hash, S: Eq + Hash> {
    bad_boy: Arc<RwLock<HashMap<T, Arc<Mutex<HashMap<S, Instant>>>>>>,
    ttl: Duration, //Time to live
}

impl<T: Eq + Hash, S: Eq + Hash> Perfect<T, S>{
    pub fn new(ttl: Duration) -> Self {
        Self {
            bad_boy: Arc::new(RwLock::new(HashMap::new())),
            ttl: ttl,
        }
    }
    pub async fn contains(&self, first: T, second: S) -> bool {
        let mut flag = false;
        let inner = {
            if let Some(t) = self.bad_boy.read().await.get(&first) {
                t.clone()
            }
            else {
                flag = true;
                let arc = Arc::new(Mutex::new(HashMap::new()));
                arc.clone()
            }
        };
        let mut innerer = inner.lock().await;
        let now = Instant::now();
        innerer.retain(|_, v| *v + self.ttl > now);
        match innerer.contains_key(&second) {
            true => true,
            false => {
                innerer.insert(second, now);
                if flag {
                    self.bad_boy.write().await.entry(first).or_insert(inner.clone());
                }
                false
            }
        }
    }
}

pub struct Naive<T: Eq + Hash + Clone, S: Eq + Hash + Clone>{
    bad_boy: Arc<Mutex<HashMap<(T, S), Instant>>>,
    ttl: Duration,
}

impl<T: Eq + Hash + Clone, S: Eq + Hash + Clone> Naive<T, S> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            bad_boy: Arc::new(Mutex::new(HashMap::new())),
            ttl: ttl,
        }
    }

    pub async fn contains(&self, first: T, second: S) -> bool {
        let mut map = self.bad_boy.lock().await;
        if map.len() > 10_000_000 {
            let now = Instant::now();
            map.retain(|_k, v| now - *v < self.ttl);
        }
        match map.get(&(first.clone(), second.clone())) {
            Some(value) => {
                if Instant::now() - *value > self.ttl {
                    map.insert((first, second), Instant::now());
                    false
                }
                else {
                    true
                }
            }
            None => {
                map.insert((first, second), Instant::now());
                false
            }
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct MyInstant(Instant);

impl From<Instant> for MyInstant {
    fn from(other: Instant) -> Self {
        Self(other)
    }
}

impl evmap::ShallowCopy for MyInstant {
    unsafe fn shallow_copy(&self) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(*self)
    }
}

/// Our main transient hashset that can be safely referenced from different threads.
pub struct TransientHashSet<T: Eq + Hash + Clone, S: Eq + Hash + Clone> {
    bad_boy_r: evmap::ReadHandle<(T, S), MyInstant>,
    bad_boy_w: Arc<Mutex<evmap::WriteHandle<(T, S), MyInstant>>>,
    ttl: Duration, //Time to live
}

impl<T: Eq + Hash + Clone, S: Eq + Hash + Clone> TransientHashSet<T, S>{
    /// Creates a new <TransientHashSet> with a give time to live for its elements
    pub fn new(ttl: Duration) -> Self {
        let (r, w) = evmap::new();
        Self {
            bad_boy_r: r,
            bad_boy_w: Arc::new(Mutex::new(w)),
            ttl: ttl,
        }
    }
    /// Checks if an element is contained in the hashset.  
    /// This also updates the hashset itself with the new element.
    pub async fn contains(&self, first: T, second: S) -> bool {
        match self.bad_boy_r.get_one(&(first.clone(), second.clone())) {
            Some(value) => {
                if Instant::now() - value.0 > self.ttl {
                    self.bad_boy_w.lock().await.insert((first, second), Instant::now().into());
                    self.bad_boy_w.lock().await.refresh();
                    false
                }
                else {
                    true
                }
            },
            None => {
                self.bad_boy_w.lock().await.insert((first, second), Instant::now().into());
                self.bad_boy_w.lock().await.refresh();
                false
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Flurry{
    boy: flurry::HashMap<(String, String), Instant>,
    ttl: Duration,
}

impl Flurry {
    pub fn new(ttl: Duration) -> Self {
        Self {
            boy: flurry::HashMap::new(),
            ttl: ttl,
        }
    }
    pub fn contains(&self, first: String, second: String) -> bool {
        if self.boy.pin().len() > 10000000 {
            let now = Instant::now();
            self.boy.pin().retain(|_k, v| now - *v < self.ttl);
        }
        match self.boy.pin().get(&(first.clone(), second.clone())) {
            Some(value) => {
                if Instant::now() - *value > self.ttl {
                    self.boy.pin().insert((first, second), Instant::now());
                    false
                }
                else {
                    true
                }
            }
            None => {
                self.boy.pin().insert((first, second), Instant::now());
                false
            }
        }
    }
}