//! Generic policy evaluation engine for Phenotype.

pub mod context;
pub mod engine;
pub mod error;
pub mod loader;
pub mod policy;
pub mod result;
pub mod rule;

#[cfg(feature = "casbin-backend")]
pub mod casbin_backend;

pub use context::EvaluationContext;
pub use engine::PolicyEngine;
pub use error::PolicyEngineError;
pub use policy::Policy;
pub use result::{PolicyResult, Severity, Violation};
pub use rule::{Rule, RuleType};

#[cfg(feature = "casbin-backend")]
pub use casbin_backend::{
    CasbinBackend, CasbinBackendError, CasbinRequest, CasbinResponse, PolicyBackend,
};

// ---------------------------------------------------------------------------
// Resilience integration (cross-workspace dep on ResilienceKit's
// `phenotype-resilience` crate).
//
// The policy engine is the natural place to wire a circuit breaker around
// policy enforcement: if a backend (Casbin, OPA, etc.) starts failing, we
// want to fail fast instead of piling up requests and stalling callers.
// The full integration is wired in `engine.rs` / `casbin_backend.rs`; the
// snippet below documents the intended use shape and keeps the import live
// so `cargo check` validates the path dep end-to-end.
// ---------------------------------------------------------------------------
#[allow(unused_imports)]
use phenotype_resilience::circuit_breaker::CircuitBreaker;

/// Simple guard example: create a circuit breaker for policy enforcement
/// with a failure threshold of 5 and a 30 s recovery timeout.
///
/// This is illustrative — the real wiring is opt-in per `PolicyEngine`
/// instance. Kept here so the symbol is exported and `cargo check`
/// validates the cross-workspace `phenotype-resilience` dep.
#[allow(dead_code)]
pub fn make_policy_enforcement_guard() -> CircuitBreaker {
    use std::time::Duration;
    CircuitBreaker::new(5, Duration::from_secs(30))
}

pub mod prelude {
    pub use crate::{
        context::EvaluationContext,
        engine::PolicyEngine,
        error::PolicyEngineError,
        policy::Policy,
        result::{PolicyResult, Severity, Violation},
        rule::{Rule, RuleType},
    };
}
