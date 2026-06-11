//! `phenotype-observability`
//!
//! Canonical OpenTelemetry observability primitives for the Phenotype
//! ecosystem.
//!
//! ## Status: scaffold (plan §10 T1) + `otel` module placeholder
//!
//! The OTel init shape is the planned re-home for the working
//! OTLP/gRPC bootstrap that currently lives in
//! `HexaKit/crates/phenotype-logging/src/otel.rs:1-89` (the only
//! known-working bootstrap in the fleet). See
//! `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md` §1
//! (TL;DR), §3 (init shape to port), §4 (proposed crate contents).
//!
//! ## Known gap (machine-checked in `otel::gap_is_documented`)
//!
//! `phenotype-logging` does **not** publicly export the OTel init
//! symbols — `phenotype-logging/src/lib.rs:1-11` only declares a
//! trivial `Error` enum and a `Result<T>` alias. The canonical
//! re-export
//!
//! ```ignore
//! pub use phenotype_logging::otel::init_with_otel;
//! ```
//!
//! does not compile today. Per the project's standing convention,
//! the `otel` module therefore exposes the function signatures as
//! **honest stubs** that return `OTelError::NotWired` rather than
//! faking a `pub use` that would not resolve. The full gap analysis
//! lives in `crate::otel`'s module-level documentation.
//!
//! Resolution path: plan §10 T3 (lift the init shape into this
//! crate's `init.rs` / `guard.rs` / etc.) and §10 T6 (delete
//! `phenotype-logging/src/otel.rs` and drop the OTel deps from
//! `phenotype-logging/Cargo.toml`).
//!
//! ## Symbol map
//!
//! - [`version`] — trivial stand-in symbol that lets the scaffold
//!   crate compile and have a public surface for tests. Will be
//!   retained as a re-export of `CARGO_PKG_VERSION` even after the
//!   OTel init lands.
//! - [`otel`] module — see the gap analysis there. Re-exports
//!   [`otel::OTelError`], [`otel::init_with_otel`],
//!   [`otel::init_with_resource`], and the
//!   [`otel::PHENOTYPE_LOGGING_EXPORTS_OTEL_INIT`] gap flag at the
//!   crate root via `pub use otel::*;` below.

/// Crate version, sourced from `Cargo.toml` at build time.
///
/// Trivial stand-in symbol so the scaffold crate has a public
/// surface that compiles and tests. Will be retained as a re-export
/// of `CARGO_PKG_VERSION` after the OTel init lands.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// OTel init shape module — see module-level documentation for the
/// gap analysis. All four public items are re-exported at the crate
/// root.
pub mod otel;

pub use otel::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }

    /// Smoke test: the OTel module compiles and its public items
    /// are reachable from the crate root via the `pub use otel::*;`
    /// re-export above.
    #[test]
    fn otel_module_re_exports_are_reachable_from_crate_root() {
        // `pub use otel::*;` must re-export the gap flag.
        assert!(!PHENOTYPE_LOGGING_EXPORTS_OTEL_INIT);
        // `OTelError` is re-exported with the `NotWired` variant.
        let err = OTelError::NotWired("smoke test".to_string());
        assert!(err.to_string().contains("smoke test"));
        // The init stubs are reachable and have the expected return type.
        let r: Result<(), OTelError> =
            init_with_otel("svc", "http://127.0.0.1:4317");
        assert!(r.is_err());
    }
}
