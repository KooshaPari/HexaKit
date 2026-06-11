//! Criterion benchmarks for `phenotype-observability::init_tracing`.
//!
//! `init_tracing()` is a one-shot subscriber installer gated by
//! [`std::sync::Once`] — see `phenotype-observability/src/lib.rs`
//! for the contract. Criterion times each call as a black box, so
//! the first iteration of every bench is the "real" install
//! (reads `RUST_LOG`, builds the `EnvFilter`, calls
//! `tracing_subscriber::fmt().try_init()`) and every subsequent
//! iteration is the `Once` fast path. That is exactly what we want
//! to measure: the public API cost of `init_tracing()` from a
//! fresh process, and the steady-state cost after the first call.
//!
//! Two scenarios, matching the `RUST_LOG` ↔ no-`RUST_LOG` axis in
//! the bench task spec:
//!
//!   1. `init_tracing_default` — runs with `RUST_LOG` **unset**.
//!      The init body falls back to the `"info"` default
//!      `EnvFilter`.
//!   2. `init_tracing_with_rust_log` — runs with
//!      `RUST_LOG=info` set on the bench's process env, so the
//!      `EnvFilter::try_from_default_env` path in `init_tracing`
//!      succeeds without the `unwrap_or_else` fallback.
//!
//! Both benches call `init_tracing()` directly. The `Once` guard
//! inside the function is intentionally **not** defeated — the
//! point of the bench is to time the public API as callers will
//! see it (first call = real install, later calls = no-op).
//!
//! Run with:
//!     cargo bench -p phenotype-observability --bench init_tracing
//! or filtered:
//!     cargo bench -p phenotype-observability --bench init_tracing init_tracing

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phenotype_observability::{init_count, init_tracing};

/// Snapshot the bench binary's `RUST_LOG` value at startup so the
/// "default" bench can clear it deterministically even when the
/// surrounding shell / `.mise.toml` exports one (the workspace's
/// `.mise.toml` sets `RUST_LOG = "info,agileplus=debug"`, which
/// would otherwise contaminate the "no RUST_LOG" measurement).
const INITIAL_RUST_LOG_AT_STARTUP: Option<&'static str> = option_env!("RUST_LOG");

/// Bench: `init_tracing()` with no `RUST_LOG` set in the env.
///
/// We snapshot whatever `RUST_LOG` value the process was launched
/// with (see [`INITIAL_RUST_LOG_AT_STARTUP`]) and restore it after
/// the bench so we don't leak global state into other benches or
/// the `with_rust_log` case.
fn bench_init_tracing_default(c: &mut Criterion) {
    // Clear `RUST_LOG` so `EnvFilter::try_from_default_env` falls
    // back to the `"info"` default inside `init_tracing`.
    //
    // `std::env::remove_var` became `unsafe` in Rust 1.85; the
    // workspace's pinned toolchain (`rustc 1.95.0`) sits above
    // that, so the call is wrapped in an `unsafe` block.
    unsafe { std::env::remove_var("RUST_LOG") };

    c.bench_function("init_tracing_default", |b| {
        b.iter(|| {
            init_tracing();
            // Touch `init_count` to make sure the compiler keeps
            // the call — `init_tracing` is the unit under test,
            // but `init_count` shares the `Once` statics and
            // helps the optimizer see this isn't dead code.
            black_box(init_count());
        });
    });

    // Best-effort restore of the startup env. We don't propagate
    // errors here because the bench framework ignores the return
    // value of the closure anyway.
    if let Some(v) = INITIAL_RUST_LOG_AT_STARTUP {
        unsafe { std::env::set_var("RUST_LOG", v) };
    }
}

/// Bench: `init_tracing()` with `RUST_LOG=info` set in the env.
///
/// This exercises the `EnvFilter::try_from_default_env()` success
/// path inside `init_tracing`, which has to parse the directives
/// string and build a filter — slightly more work than the
/// fallback path in [`bench_init_tracing_default`].
fn bench_init_tracing_with_rust_log(c: &mut Criterion) {
    // `std::env::set_var` became `unsafe` in Rust 1.85; wrap in
    // an `unsafe` block to silence the lint and to stay portable
    // to any future deny-by-default lint the workspace may adopt.
    unsafe { std::env::set_var("RUST_LOG", "info") };

    c.bench_function("init_tracing_with_rust_log", |b| {
        b.iter(|| {
            init_tracing();
            black_box(init_count());
        });
    });

    if let Some(v) = INITIAL_RUST_LOG_AT_STARTUP {
        unsafe { std::env::set_var("RUST_LOG", v) };
    }
}

criterion_group!(
    benches,
    bench_init_tracing_default,
    bench_init_tracing_with_rust_log
);
criterion_main!(benches);
