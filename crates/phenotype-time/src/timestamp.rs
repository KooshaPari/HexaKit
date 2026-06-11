//! Timestamp utilities for working with chrono DateTime.
//!
//! ## Example
//!
//! ```
//! use chrono::{DateTime, Utc};
//! use phenotype_time::Timestamp;
//!
//! // Fixed reference instant (UTC) and ISO 8601 round-trip.
//! let original: DateTime<Utc> = "2024-01-15T12:34:56Z".parse().unwrap();
//! let iso = original.to_iso();
//! let parsed = DateTime::<Utc>::parse(&iso).unwrap();
//! assert_eq!(original, parsed);
//!
//! // to_utc on an already-UTC value is the identity.
//! assert_eq!(original.to_utc(), original);
//! ```
//!
//! `Timestamp::now` is non-deterministic and is therefore exercised by
//! the regular `#[test]` suite rather than doctests; run
//! `cargo test -p phenotype-time` to execute it.

use chrono::{DateTime, Utc};

/// Timestamp kind (UTC vs Offset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampKind {
    /// UTC timestamp.
    Utc,
    /// Timestamp with timezone offset.
    Offset,
}

/// Extension trait for DateTime.
pub trait Timestamp {
    /// Get current UTC timestamp.
    fn now() -> DateTime<Utc>;

    /// Parse from ISO 8601 string.
    fn parse(s: &str) -> Result<DateTime<Utc>, chrono::ParseError>;

    /// Format as ISO 8601 string.
    fn to_iso(&self) -> String;

    /// Get the kind of timestamp.
    fn kind(&self) -> TimestampKind;

    /// Convert to UTC.
    fn to_utc(&self) -> DateTime<Utc>;
}

impl Timestamp for DateTime<Utc> {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn parse(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
    }

    fn to_iso(&self) -> String {
        self.to_rfc3339()
    }

    fn kind(&self) -> TimestampKind {
        TimestampKind::Utc
    }

    fn to_utc(&self) -> DateTime<Utc> {
        *self
    }
}

/// Time-related constants.
pub mod constants {
    use chrono::{Duration as ChronoDuration, Utc};

    /// Unix epoch.
    pub const UNIX_EPOCH: chrono::DateTime<Utc> = chrono::DateTime::UNIX_EPOCH;

    /// Average seconds in a minute.
    pub const SECONDS_PER_MINUTE: i64 = 60;

    /// Average seconds in an hour.
    pub const SECONDS_PER_HOUR: i64 = 3600;

    /// Average seconds in a day.
    pub const SECONDS_PER_DAY: i64 = 86400;

    /// Average seconds in a week.
    pub const SECONDS_PER_WEEK: i64 = 604800;

    /// Average seconds in a month (30 days).
    pub const SECONDS_PER_MONTH: i64 = 2592000;

    /// Average seconds in a year (365 days).
    pub const SECONDS_PER_YEAR: i64 = 31536000;

    /// One minute as ChronoDuration.
    pub const MINUTE: ChronoDuration = ChronoDuration::minutes(1);

    /// One hour as ChronoDuration.
    pub const HOUR: ChronoDuration = ChronoDuration::hours(1);

    /// One day as ChronoDuration.
    pub const DAY: ChronoDuration = ChronoDuration::days(1);

    /// One week as ChronoDuration.
    pub const WEEK: ChronoDuration = ChronoDuration::weeks(1);
}
