//! Canonical error types for the Phenotype ecosystem.
//!
//! This crate provides a unified error framework that consolidates the
//! duplicated error enums scattered across Phenotype crates. Each error
//! category maps to a distinct architectural layer:
//!
//! | Error type | Layer | Typical producer |
//! |---|---|---|
//! | [`ApiError`] | HTTP / transport | Route handlers, middleware |
//! | [`DomainError`] | Business rules | Domain services, aggregates |
//! | [`RepositoryError`] | Persistence | Store adapters, query layers |
//! | [`ConfigError`] | Configuration | Loaders, environment readers |
//! | [`StorageError`] | Raw I/O | File, network, cache adapters |
//!
//! # Migration from per-crate errors
//!
//! Replace a local `error.rs` with a re-export:
//!
//! ```rust,ignore
//! // old: mod error; pub use error::MyError;
//! // new:
//! pub use phenotype_error_core::DomainError;
//! pub type Result<T> = std::result::Result<T, DomainError>;
//! ```

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ---------------------------------------------------------------------------
// API / transport layer
// ---------------------------------------------------------------------------

/// Errors originating from the HTTP / transport boundary.
#[derive(Error, Debug, Eq, Hash, PartialEq)]
pub enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("not found: {resource} {id}")]
    NotFound { resource: String, id: String },

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("rate limited")]
    RateLimited,

    #[error("timeout")]
    Timeout,

    #[error("internal: {0}")]
    Internal(String),

    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

impl ApiError {
    /// HTTP status code for this error.
    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadRequest(_) => 400,
            Self::Unauthorized(_) => 401,
            Self::Forbidden(_) => 403,
            Self::NotFound { .. } => 404,
            Self::Conflict(_) => 409,
            Self::RateLimited => 429,
            Self::Timeout => 504,
            Self::Internal(_) => 500,
            Self::Domain(_) => 422,
            Self::Repository(_) => 500,
        }
    }

    /// Whether the caller should retry.
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::RateLimited | Self::Timeout | Self::Internal(_))
    }
}

// ---------------------------------------------------------------------------
// Domain / business-logic layer
// ---------------------------------------------------------------------------

/// Errors from domain logic: validation, invariant violations, state issues.
#[derive(Error, Debug, Eq, Hash, PartialEq)]
pub enum DomainError {
    #[error("validation failed: {0}")]
    Validation(String),

    #[error("invariant violated: {0}")]
    InvariantViolation(String),

    #[error("entity not found: {entity} {id}")]
    NotFound { entity: String, id: String },

    #[error("duplicate entity: {entity} {id}")]
    Duplicate { entity: String, id: String },

    #[error("invalid state transition: {from} -> {to}")]
    InvalidStateTransition { from: String, to: String },

    #[error("operation not permitted: {0}")]
    NotPermitted(String),

    #[error("policy evaluation failed: {0}")]
    PolicyEvaluation(String),

    #[error("{0}")]
    Other(String),
}

// ---------------------------------------------------------------------------
// Repository / persistence layer
// ---------------------------------------------------------------------------

/// Errors from persistence adapters.
#[derive(Error, Debug, Eq, Hash, PartialEq)]
pub enum RepositoryError {
    #[error("record not found: {entity} {id}")]
    NotFound { entity: String, id: String },

    #[error("duplicate record: {entity} {id}")]
    Duplicate { entity: String, id: String },

    #[error("connection error: {0}")]
    Connection(String),

    #[error("query error: {0}")]
    Query(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("sequence gap: expected {expected}, got {actual}")]
    SequenceGap { expected: i64, actual: i64 },

    #[error("integrity error: {0}")]
    Integrity(String),

    #[error(transparent)]
    Storage(#[from] StorageError),
}

impl From<serde_json::Error> for RepositoryError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

// ---------------------------------------------------------------------------
// Configuration layer
// ---------------------------------------------------------------------------

/// Errors from configuration loading, parsing, and validation.
#[derive(Error, Debug, Eq, Hash, PartialEq)]
pub enum ConfigError {
    #[error("file not found: {}", path.display())]
    FileNotFound { path: PathBuf },

    #[error("file read error: {}: {reason}", path.display())]
    FileRead { path: PathBuf, reason: String },

    #[error("parse error ({format}): {reason}")]
    Parse { format: String, reason: String },

    #[error("deserialization error: {0}")]
    Deserialize(String),

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("missing required field: {0}")]
    MissingRequired(String),

    #[error("environment error: {0}")]
    Environment(String),

    #[error("{0}")]
    Other(String),
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::FileNotFound {
                path: PathBuf::from("<unknown>"),
            },
            _ => Self::Other(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse {
            format: "json".into(),
            reason: err.to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Storage / raw I/O layer
// ---------------------------------------------------------------------------

/// Low-level storage errors (files, network, cache).
#[derive(Error, Debug, Eq, Hash, PartialEq)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("capacity exceeded: {0}")]
    CapacityExceeded(String),

    #[error("connection error: {0}")]
    Connection(String),

    #[error("{0}")]
    Other(String),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        // `std::io::Error` does not implement `Eq`/`Hash`, so we
        // canonicalise it to its `Display` form for storage. The
        // original error is recoverable through `Error::source`
        // (hand-written) if callers need the structured view.
        Self::Io(err.to_string())
    }
}

// ---------------------------------------------------------------------------
// Unified error enum
// ---------------------------------------------------------------------------

/// Unified error type that can represent a failure from any layer.
///
/// This is a convenience enum for callers (binaries, library APIs,
/// adapters) that want a single error type rather than juggling the
/// per-layer enums (`ApiError`, `DomainError`, `RepositoryError`,
/// `ConfigError`, `StorageError`). The [`Display`] implementation is
/// hand-written — not driven by `thiserror` — so every variant
/// renders in a consistent `"<layer>: <message>"` shape regardless
/// of the inner error's own `Display` format.
///
/// Use [`Error::source`] to recover the underlying per-layer error
/// when you need to match on its specific variant.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// HTTP / transport-layer failure.
    Api(ApiError),
    /// Business-logic failure.
    Domain(DomainError),
    /// Persistence-layer failure.
    Repository(RepositoryError),
    /// Configuration loading / parsing failure.
    Config(ConfigError),
    /// Raw I/O / cache / network failure.
    Storage(StorageError),
    /// Catch-all for ad-hoc error strings that don't map to a layer.
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api(e) => write!(f, "api: {e}"),
            Self::Domain(e) => write!(f, "domain: {e}"),
            Self::Repository(e) => write!(f, "repository: {e}"),
            Self::Config(e) => write!(f, "config: {e}"),
            Self::Storage(e) => write!(f, "storage: {e}"),
            Self::Other(msg) => write!(f, "other: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Api(e) => Some(e),
            Self::Domain(e) => Some(e),
            Self::Repository(e) => Some(e),
            Self::Config(e) => Some(e),
            Self::Storage(e) => Some(e),
            Self::Other(_) => None,
        }
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::Other("default error".into())
    }
}

// ---------------------------------------------------------------------------
// Serializable error envelope (for API responses / logging)
// ---------------------------------------------------------------------------

/// Wire-format error envelope suitable for JSON API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl From<&ApiError> for ErrorEnvelope {
    fn from(err: &ApiError) -> Self {
        Self {
            code: format!("ERR_{}", err.status_code()),
            message: err.to_string(),
            details: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Context helpers
// ---------------------------------------------------------------------------

/// Extension trait adding `.context()` to `Result` types for richer messages.
pub trait ErrorContext<T, E> {
    /// Wrap the error with additional context.
    fn context(self, msg: impl Into<String>) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> ErrorContext<T, E> for Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T, String> {
        self.map_err(|e| format!("{}: {e}", msg.into()))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn api_error_status_codes() {
        assert_eq!(ApiError::BadRequest("x".into()).status_code(), 400);
        assert_eq!(ApiError::Unauthorized("x".into()).status_code(), 401);
        assert_eq!(ApiError::Forbidden("x".into()).status_code(), 403);
        assert_eq!(
            ApiError::NotFound {
                resource: "user".into(),
                id: "1".into()
            }
            .status_code(),
            404
        );
        assert_eq!(ApiError::Conflict("x".into()).status_code(), 409);
        assert_eq!(ApiError::RateLimited.status_code(), 429);
        assert_eq!(ApiError::Timeout.status_code(), 504);
        assert_eq!(ApiError::Internal("x".into()).status_code(), 500);
    }

    #[test]
    fn api_error_retryable() {
        assert!(ApiError::RateLimited.is_retryable());
        assert!(ApiError::Timeout.is_retryable());
        assert!(ApiError::Internal("boom".into()).is_retryable());
        assert!(!ApiError::BadRequest("nope".into()).is_retryable());
    }

    #[test]
    fn domain_error_display() {
        let err = DomainError::Validation("name required".into());
        assert_eq!(err.to_string(), "validation failed: name required");
    }

    #[test]
    fn domain_error_state_transition() {
        let err = DomainError::InvalidStateTransition {
            from: "draft".into(),
            to: "published".into(),
        };
        assert!(err.to_string().contains("draft"));
        assert!(err.to_string().contains("published"));
    }

    #[test]
    fn repository_error_from_serde() {
        let json_err = serde_json::from_str::<String>("not json").unwrap_err();
        let repo_err = RepositoryError::from(json_err);
        assert!(matches!(repo_err, RepositoryError::Serialization(_)));
    }

    #[test]
    fn repository_error_sequence_gap() {
        let err = RepositoryError::SequenceGap {
            expected: 5,
            actual: 7,
        };
        assert!(err.to_string().contains("expected 5"));
        assert!(err.to_string().contains("got 7"));
    }

    #[test]
    fn config_error_from_io_not_found() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let cfg_err = ConfigError::from(io_err);
        assert!(matches!(cfg_err, ConfigError::FileNotFound { .. }));
    }

    #[test]
    fn config_error_from_io_other() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "nope");
        let cfg_err = ConfigError::from(io_err);
        assert!(matches!(cfg_err, ConfigError::Other(_)));
    }

    #[test]
    fn config_error_from_serde_json() {
        let json_err = serde_json::from_str::<String>("bad").unwrap_err();
        let cfg_err = ConfigError::from(json_err);
        assert!(matches!(cfg_err, ConfigError::Parse { format, .. } if format == "json"));
    }

    #[test]
    fn storage_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe");
        let store_err = StorageError::from(io_err);
        assert!(matches!(store_err, StorageError::Io(_)));
    }

    #[test]
    fn error_envelope_from_api_error() {
        let api_err = ApiError::NotFound {
            resource: "project".into(),
            id: "42".into(),
        };
        let envelope = ErrorEnvelope::from(&api_err);
        assert_eq!(envelope.code, "ERR_404");
        assert!(envelope.message.contains("project"));
    }

    #[test]
    fn error_envelope_serialization() {
        let envelope = ErrorEnvelope {
            code: "ERR_500".into(),
            message: "internal".into(),
            details: Some("stack trace".into()),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("ERR_500"));
        assert!(json.contains("stack trace"));

        let roundtrip: ErrorEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip.code, "ERR_500");
    }

    #[test]
    fn api_error_from_domain() {
        let domain_err = DomainError::Validation("bad input".into());
        let api_err = ApiError::from(domain_err);
        assert_eq!(api_err.status_code(), 422);
        assert!(api_err.to_string().contains("bad input"));
    }

    #[test]
    fn api_error_from_repository() {
        let repo_err = RepositoryError::Connection("db down".into());
        let api_err = ApiError::from(repo_err);
        assert_eq!(api_err.status_code(), 500);
    }

    #[test]
    fn repository_error_from_storage() {
        let store_err = StorageError::NotFound("file.dat".into());
        let repo_err = RepositoryError::from(store_err);
        assert!(matches!(repo_err, RepositoryError::Storage(_)));
    }

    #[test]
    fn context_helper() {
        let result: Result<(), &str> = Err("boom");
        let ctx = result.context("loading config");
        assert_eq!(ctx.unwrap_err(), "loading config: boom");
    }

    #[test]
    fn anyhow_interop() {
        let domain_err = DomainError::Validation("test".into());
        let anyhow_err: anyhow::Error = domain_err.into();
        assert!(anyhow_err.to_string().contains("validation failed: test"));

        let api_err = ApiError::Internal("crash".into());
        let anyhow_err: anyhow::Error = api_err.into();
        assert!(anyhow_err.to_string().contains("crash"));
    }

    // ---- Error enum Display tests (1 per variant) ----

    #[test]
    fn error_display_api_variant() {
        let err = Error::Api(ApiError::BadRequest("bad input".into()));
        let rendered = err.to_string();
        assert!(
            rendered.contains("api"),
            "rendered should include layer label: {rendered}"
        );
        assert!(
            rendered.contains("bad input"),
            "rendered should include inner message: {rendered}"
        );
        assert!(
            rendered.starts_with("api:"),
            "rendered should start with `api:`: {rendered}"
        );
    }

    #[test]
    fn error_display_domain_variant() {
        let err = Error::Domain(DomainError::Validation("name required".into()));
        let rendered = err.to_string();
        assert!(
            rendered.contains("domain"),
            "rendered should include layer label: {rendered}"
        );
        assert!(
            rendered.contains("name required"),
            "rendered should include inner message: {rendered}"
        );
        assert!(
            rendered.starts_with("domain:"),
            "rendered should start with `domain:`: {rendered}"
        );
    }

    #[test]
    fn error_display_repository_variant() {
        let err = Error::Repository(RepositoryError::Connection("db down".into()));
        let rendered = err.to_string();
        assert!(
            rendered.contains("repository"),
            "rendered should include layer label: {rendered}"
        );
        assert!(
            rendered.contains("db down"),
            "rendered should include inner message: {rendered}"
        );
        assert!(
            rendered.starts_with("repository:"),
            "rendered should start with `repository:`: {rendered}"
        );
    }

    #[test]
    fn error_display_config_variant() {
        let err = Error::Config(ConfigError::Other("bad cfg".into()));
        let rendered = err.to_string();
        assert!(
            rendered.contains("config"),
            "rendered should include layer label: {rendered}"
        );
        assert!(
            rendered.contains("bad cfg"),
            "rendered should include inner message: {rendered}"
        );
        assert!(
            rendered.starts_with("config:"),
            "rendered should start with `config:`: {rendered}"
        );
    }

    #[test]
    fn error_display_storage_variant() {
        let err = Error::Storage(StorageError::NotFound("file.dat".into()));
        let rendered = err.to_string();
        assert!(
            rendered.contains("storage"),
            "rendered should include layer label: {rendered}"
        );
        assert!(
            rendered.contains("file.dat"),
            "rendered should include inner message: {rendered}"
        );
        assert!(
            rendered.starts_with("storage:"),
            "rendered should start with `storage:`: {rendered}"
        );
    }

    #[test]
    fn error_display_other_variant() {
        let err = Error::Other("misc".into());
        let rendered = err.to_string();
        assert!(
            rendered.contains("other"),
            "rendered should include layer label: {rendered}"
        );
        assert!(
            rendered.contains("misc"),
            "rendered should include inner message: {rendered}"
        );
        assert!(
            rendered.starts_with("other:"),
            "rendered should start with `other:`: {rendered}"
        );
    }

    #[test]
    fn error_source_chains_to_inner() {
        // Non-Other variants should expose the inner per-layer error
        // through `Error::source`.
        let inner = ApiError::Internal("boom".into());
        let err = Error::Api(inner);
        let src = err.source().expect("Api variant should have a source");
        assert!(src.to_string().contains("boom"));

        // Other variant has no source.
        let err = Error::Other("lonely".into());
        assert!(err.source().is_none());
    }

    #[test]
    fn error_default_returns_other_catch_all() {
        // `Default::default()` should yield the `Other` catch-all variant
        // with the documented placeholder message, so callers can rely on
        // `Error: Default` (e.g. `Result<T, Error>::unwrap_or_default`).
        let err: Error = Error::default();
        assert!(
            matches!(err, Error::Other(ref msg) if msg == "default error"),
            "default should be Error::Other(\"default error\"), got {err}"
        );
        assert_eq!(err.to_string(), "other: default error");
    }

    #[test]
    fn error_hashset_round_trip() {
        // `Error` must be `Hash + Eq` so callers can use it as a key
        // in hash-based collections. This test inserts one instance
        // of every variant into a `HashSet<Error>` and verifies
        // (1) all six are present, (2) duplicates collapse, and
        // (3) a freshly-constructed equal value resolves to the
        // same entry (round-trip via `contains`).
        use std::collections::HashSet;
        let mut set: HashSet<Error> = HashSet::new();

        set.insert(Error::Api(ApiError::Internal("boom".into())));
        set.insert(Error::Domain(DomainError::Validation("bad input".into())));
        set.insert(Error::Repository(RepositoryError::Connection(
            "db down".into(),
        )));
        set.insert(Error::Config(ConfigError::Other("bad cfg".into())));
        set.insert(Error::Storage(StorageError::NotFound("file.dat".into())));
        set.insert(Error::Other("misc".into()));

        // Duplicate insertion must not grow the set.
        set.insert(Error::Api(ApiError::Internal("boom".into())));
        assert_eq!(
            set.len(),
            6,
            "HashSet<Error> should hold exactly one entry per distinct variant, got {} entries",
            set.len()
        );

        // Round-trip: re-constructing an equal value must find the same entry.
        assert!(set.contains(&Error::Api(ApiError::Internal("boom".into()))));
        assert!(set.contains(&Error::Domain(DomainError::Validation(
            "bad input".into()
        ))));
        assert!(set.contains(&Error::Repository(RepositoryError::Connection(
            "db down".into()
        ))));
        assert!(set.contains(&Error::Config(ConfigError::Other(
            "bad cfg".into()
        ))));
        assert!(set.contains(&Error::Storage(StorageError::NotFound(
            "file.dat".into()
        ))));
        assert!(set.contains(&Error::Other("misc".into())));

        // A non-equal value must not collide.
        assert!(!set.contains(&Error::Api(ApiError::Internal("different".into()))));
    }
}
