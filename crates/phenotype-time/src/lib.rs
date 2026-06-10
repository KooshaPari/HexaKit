//! `phenotype-time`
//!
//! Time and duration utilities for the Phenotype ecosystem.
//!
//! ## Features
//!
//! - **[`DurationExt`]** — ergonomic constructors and a human-readable
//!   [`format_human`](DurationExt::format_human) formatter for
//!   [`std::time::Duration`].
//! - **[`Timestamp`]** — UTC [`chrono::DateTime<Utc>`] extensions with
//!   ISO 8601 parse/format round-trip via [`Timestamp::to_iso`] /
//!   [`Timestamp::parse`].
//! - **Constants** — pre-defined TTLs, timeouts, retry/backoff windows
//!   and per-unit seconds multipliers (see [`duration_constants`] and
//!   [`time_constants`]).
//!
//! ## Quick start
//!
//! ```
//! use std::time::Duration;
//! use chrono::{DateTime, Utc};
//! use phenotype_time::{DurationExt, Timestamp};
//!
//! // Build a duration with the trait and format it.
//! let d = Duration::minutes(2) + Duration::seconds(30);
//! assert_eq!(d.as_secs(), 150);
//! assert_eq!(d.format_human(), "2m 30s");
//!
//! // ISO 8601 round-trip.
//! let original: DateTime<Utc> = "2024-01-15T12:34:56Z".parse().unwrap();
//! let iso = original.to_iso();
//! let parsed = DateTime::<Utc>::parse(&iso).unwrap();
//! assert_eq!(original, parsed);
//! assert_eq!(parsed.to_utc(), original);
//! ```

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod duration;
pub mod timestamp;

pub use duration::constants as duration_constants;
pub use duration::DurationExt;
pub use timestamp::constants as time_constants;
pub use timestamp::Timestamp;
