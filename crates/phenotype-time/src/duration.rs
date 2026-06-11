//! Duration extension traits and utilities.
//!
//! ## Example
//!
//! ```
//! use std::time::Duration;
//! use phenotype_time::DurationExt;
//!
//! let d = Duration::hours(2) + Duration::minutes(15) + Duration::seconds(7);
//! assert_eq!(d.as_secs(), 2 * 3600 + 15 * 60 + 7);
//! assert_eq!(d.format_human(), "2h 15m 7s");
//!
//! // Zero is rendered as "0s" rather than an empty string.
//! assert_eq!(Duration::seconds(0).format_human(), "0s");
//! ```

use std::time::Duration;

/// Duration extension trait for convenience methods.
pub trait DurationExt {
    /// Create a Duration from the given number of seconds.
    fn seconds(s: u64) -> Duration;

    /// Create a Duration from the given number of minutes.
    fn minutes(m: u64) -> Duration;

    /// Create a Duration from the given number of hours.
    fn hours(h: u64) -> Duration;

    /// Create a Duration from the given number of days.
    fn days(d: u64) -> Duration;

    /// Create a Duration from the given number of milliseconds.
    fn millis(ms: u64) -> Duration;

    /// Format as human-readable string (e.g., "5m 30s").
    ///
    /// Renders a `Duration` as a space-separated list of `<n><unit>`
    /// components (days, hours, minutes, seconds). Zero-valued components
    /// are omitted, and a zero `Duration` is rendered as `"0s"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use phenotype_time::DurationExt;
    ///
    /// assert_eq!(Duration::seconds(45).format_human(), "45s");
    /// assert_eq!(
    ///     (Duration::days(1) + Duration::hours(2) + Duration::minutes(3) + Duration::seconds(4))
    ///         .format_human(),
    ///     "1d 2h 3m 4s",
    /// );
    /// ```
    fn format_human(&self) -> String;
}

impl DurationExt for Duration {
    fn seconds(s: u64) -> Duration {
        Duration::from_secs(s)
    }

    fn minutes(m: u64) -> Duration {
        Duration::from_secs(m * 60)
    }

    fn hours(h: u64) -> Duration {
        Duration::from_secs(h * 3600)
    }

    fn days(d: u64) -> Duration {
        Duration::from_secs(d * 86400)
    }

    fn millis(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }

    fn format_human(&self) -> String {
        let total_secs = self.as_secs();
        let days = total_secs / 86400;
        let hours = (total_secs % 86400) / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        let mut parts = Vec::new();
        if days > 0 {
            parts.push(format!("{days}d"));
        }
        if hours > 0 {
            parts.push(format!("{hours}h"));
        }
        if minutes > 0 {
            parts.push(format!("{minutes}m"));
        }
        if seconds > 0 || parts.is_empty() {
            parts.push(format!("{seconds}s"));
        }

        parts.join(" ")
    }
}

/// Well-known duration constants for common use cases.
pub mod constants {
    use std::time::Duration;

    /// One second.
    pub const SECOND: Duration = Duration::from_secs(1);

    /// One minute (60 seconds).
    pub const MINUTE: Duration = Duration::from_secs(60);

    /// One hour (3600 seconds).
    pub const HOUR: Duration = Duration::from_secs(3600);

    /// One day (86400 seconds).
    pub const DAY: Duration = Duration::from_secs(86400);

    /// One week (604800 seconds).
    pub const WEEK: Duration = Duration::from_secs(604800);

    // Cache TTLs
    /// 30 seconds - short-lived cache entries.
    pub const CACHE_TTL_SHORT: Duration = Duration::from_secs(30);

    /// 5 minutes - medium-lived cache entries.
    pub const CACHE_TTL_MEDIUM: Duration = Duration::from_secs(300);

    /// 15 minutes - long-lived cache entries.
    pub const CACHE_TTL_LONG: Duration = Duration::from_secs(900);

    /// 1 hour - session cache entries.
    pub const CACHE_TTL_SESSION: Duration = Duration::from_secs(3600);

    // Timeouts
    /// 5 seconds - fast operations.
    pub const TIMEOUT_FAST: Duration = Duration::from_secs(5);

    /// 30 seconds - normal operations.
    pub const TIMEOUT_NORMAL: Duration = Duration::from_secs(30);

    /// 60 seconds - slow operations.
    pub const TIMEOUT_SLOW: Duration = Duration::from_secs(60);

    /// 5 minutes - batch operations.
    pub const TIMEOUT_BATCH: Duration = Duration::from_secs(300);

    // Retries
    /// 100 milliseconds - fast retry.
    pub const RETRY_FAST: Duration = Duration::from_millis(100);

    /// 500 milliseconds - normal retry.
    pub const RETRY_NORMAL: Duration = Duration::from_millis(500);

    /// 1 second - slow retry.
    pub const RETRY_SLOW: Duration = Duration::from_secs(1);

    /// Exponential backoff base (2 seconds).
    pub const BACKOFF_BASE: Duration = Duration::from_secs(2);

    /// Exponential backoff max (60 seconds).
    pub const BACKOFF_MAX: Duration = Duration::from_secs(60);
}
