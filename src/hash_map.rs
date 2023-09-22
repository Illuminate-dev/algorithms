use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

const INITIAL_BUCKETS: usize = 1;

pub struct HashMap<K, V, S: BuildHasher = RandomState> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
    build_hash: S,
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
            build_hash: RandomState::new(),
        }
    }
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let mut hasher = self.build_hash.build_hasher();
        key.hash(&mut hasher);
        let bucket = hasher.finish() % self.buckets.len() as u64;
        let bucket = &mut self.buckets[bucket as usize];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                use std::mem;
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        None
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS,
            n => n * 2,
        };

        let mut buckets = Vec::with_capacity(target_size);
        buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = self.build_hash.build_hasher();
            key.hash(&mut hasher);

            let bucket = (hasher.finish() % buckets.len() as u64) as usize;
            buckets[bucket].push((key, value));
        }

        std::mem::replace(&mut self.buckets, buckets);
    }
}
