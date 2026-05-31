//! Domain layer.

pub mod cache;
pub mod policy;
pub mod ports;
pub mod errors;
pub mod entities;
pub mod events;
pub mod value_objects;

// Re-exports
pub use cache::{CacheKey, CacheValue, Entry};
pub use policy::{EvictionPolicy, LruPolicy, LfuPolicy, TtlPolicy};
pub use errors::CacheError;
pub use ports::Cache;
