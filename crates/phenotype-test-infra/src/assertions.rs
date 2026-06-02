//! Assertion Helpers
//!
//! Provides convenient assertion macros and functions for tests.

/// Result of an assertion
pub type AssertionResult = Result<(), AssertionError>;

/// Error when an assertion fails
#[derive(Debug, Clone, thiserror::Error)]
#[error("Assertion failed: {message}")]
pub struct AssertionError {
    pub message: String,
}

/// Assertion helpers trait
pub trait Assertion {
    type Item;
    
    /// Assert that the value is equal to expected
    fn assert_eq(&self, expected: &Self::Item, msg: &str) -> AssertionResult;
    
    /// Assert that the value is not equal to expected
    fn assert_ne(&self, unexpected: &Self::Item, msg: &str) -> AssertionResult;
}

impl<T: PartialEq + std::fmt::Debug> Assertion for T {
    type Item = T;
    
    fn assert_eq(&self, expected: &T, msg: &str) -> AssertionResult {
        if self != expected {
            return Err(AssertionError {
                message: format!("{msg}: expected {expected:?}, got {self:?}"),
            });
        }
        Ok(())
    }
    
    fn assert_ne(&self, unexpected: &T, msg: &str) -> AssertionResult {
        if self == unexpected {
            return Err(AssertionError {
                message: format!("{msg}: expected not equal to {unexpected:?}"),
            });
        }
        Ok(())
    }
}

/// Assert that an option is Some
pub fn assert_some<T>(opt: &Option<T>, msg: &str) -> AssertionResult {
    if opt.is_none() {
        return Err(AssertionError {
            message: format!("{msg}: expected Some, got None"),
        });
    }
    Ok(())
}

/// Assert that an option is None
pub fn assert_none<T>(opt: &Option<T>, msg: &str) -> AssertionResult {
    if opt.is_some() {
        return Err(AssertionError {
            message: format!("{msg}: expected None, got Some"),
        });
    }
    Ok(())
}

/// Assert that a result is Ok
pub fn assert_ok<T, E>(res: &Result<T, E>, msg: &str) -> AssertionResult {
    if res.is_err() {
        return Err(AssertionError {
            message: format!("{msg}: expected Ok, got Err"),
        });
    }
    Ok(())
}

/// Assert that a result is Err
pub fn assert_err<T, E>(res: &Result<T, E>, msg: &str) -> AssertionResult {
    if res.is_ok() {
        return Err(AssertionError {
            message: format!("{msg}: expected Err, got Ok"),
        });
    }
    Ok(())
}

/// Assert that a vector contains an element
pub fn assert_contains<T: PartialEq + std::fmt::Debug>(vec: &[T], item: &T, msg: &str) -> AssertionResult {
    if !vec.contains(item) {
        return Err(AssertionError {
            message: format!("{msg}: expected vector to contain {item:?}"),
        });
    }
    Ok(())
}

/// Assert that a string matches a pattern
pub fn assert_matches(haystack: &str, needle: &str, msg: &str) -> AssertionResult {
    if !haystack.contains(needle) {
        return Err(AssertionError {
            message: format!(
                "{msg}: expected '{haystack}' to contain '{needle}'"
            ),
        });
    }
    Ok(())
}
