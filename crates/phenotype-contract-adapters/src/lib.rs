//! HexaKit scaffold adapters for hexagonal ports.
//!
//! In-memory implementations used by `phenotype-core` re-exports. Canonical
//! contract traits live in phenoShared `phenotype-contracts`.

pub mod adapters;
pub mod error;
pub mod outbound;

pub use adapters::{InMemoryCache, InMemoryEventBus, InMemoryRepository, InMemorySecretManager};
