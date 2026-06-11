//! OpenTelemetry integration.
//!
//! ## Status: GAP DOCUMENTED (plan §10 T3 — blocked on source crate)
//!
//! This module is the planned re-home for the OTel init shape that
//! currently lives in
//! `HexaKit/crates/phenotype-logging/src/otel.rs:1-89` (the only
//! known-working OTLP/gRPC bootstrap in the fleet). The migration is
//! described in
//! `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md` §1
//! (TL;DR) and §3 (init shape to port).
//!
//! ### The gap (machine-checked by `gap_is_documented` below)
//!
//! `phenotype-logging` does **not** publicly export the OTel init
//! symbols. `phenotype-logging/src/lib.rs:1-11` declares only a
//! trivial `Error` enum and a `Result<T>` alias — there is no
//! `pub mod otel;` and no `pub use otel::*;`.
//!
//! The canonical re-export shape:
//!
//! ```ignore
//! pub use phenotype_logging::otel::init_with_otel;
//! ```
//!
//! therefore does not compile today. The `init_with_otel`,
//! `init_with_resource`, and `OTelError` items in
//! `phenotype-logging/src/otel.rs` are private to that crate.
//!
//! ### Resolution path (per plan §10 T3, then §10 T6)
//!
//! 1. Lift the OTel init shape into this crate's `init.rs` /
//!    `guard.rs` / `error.rs` / `config.rs` modules (plan §10 T3).
//!    The init is traces-only today; `TelemetryGuard`, sampler
//!    logic mirroring Tracera's `tracer.go:81-86`, and an explicit
//!    propagator are added at the same time (plan §3.3).
//! 2. Switch consumers to `phenotype_observability::init_with_otel`.
//! 3. Delete `phenotype-logging/src/otel.rs` and drop OTel deps
//!    from `phenotype-logging/Cargo.toml` (plan §10 T6).
//!
//! Until then, this module exposes the function signatures as
//! **honest stubs** that return `OTelError::NotWired`. The
//! signatures are stable; the bodies will be filled in when step 1
//! above lands. No `pub use phenotype_logging::...` is performed —
//! doing so would be a fake import per the project's standing
//! convention (see the prior module-level comment in
//! `phenotype-observability/src/lib.rs`).
//!
//! ## Migration evidence
//!
//! The OTel init function bodies we will eventually lift (currently
//! in `phenotype-logging/src/otel.rs:7-36` for `init_with_otel` and
//! lines 39-76 for `init_with_resource`) are:
//!
//! - `init_with_otel(service_name, otlp_endpoint) -> Result<(), OTelError>`
//! - `init_with_resource(service_name, otlp_endpoint, attributes) -> Result<(), OTelError>`
//!
//! Both use the same `opentelemetry_otlp::SpanExporter::builder()`
//! + `tracing_opentelemetry::layer().with_tracer(tracer)` +
//! `tracing_subscriber::registry().init()` shape. The signatures
//! here are byte-for-byte equivalent to those in
//! `phenotype-logging/src/otel.rs:7` and `:39` so that consumers
//! adopting this crate can later swap implementations without
//! touching call sites.

use thiserror::Error;

/// OpenTelemetry init errors.
///
/// The `NotWired` variant is the only one reachable in the current
/// stub state. The other three variants are pre-declared to match
/// the variants in `phenotype-logging/src/otel.rs:79-89` so that the
/// eventual lift (plan §10 T3) does not need to change this enum's
/// public surface.
#[derive(Debug, Error)]
pub enum OTelError {
    /// Stub-only variant. Returned by the placeholder
    /// `init_with_otel` / `init_with_resource` bodies below. Will
    /// be removed (or downgraded to a `#[deprecated]` alias) when
    /// the real OTel init is lifted into this crate per plan §10 T3.
    #[error("OTel init not yet wired in phenotype-observability: {0}")]
    NotWired(String),

    #[error("export error: {0}")]
    ExportError(String),

    #[error("configuration error: {0}")]
    ConfigError(String),

    #[error("runtime error: {0}")]
    RuntimeError(String),
}

/// Machine-checkable flag: `true` once `phenotype-logging` publicly
/// exports the OTel init symbols. Currently `false` because
/// `phenotype-logging/src/lib.rs` does not declare `pub mod otel;`.
///
/// When this becomes `true`, the stub bodies below can be replaced
/// with the real re-export
/// (`pub use phenotype_logging::otel::{init_with_otel, init_with_resource, OTelError};`)
/// or — more likely per plan §10 T3 — the init shape is lifted
/// directly into this crate and `phenotype-logging/src/otel.rs` is
/// deleted entirely.
pub const PHENOTYPE_LOGGING_EXPORTS_OTEL_INIT: bool = false;

/// Initialize tracing with OpenTelemetry OTLP exporter.
///
/// **Stub:** the signature is byte-for-byte equivalent to
/// `phenotype-logging/src/otel.rs:7`, but the body returns
/// `OTelError::NotWired` until the gap documented in this module's
/// top-level doc is resolved. See plan §10 T3 / §10 T6.
///
/// Does **not** actually init OTel. The `cargo check` and `cargo
/// test` runs for this crate will not spawn a Tokio runtime or open
/// any network connection.
pub fn init_with_otel(service_name: &str, otlp_endpoint: &str) -> Result<(), OTelError> {
    Err(OTelError::NotWired(format!(
        "phenotype-observability::otel::init_with_otel is a stub: \
         phenotype-logging/src/lib.rs does not declare `pub mod otel;`. \
         See plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md §3.4. \
         Requested init: service_name={service_name:?}, otlp_endpoint={otlp_endpoint:?}"
    )))
}

/// Initialize OpenTelemetry with a custom resource.
///
/// **Stub:** placeholder body. See [`init_with_otel`] for the gap
/// analysis. Signature matches
/// `phenotype-logging/src/otel.rs:39`.
pub fn init_with_resource(
    service_name: &str,
    otlp_endpoint: &str,
    attributes: Vec<(&str, &str)>,
) -> Result<(), OTelError> {
    Err(OTelError::NotWired(format!(
        "phenotype-observability::otel::init_with_resource is a stub: \
         phenotype-logging/src/lib.rs does not declare `pub mod otel;`. \
         See plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md §3.4. \
         Requested init: service_name={service_name:?}, otlp_endpoint={otlp_endpoint:?}, \
         attribute_count={}",
        attributes.len()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify the function signatures exist and have the expected
    /// shape, without actually initialising OTel. The bodies are
    /// stubs that return `OTelError::NotWired`, so this test does
    /// not spawn a Tokio runtime or open any network connection.
    #[test]
    fn init_with_otel_signature_exists() {
        let f: fn(&str, &str) -> Result<(), OTelError> = init_with_otel;
        let result = f("test-service", "http://127.0.0.1:4317");
        assert!(
            matches!(result, Err(OTelError::NotWired(_))),
            "expected OTelError::NotWired from stub, got {result:?}"
        );
    }

    /// Verify the `init_with_resource` signature exists, including
    /// the `Vec<(&str, &str)>` resource-attributes parameter. As
    /// above, no OTel is initialised.
    #[test]
    fn init_with_resource_signature_exists() {
        let f: fn(&str, &str, Vec<(&str, &str)>) -> Result<(), OTelError> = init_with_resource;
        let result = f(
            "test-service",
            "http://127.0.0.1:4317",
            vec![("environment", "test"), ("region", "local")],
        );
        assert!(
            matches!(result, Err(OTelError::NotWired(_))),
            "expected OTelError::NotWired from stub, got {result:?}"
        );
    }

    /// Machine-checkable assertion of the gap. If this test ever
    /// fails, `phenotype-logging` has started publicly exporting
    /// the OTel init symbols — at which point the stub bodies in
    /// this module can be replaced with the real re-export (or the
    /// init shape can be lifted per plan §10 T3).
    #[test]
    fn gap_is_documented() {
        assert!(
            !PHENOTYPE_LOGGING_EXPORTS_OTEL_INIT,
            "phenotype-logging now publicly exports OTel init symbols — \
             update otel.rs to perform the re-export (or proceed with \
             plan §10 T3 lift)."
        );
    }
}
