//! HMAC signature utilities (stub).
//!
//! Minimal placeholder API. Real implementations live in
//! `crates/cipher/src/core/signatures.rs` and `hashing.rs`. These stubs
//! exist to keep the top-level `hexakit` crate compiling while the canonical
//! migration is in progress.

/// Compute an HMAC-SHA256 signature (stub: returns empty Vec).
pub fn compute_hmac(_key: &[u8], _data: &[u8]) -> Vec<u8> {
    Vec::new()
}

/// Compute an HMAC-SHA256 signature, hex-encoded (stub: returns empty String).
pub fn compute_hmac_hex(_key: &[u8], _data: &[u8]) -> String {
    String::new()
}

/// Verify an HMAC-SHA256 signature (stub: always returns false).
pub fn verify_hmac(_key: &[u8], _data: &[u8], _expected: &[u8]) -> bool {
    false
}

/// Verify a hex-encoded HMAC-SHA256 signature (stub: always returns false).
pub fn verify_hmac_hex(_key: &[u8], _data: &[u8], _expected_hex: &str) -> bool {
    false
}
