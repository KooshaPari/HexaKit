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
//! - [`init_tracing`] — installs a process-wide `tracing-subscriber`
//!   at most once (gated by [`std::sync::Once`]). Honours
//!   `RUST_LOG` via `EnvFilter::try_from_default_env` and falls
//!   back to `info` when unset. See [`init_count`] for the
//!   one-shot bookkeeping that goes with it.
//! - [`init_count`] — returns the number of times the body of
//!   [`init_tracing`] has actually run. `0` before the first call,
//!   `1` from the first call onward (the `Once` guard prevents the
//!   body from re-running, so the count never exceeds `1`).
//! - [`otel`] module — see the gap analysis there. Re-exports
//!   [`otel::OTelError`], [`otel::init_with_otel`],
//!   [`otel::init_with_resource`], and the
//!   [`otel::PHENOTYPE_LOGGING_EXPORTS_OTEL_INIT`] gap flag at the
//!   crate root via `pub use otel::*;` below.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Once;

/// One-shot guard around the body of [`init_tracing`]. `Once::call_once`
/// ensures the subscriber install + counter bump run at most once per
/// process; subsequent calls short-circuit without touching global state.
static INIT_TRACING_ONCE: Once = Once::new();

/// Bookkeeping for [`init_count`]. Initialised to `0`; bumped to `1`
/// inside the `INIT_TRACING_ONCE` closure. Held as `AtomicU64` so
/// [`init_count`] can be called from any thread without a lock.
static INIT_COUNT: AtomicU64 = AtomicU64::new(0);

/// Crate version, sourced from `Cargo.toml` at build time.
///
/// Trivial stand-in symbol so the scaffold crate has a public
/// surface that compiles and tests. Will be retained as a re-export
/// of `CARGO_PKG_VERSION` after the OTel init lands.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Initialise the global [`tracing_subscriber`] fmt subscriber.
///
/// **Idempotent.** The body is gated by [`std::sync::Once`], so the
/// underlying `tracing_subscriber::fmt::Subscriber` is installed at
/// most once per process. Subsequent calls (including the second,
/// third, … call in the same test binary) are no-ops and return
/// immediately after touching the `Once` flag.
///
/// On the first call, this:
///
/// 1. Builds a `tracing_subscriber::EnvFilter` from the `RUST_LOG`
///    environment variable when set, falling back to `info` when it
///    is not.
/// 2. Calls `tracing_subscriber::fmt().with_env_filter(...).try_init()`
///    to register the subscriber as the process-wide default.
///    `try_init` is itself idempotent (it returns `Err` if a global
///    subscriber was already installed by some other code path);
///    any such error is intentionally swallowed here so that
///    `init_tracing` stays non-fatal — the `Once` guard means the
///    error cannot repeat.
/// 3. Bumps [`INIT_COUNT`] to `1` (see [`init_count`]).
///
/// **Does not** touch the OTel pipeline: that lives behind
/// [`otel::init_with_otel`] / [`otel::init_with_resource`] until the
/// lift per plan §10 T3 lands.
pub fn init_tracing() {
    INIT_TRACING_ONCE.call_once(|| {
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
        // `try_init` returns `Err` only if some other code path
        // already installed a global subscriber. The `Once` guard
        // makes that case unreachable from *this* function, so an
        // error here means external interference. We deliberately
        // ignore it: `init_tracing` is best-effort and non-fatal.
        let _ = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .try_init();
        // Publish the count last so callers that race
        // `init_count()` against `init_tracing()` always observe
        // either `0` (init body hasn't finished) or `1` (it has).
        // The body never re-runs, so the count never exceeds `1`.
        INIT_COUNT.store(1, Ordering::SeqCst);
    });
}

/// Return the number of times the body of [`init_tracing`] has
/// actually run in this process.
///
/// Contract:
///
/// - `0` before the first call to [`init_tracing`].
/// - `1` once [`init_tracing`] has been invoked at least once, and
///   **stays at `1`** for every subsequent call. The
///   [`std::sync::Once`] guard around the init body prevents the
///   counter from ever exceeding `1` — the body that would bump
///   it runs at most once.
///
/// This is purely a bookkeeping / observability helper for callers
/// (tests, benches) that want to assert idempotency.
pub fn init_count() -> u64 {
    INIT_COUNT.load(Ordering::SeqCst)
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

    /// `init_count()` is order-independent: tests share a single
    /// test binary in the same process, so by the time this test
    /// runs, `init_tracing()` may already have been called by
    /// `init_count_increments_on_init` (or vice versa). What we
    /// verify here is the **upper bound**: the `Once` guard around
    /// the init body means the count can never exceed `1`, and the
    /// pre-init value is `0`. So `init_count()` must return
    /// **either** `0` **or** `1` — never `2+`, never negative.
    #[test]
    fn init_count_is_zero_or_one_before_init() {
        let observed = init_count();
        assert!(
            observed == 0 || observed == 1,
            "init_count() must be 0 (pre-init) or 1 (post-init), got {observed}"
        );
    }

    /// `init_tracing()` is idempotent and never raises the count
    /// past `1`. The first call drives the count to `1`; any
    /// further call is a no-op and leaves the count untouched.
    ///
    /// Note: tests in the same binary share process state. The
    /// `Once` guard means the count is already `1` from the very
    /// first invocation in this process, regardless of which test
    /// triggered it. We therefore assert:
    ///   1. After at least one call to `init_tracing`, `init_count()`
    ///      is exactly `1` (the `Once` body has run exactly once).
    ///   2. A second call to `init_tracing()` does **not** raise
    ///      the count — it stays at `1`.
    #[test]
    fn init_count_increments_on_init() {
        // First call: the `Once` body runs (or already ran earlier
        // in this binary) and bumps the count to `1`.
        init_tracing();
        let after_first = init_count();
        assert!(
            after_first >= 1,
            "init_count() must be >= 1 after init_tracing(), got {after_first}"
        );

        // Second call: gated by the same `Once`, so the body is
        // skipped and the count must not move.
        init_tracing();
        let after_second = init_count();
        assert_eq!(
            after_second, 1,
            "init_count() must stay at 1 across repeated init_tracing() calls; \
             observed {after_first} -> {after_second}"
        );
    }
}
