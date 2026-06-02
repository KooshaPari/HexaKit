//! Configuration management framework.
//!
//! # Architecture
//!
//! settly follows hexagonal architecture:
//!
//! - **Domain**: Pure business logic (config entities, layers, validation)
//! - **Application**: Use cases and configuration builder
//! - **Adapters**: File parsers, env sources, validators
//! - **Infrastructure**: Cross-cutting concerns (error handling, logging)
//!
//! # Quick Start
//!
//! ```
//! use settly::ConfigBuilder;
//!
//! let config = ConfigBuilder::new().build().unwrap();
//! ```

pub mod domain;
pub mod application;
pub mod adapters;
pub mod infrastructure;

// Re-exports
pub use domain::{Config, ConfigValue, Layer, LayerPriority};
pub use domain::errors::ConfigError;
pub use domain::{IdempotencyKey, IdempotencyStore, SubmissionResult, DeadLetterEntry, DeadLetterQueue};
pub use application::builder::ConfigBuilder;
pub use application::submission::SubmissionService;
pub use adapters::idempotency::{InMemoryIdempotencyStore, InMemoryDlq};
pub use infrastructure::error::ConfigKitError;

/// Framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
