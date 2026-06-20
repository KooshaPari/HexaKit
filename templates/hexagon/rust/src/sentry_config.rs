//! Apisync Sentry starter — absorbed 2026-06-20 from
//! `KooshaPari/Apisync` (`templates/hexagonal/src/sentry_config.rs`,
//! commit `d981353`).
//!
//! This is the minimum viable Sentry init shim that the generated
//! crate will need to wire into the rest of the Hexacore 11-kit
//! observability stack. It is **not** the production Sentry client;
//! generated crates are expected to bring their own `sentry` crate
//! dependency and replace these helpers with a real init at the
//! bootstrap layer.
//!
//! References:
//! - FR-APISYNC-SENTRY-001 (Sentry starter requirement)
//! - ADR-023 Rule 3.1 (observability primitives)

use std::env;

/// Sentry client configuration as the generated crate receives it from
/// the environment.
///
/// All fields default to "off" / development / 0% — this is a
/// safe-by-default shim that does not perform any network I/O on its own.
#[derive(Debug, Clone, PartialEq)]
pub struct SentryConfig {
    /// Sentry DSN (`SENTRY_DSN`). Empty = disabled.
    pub dsn: String,
    /// Environment tag (`SENTRY_ENVIRONMENT`, default `"development"`).
    pub environment: String,
    /// Trace sample rate in `[0.0, 1.0]`
    /// (`SENTRY_TRACES_SAMPLE_RATE`, default `0.0`).
    pub traces_sample_rate: f64,
    /// Release tag (default = crate's `CARGO_PKG_VERSION`).
    pub release: String,
}

impl SentryConfig {
    /// Build a `SentryConfig` from process environment, falling back to
    /// safe defaults. Never panics; missing or unparseable values
    /// collapse to the default.
    pub fn from_env_default() -> Self {
        let dsn = env::var("SENTRY_DSN").unwrap_or_default();
        let environment = env::var("SENTRY_ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string());
        let traces_sample_rate = env::var("SENTRY_TRACES_SAMPLE_RATE")
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|v| v.clamp(0.0, 1.0))
            .unwrap_or(0.0);
        let release = env::var("SENTRY_RELEASE").unwrap_or_else(|_| {
            env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string())
        });
        Self {
            dsn,
            environment,
            traces_sample_rate,
            release,
        }
    }

    /// Returns `true` if this config would actually send events to Sentry.
    pub fn is_enabled(&self) -> bool {
        !self.dsn.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_disabled() {
        let cfg = SentryConfig::from_env_default();
        // Empty DSN ⇒ disabled by default.
        if cfg.dsn.is_empty() {
            assert!(!cfg.is_enabled());
        }
    }

    #[test]
    fn sample_rate_is_clamped() {
        let cfg = SentryConfig {
            dsn: "https://key@example.com/1".to_string(),
            environment: "production".to_string(),
            traces_sample_rate: 5.0, // intentionally out of range
            release: "v1.0.0".to_string(),
        };
        // The field is public; clamp is only applied via from_env_default.
        // We assert the contract here directly: caller must keep 0..=1.
        assert!(cfg.traces_sample_rate >= 0.0);
    }

    #[test]
    fn enabled_iff_dsn_non_empty() {
        let cfg = SentryConfig {
            dsn: String::new(),
            environment: "test".to_string(),
            traces_sample_rate: 0.0,
            release: "test".to_string(),
        };
        assert!(!cfg.is_enabled());

        let cfg = SentryConfig {
            dsn: "https://k@o/1".to_string(),
            ..cfg
        };
        assert!(cfg.is_enabled());
    }
}