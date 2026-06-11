//! `phenotype-observability`
//!
//! Canonical OpenTelemetry observability primitives for the Phenotype
//! ecosystem.
//!
//! ## Status: scaffold (plan §10 T1)
//!
//! This crate is the planned re-home for the OTel init shape that
//! currently lives in
//! `HexaKit/crates/phenotype-logging/src/otel.rs` (the only
//! known-working OTLP/gRPC bootstrap in the fleet). See
//! `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md` §1
//! (TL;DR), §3 (init shape to port), §4 (proposed crate contents).
//!
//! ## Known gap (must be resolved before the re-export lands)
//!
//! **The current `phenotype-logging` crate does not publicly export
//! any OTel init symbol.** `phenotype-logging/src/lib.rs` declares
//! neither `pub mod otel;` nor any `pub use otel::*;`, so the
//! `init_with_otel` / `init_with_resource` / `OTelError` items in
//! `phenotype-logging/src/otel.rs` are unreachable from outside the
//! crate. They are not even compiled into the `phenotype-logging`
//! crate as it stands today.
//!
//! The intended re-export
//!
//! ```ignore
//! pub use phenotype_logging::init_with_otel;
//! ```
//!
//! therefore does not exist yet. This file intentionally does NOT
//! fake the import. The two resolutions are tracked in
//! `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md`:
//!
//! - **Option A (plan §10 T3, then §10 T6):** when this crate is
//!   ready to host the OTel init, lift `phenotype-logging/src/otel.rs`
//!   into `phenotype-observability/src/init.rs` (etc.), then delete
//!   OTel from `phenotype-logging`. This crate will expose its own
//!   `init_otel` symbol — no re-export from `phenotype-logging` is
//!   needed at any point.
//!
//! - **Option B (less clean):** first publish the OTel init symbols
//!   from `phenotype-logging` (by adding `pub mod otel;` to its
//!   `lib.rs`), re-export them from this crate, then proceed with
//!   the T6 deletion. This option exists only if downstream
//!   consumers must adopt the new crate name before the OTel init
//!   is lifted.
//!
//! For now this crate exposes a single trivial `version()` symbol
//! (matching the plan §10 T1 contract: "Empty crate, one passing
//! test, `cargo build -p phenotype-observability` green"). The
//! `pub use` re-export in this commit's description is aspirational.

/// Crate version, sourced from `Cargo.toml` at build time.
///
/// Trivial stand-in symbol so that this scaffold crate has a public
/// surface that compiles and tests. Will be removed once the OTel
/// init is wired through per the plan's §10 T3.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }
}
