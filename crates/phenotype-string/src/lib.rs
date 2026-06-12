//! phenotype-string

use thiserror::Error;

pub mod compression;
pub mod join;
pub mod normalization;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    Invalid(String),
    #[error("Compression error: {0}")]
    Compression(String),
    #[error("Decompression error: {0}")]
    Decompression(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_enum_implements_clone() {
        let original = Error::Invalid("bad input".to_string());
        let cloned = original.clone();
        assert_eq!(format!("{original}"), format!("{cloned}"));
    }
}
