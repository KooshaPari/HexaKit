//! Rate limiting placeholder.
//!
//! TODO(H14.x): implement per-account token-bucket once routing-plane quota
//! semantics are finalised. Stub keeps `lib.rs`'s `pub mod rate_limit;`
//! resolvable.

#[derive(Debug, Clone, Copy, Default)]
pub struct RateLimiter {
    _private: (),
}

impl RateLimiter {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs() {
        let _ = RateLimiter::new();
    }
}
