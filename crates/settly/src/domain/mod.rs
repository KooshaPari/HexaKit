//! Domain layer - pure configuration logic.

pub mod config;
pub mod layers;
pub mod sources;
pub mod validation;
pub mod ports;
pub mod errors;
pub mod idempotency;

// Re-exports
pub use config::{Config, ConfigValue, ConfigPath};
pub use layers::{Layer, LayerPriority, LayerStack, MergeStrategy};
pub use sources::Source;
pub use validation::Validator;
pub use errors::ConfigError;
pub use idempotency::{
    IdempotencyKey, IdempotencyStore, SubmissionResult,
    DeadLetterEntry, DeadLetterQueue,
};
