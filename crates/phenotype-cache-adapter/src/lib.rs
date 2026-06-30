//! phenotype-cache-adapter
//!
//! Two-tier cache with L1 (LRU) and L2 (Moka), plus the `CacheAdapter` trait
//! that consuming crates (e.g. `phenotype-core`) re-export.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Generic cache port trait.  Implement this on any backing store that should
/// be swappable without changing callsite code.
pub trait CacheAdapter: Send + Sync {
    type Key: Clone + Eq + std::hash::Hash + Send + Sync + Debug + 'static;
    type Value: Clone + Send + Sync + Debug + 'static;

    /// Retrieve a value by key.  Returns `None` on a miss.
    fn get(&self, key: &Self::Key) -> Option<Self::Value>;

    /// Insert or replace a value.
    fn put(&self, key: Self::Key, value: Self::Value);

    /// Remove a key, returning the previous value if it existed.
    fn remove(&self, key: &Self::Key) -> Option<Self::Value>;
}

/// Metrics hook for observability.
pub trait MetricsHook: Send + Sync + Debug {
    fn record_hit(&self, tier: &str);
    fn record_miss(&self, tier: &str);
}

#[derive(Clone, Serialize, Deserialize)]
struct CacheEntry<V> {
    value: V,
}

/// Two-tier cache implementation.
pub struct TwoTierCache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + Debug + 'static,
    V: Clone + Send + Sync + Debug + 'static,
{
    l1: std::sync::Arc<std::sync::Mutex<lru::LruCache<K, CacheEntry<V>>>>,
    l2: moka::sync::Cache<K, CacheEntry<V>>,
}

impl<K, V> TwoTierCache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + Debug + 'static,
    V: Clone + Send + Sync + Debug + 'static,
{
    pub fn new(l1_cap: usize, l2_cap: u64) -> Self {
        Self {
            l1: std::sync::Arc::new(std::sync::Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(l1_cap)
                    .unwrap_or(std::num::NonZeroUsize::new(100).unwrap()),
            ))),
            l2: moka::sync::Cache::builder().max_capacity(l2_cap).build(),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut l1 = self.l1.lock().unwrap();
        if let Some(entry) = l1.get(key) {
            return Some(entry.value.clone());
        }
        drop(l1);

        if let Some(entry) = self.l2.get(key) {
            let value = entry.value.clone();
            let mut l1 = self.l1.lock().unwrap();
            l1.put(
                key.clone(),
                CacheEntry {
                    value: value.clone(),
                },
            );
            return Some(value);
        }
        None
    }

    pub fn put(&self, key: K, value: V) {
        let mut l1 = self.l1.lock().unwrap();
        l1.put(
            key.clone(),
            CacheEntry {
                value: value.clone(),
            },
        );
        drop(l1);
        self.l2.insert(key, CacheEntry { value });
    }
}
