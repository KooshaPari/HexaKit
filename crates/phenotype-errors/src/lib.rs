//! # Phenotype Errors
//!
//! Unified error types for the Phenotype ecosystem.

pub use phenotype_error_core::{ApiError, ConfigError, DomainError, RepositoryError, StorageError};

/// Canonical error type alias.
pub type Error = ApiError;

/// Convenience result type.
pub type Result<T> = std::result::Result<T, ApiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error() {
        let err = ApiError::NotFound {
            resource: "user".into(),
            id: "42".into(),
        };
        assert_eq!(err.status_code(), 404);
    }

    #[test]
    fn test_result_type_ok() {
        let r: Result<i32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn test_domain_error() {
        let err = DomainError::Validation("invalid".into());
        assert!(err.to_string().contains("validation failed"));
    }

    #[test]
    fn test_error_from_serde_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("{not valid").unwrap_err();
        let original = json_err.to_string();
        let err: Error = json_err.into();
        assert_eq!(err.status_code(), 400);
        assert!(err.to_string().contains("bad request"));
        assert!(err.to_string().contains(&original));
    }
}
