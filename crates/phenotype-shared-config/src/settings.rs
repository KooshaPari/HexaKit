//! # HexaKit Unified Settings
//!
//! Consolidates all hardcoded configuration values from across the HexaKit
//! workspace into a single, deserializable settings struct. Supports loading
//! from TOML files and environment variables via `figment`.
//!
//! ## Sources (highest-to-lowest priority)
//!
//! 1. Environment variables prefixed with `HEXAKIT_` (e.g. `HEXAKIT_BULKHEAD_MAX_CONCURRENT=20`)
//! 2. Config file passed via `HEXAKIT_CONFIG` env var or `./hexakit.toml`
//! 3. Built-in defaults below

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Top-level settings
// ---------------------------------------------------------------------------

/// Consolidated settings for the entire HexaKit workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HexaKitSettings {
    /// Bulkhead settings (resource isolation).
    pub bulkhead: BulkheadSettings,
    /// Circuit-breaker settings (fault tolerance).
    pub circuit: CircuitSettings,
    /// Rate-limiter settings (token bucket).
    pub rate_limit: RateLimitSettings,
    /// Connection-pool settings.
    pub pool: PoolSettings,
    /// Path / directory settings.
    pub paths: PathSettings,
    /// Security scoring settings.
    pub security: SecuritySettings,
    /// Repository cache TTL settings.
    pub ttl: TtlSettings,
}

impl Default for HexaKitSettings {
    fn default() -> Self {
        Self {
            bulkhead: BulkheadSettings::default(),
            circuit: CircuitSettings::default(),
            rate_limit: RateLimitSettings::default(),
            pool: PoolSettings::default(),
            paths: PathSettings::default(),
            security: SecuritySettings::default(),
            ttl: TtlSettings::default(),
        }
    }
}

impl HexaKitSettings {
    /// Load settings from the default sources (env + optional file + defaults).
    ///
    /// The provider chain is:
    ///   `Defaults` ← `File(./hexakit.toml or $HEXAKIT_CONFIG)` ← `Env(HEXAKIT_*)`
    pub fn load() -> Result<Self, LoadError> {
        use figment::providers::{Env, Format, Toml};
        use figment::Figment;

        let config_path = std::env::var("HEXAKIT_CONFIG")
            .ok()
            .filter(|p| !p.is_empty())
            .unwrap_or_else(|| "./hexakit.toml".to_string());

        let figment = Figment::new()
            .merge(serde_figment_defaults())
            .merge(Toml::file(&config_path))
            .merge(Env::prefixed("HEXAKIT_").global().split('_'));
        figment
            .extract()
            .map_err(|e| LoadError::Figment(e.to_string()))
    }

    /// Load settings from a specific file path, with env overrides and defaults.
    pub fn load_from(path: &str) -> Result<Self, LoadError> {
        use figment::providers::{Env, Format, Toml};
        use figment::Figment;

        let figment = Figment::new()
            .merge(serde_figment_defaults())
            .merge(Toml::file(path))
            .merge(Env::prefixed("HEXAKIT_").global().split('_'));
        figment
            .extract()
            .map_err(|e| LoadError::Figment(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// Bulkhead
// ---------------------------------------------------------------------------

/// Bulkhead (resource isolation) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BulkheadSettings {
    /// Maximum number of concurrent executions.
    pub max_concurrent: usize,
    /// Maximum number of queued waiters.
    pub max_queue: usize,
    /// Queue timeout in milliseconds.
    pub queue_timeout_ms: u64,
}

impl Default for BulkheadSettings {
    fn default() -> Self {
        Self {
            max_concurrent: 10,
            max_queue: 20,
            queue_timeout_ms: 1000,
        }
    }
}

// ---------------------------------------------------------------------------
// Circuit breaker
// ---------------------------------------------------------------------------

/// Circuit-breaker (fault tolerance) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CircuitSettings {
    /// Failure count threshold before the circuit opens.
    pub failure_threshold: u32,
    /// Success count in half-open state before closing.
    pub success_threshold: u32,
    /// Duration (seconds) to wait before transitioning to half-open.
    pub open_duration_secs: u64,
    /// Duration (seconds) of the sliding window for counting failures.
    pub window_size_secs: u64,
}

impl Default for CircuitSettings {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            open_duration_secs: 30,
            window_size_secs: 60,
        }
    }
}

// ---------------------------------------------------------------------------
// Rate limiter
// ---------------------------------------------------------------------------

/// Token-bucket rate-limiter configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RateLimitSettings {
    /// Maximum token bucket capacity.
    pub capacity: u32,
    /// Token refill rate (tokens per second).
    pub refill_rate: u32,
    /// Refill interval in milliseconds.
    pub refill_interval_ms: u64,
}

impl Default for RateLimitSettings {
    fn default() -> Self {
        Self {
            capacity: 100,
            refill_rate: 10,
            refill_interval_ms: 1000,
        }
    }
}

// ---------------------------------------------------------------------------
// Connection pool
// ---------------------------------------------------------------------------

/// Generic connection-pool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PoolSettings {
    /// Maximum number of connections in the pool.
    pub max_size: usize,
    /// Minimum number of idle connections to retain.
    pub min_idle: usize,
    /// Maximum connection lifetime in seconds.
    pub max_lifetime_secs: u64,
    /// Idle timeout in seconds.
    pub idle_timeout_secs: u64,
    /// Connection acquisition timeout in seconds.
    pub connection_timeout_secs: u64,
}

impl Default for PoolSettings {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: 1,
            max_lifetime_secs: 1800,            // 30 minutes
            idle_timeout_secs: 600,              // 10 minutes
            connection_timeout_secs: 30,
        }
    }
}

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

/// File-system path configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PathSettings {
    /// System-wide config directory path template.
    /// `{app_name}` is replaced at runtime.
    pub system_config_path: String,
    /// Environment variable name that can override the config directory.
    pub config_dir_env_var: String,
    /// Default config directory environment variable fallback value.
    pub default_config_dir: String,
}

impl Default for PathSettings {
    fn default() -> Self {
        Self {
            system_config_path: "/etc/{app_name}".into(),
            config_dir_env_var: "CONFIG_DIR".into(),
            default_config_dir: "./config".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Security
// ---------------------------------------------------------------------------

/// Security-alert scoring configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecuritySettings {
    /// Base URL for the GitHub API.
    pub github_api_base_url: String,
    /// Score deduction per critical severity alert.
    pub critical_deduction: f32,
    /// Score deduction per high severity alert.
    pub high_deduction: f32,
    /// Score deduction per medium severity alert.
    pub medium_deduction: f32,
    /// Maximum possible security score.
    pub max_score: f32,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            github_api_base_url: "https://api.github.com".into(),
            critical_deduction: 25.0,
            high_deduction: 10.0,
            medium_deduction: 2.0,
            max_score: 100.0,
        }
    }
}

// ---------------------------------------------------------------------------
// TTL (cache / repository)
// ---------------------------------------------------------------------------

/// Time-to-live constants for cached or replicated data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TtlSettings {
    /// One minute in seconds.
    pub one_minute_secs: u64,
    /// Five minutes in seconds.
    pub five_minutes_secs: u64,
    /// Fifteen minutes in seconds.
    pub fifteen_minutes_secs: u64,
    /// Thirty minutes in seconds.
    pub thirty_minutes_secs: u64,
    /// One hour in seconds.
    pub one_hour_secs: u64,
    /// One day in seconds.
    pub one_day_secs: u64,
    /// One week in seconds.
    pub one_week_secs: u64,
}

impl Default for TtlSettings {
    fn default() -> Self {
        Self {
            one_minute_secs: 60,
            five_minutes_secs: 300,
            fifteen_minutes_secs: 900,
            thirty_minutes_secs: 1800,
            one_hour_secs: 3600,
            one_day_secs: 86400,
            one_week_secs: 604800,
        }
    }
}

// ---------------------------------------------------------------------------
// LoadError
// ---------------------------------------------------------------------------

/// Errors that can occur when loading settings.
#[derive(Debug)]
pub enum LoadError {
    /// Wraps a figment extraction error.
    Figment(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Figment(msg) => write!(f, "settings load error: {msg}"),
        }
    }
}

impl std::error::Error for LoadError {}

// ---------------------------------------------------------------------------
// Helper — produce a Figment provider from the Default impl
// ---------------------------------------------------------------------------

fn serde_figment_defaults() -> impl figment::Provider {
    let defaults = HexaKitSettings::default();
    // Serialize to a JSON Value so figment can merge it.
    let value = serde_json::to_value(&defaults).expect("defaults are always serializable");
    figment::value::Value::from(value)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_defaults() {
        let settings = HexaKitSettings::default();

        // Bulkhead
        assert_eq!(settings.bulkhead.max_concurrent, 10);
        assert_eq!(settings.bulkhead.max_queue, 20);
        assert_eq!(settings.bulkhead.queue_timeout_ms, 1000);

        // Circuit
        assert_eq!(settings.circuit.failure_threshold, 5);
        assert_eq!(settings.circuit.success_threshold, 3);
        assert_eq!(settings.circuit.open_duration_secs, 30);
        assert_eq!(settings.circuit.window_size_secs, 60);

        // Rate limit
        assert_eq!(settings.rate_limit.capacity, 100);
        assert_eq!(settings.rate_limit.refill_rate, 10);
        assert_eq!(settings.rate_limit.refill_interval_ms, 1000);

        // Pool
        assert_eq!(settings.pool.max_size, 10);
        assert_eq!(settings.pool.min_idle, 1);
        assert_eq!(settings.pool.max_lifetime_secs, 1800);
        assert_eq!(settings.pool.idle_timeout_secs, 600);
        assert_eq!(settings.pool.connection_timeout_secs, 30);

        // Paths
        assert_eq!(settings.paths.system_config_path, "/etc/{app_name}");
        assert_eq!(settings.paths.config_dir_env_var, "CONFIG_DIR");
        assert_eq!(settings.paths.default_config_dir, "./config");

        // Security
        assert_eq!(settings.security.github_api_base_url, "https://api.github.com");
        assert_eq!(settings.security.critical_deduction, 25.0);
        assert_eq!(settings.security.high_deduction, 10.0);
        assert_eq!(settings.security.medium_deduction, 2.0);
        assert_eq!(settings.security.max_score, 100.0);

        // TTL
        assert_eq!(settings.ttl.one_minute_secs, 60);
        assert_eq!(settings.ttl.one_hour_secs, 3600);
        assert_eq!(settings.ttl.one_day_secs, 86400);
        assert_eq!(settings.ttl.one_week_secs, 604800);
    }

    #[test]
    fn test_settings_serde_roundtrip() {
        let settings = HexaKitSettings::default();

        // Serialize to TOML and back.
        let toml_str = toml::to_string_pretty(&settings).expect("serialize toml");
        let deserialized: HexaKitSettings =
            toml::from_str(&toml_str).expect("deserialize toml");

        // Spot-check a few fields survive the round-trip.
        assert_eq!(deserialized.bulkhead.max_concurrent, 10);
        assert_eq!(deserialized.circuit.failure_threshold, 5);
        assert_eq!(deserialized.rate_limit.capacity, 100);
        assert_eq!(deserialized.pool.max_size, 10);
        assert_eq!(deserialized.paths.system_config_path, "/etc/{app_name}");
        assert_eq!(deserialized.security.github_api_base_url, "https://api.github.com");
        assert_eq!(deserialized.ttl.one_hour_secs, 3600);
    }

    #[test]
    fn test_settings_toml_deserialize_custom_values() {
        let toml_str = r#"
[bulkhead]
max_concurrent = 25
max_queue = 50
queue_timeout_ms = 2000

[circuit]
failure_threshold = 10
success_threshold = 5
open_duration_secs = 60
window_size_secs = 120

[rate_limit]
capacity = 500
refill_rate = 50
refill_interval_ms = 500

[pool]
max_size = 20
min_idle = 2
max_lifetime_secs = 3600
idle_timeout_secs = 1200
connection_timeout_secs = 15

[paths]
system_config_path = "/custom/etc/{app_name}"
config_dir_env_var = "MY_CONFIG_DIR"
default_config_dir = "/var/app/config"

[security]
github_api_base_url = "https://api.github.mycompany.com"
critical_deduction = 50.0
high_deduction = 20.0
medium_deduction = 5.0
max_score = 100.0

[ttl]
one_minute_secs = 120
five_minutes_secs = 600
fifteen_minutes_secs = 1800
thirty_minutes_secs = 3600
one_hour_secs = 7200
one_day_secs = 172800
one_week_secs = 1209600
"#;

        let settings: HexaKitSettings =
            toml::from_str(toml_str).expect("parse custom toml");

        assert_eq!(settings.bulkhead.max_concurrent, 25);
        assert_eq!(settings.bulkhead.queue_timeout_ms, 2000);
        assert_eq!(settings.circuit.failure_threshold, 10);
        assert_eq!(settings.circuit.open_duration_secs, 60);
        assert_eq!(settings.rate_limit.capacity, 500);
        assert_eq!(settings.rate_limit.refill_interval_ms, 500);
        assert_eq!(settings.pool.max_size, 20);
        assert_eq!(settings.pool.connection_timeout_secs, 15);
        assert_eq!(settings.paths.system_config_path, "/custom/etc/{app_name}");
        assert_eq!(settings.paths.config_dir_env_var, "MY_CONFIG_DIR");
        assert_eq!(settings.security.github_api_base_url, "https://api.github.mycompany.com");
        assert_eq!(settings.security.critical_deduction, 50.0);
        assert_eq!(settings.ttl.one_hour_secs, 7200);
        assert_eq!(settings.ttl.one_week_secs, 1209600);
    }

    #[test]
    fn test_settings_partial_toml_uses_defaults() {
        // Only override bulkhead; everything else should fall back to defaults.
        let toml_str = r#"
[bulkhead]
max_concurrent = 5
"#;

        let settings: HexaKitSettings =
            toml::from_str(toml_str).expect("parse partial toml");

        assert_eq!(settings.bulkhead.max_concurrent, 5);
        assert_eq!(settings.bulkhead.max_queue, 20); // default
        assert_eq!(settings.bulkhead.queue_timeout_ms, 1000); // default

        // Circuit defaults unchanged.
        assert_eq!(settings.circuit.failure_threshold, 5);

        // Rate limit defaults unchanged.
        assert_eq!(settings.rate_limit.capacity, 100);
    }
}
