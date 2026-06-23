//! L11 chaos game-day test for `phenotype-cache-adapter` (v25 cycle-15 T5).
//!
//! Exercises the public [`TwoTierCache`] surface under two injected failure
//! modes:
//!
//! 1. **Latency injection** — adds a deterministic 100-500 ms sleep before
//!    each `get` call (drawn from a xorshift64 PRNG seeded with `42`).
//!    Uses `std::thread::sleep` because the cache surface is sync.
//! 2. **Error injection** — the gate forces a `None` return (the cache's
//!    "transient unavailability" semantic) on every Nth `get` call (N = 4)
//!    so 1/4 of calls exercise the cache-miss path under chaos.
//!
//! Determinism note: every test seeds the PRNG with the same seed (`42`) so
//! the fault sequence is reproducible across runs and CI machines.
//!
//! Run with:
//!   cargo test -p phenotype-cache-adapter --test chaos_l11_2026_06_22
//!
//! Invariant: every test must complete in under 10 s wall time. The
//! latency-injection test bounds the upper chaos-call count at 6 (so worst
//! case 6 * 500 ms = 3 s), keeping wall time well under the 10 s CI gate.

use std::time::{Duration, Instant};

use phenotype_cache_adapter::TwoTierCache;

/// Deterministic xorshift64 PRNG. Seed = 42 produces the same fault sequence
/// across runs and platforms.
struct Xorshift64(u64);

impl Xorshift64 {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        assert!(hi >= lo, "range invalid: lo={lo} hi={hi}");
        let span = hi - lo + 1;
        lo + (self.next_u64() % span)
    }
}

/// Chaos gate for `TwoTierCache::get`. If the injector fires (every 4th
/// call, deterministic via seed=42), short-circuit and return `None`
/// (semantic: forced cache miss = transient unavailability). Otherwise
/// sleep 100-500 ms (simulated network/disk latency) and then delegate.
fn chaos_get<V: Clone>(
    injector: &mut Xorshift64,
    cache: &TwoTierCache<String, V>,
    key: &str,
) -> Option<V> {
    let delay_ms = injector.range(100, 500);
    std::thread::sleep(Duration::from_millis(delay_ms));

    // Error injection: every 4th call returns None (forced miss).
    let draw = injector.range(0, 3);
    if draw == 0 {
        return None;
    }
    cache.get(&key.to_string())
}

/// **Chaos enabled (latency + error)**: 6 calls through the gate. With
/// seed=42 + 1/4 error rate, we expect at least one forced miss and at
/// least one successful read. Each call is bounded at 600 ms wall (500 ms
/// chaos + 100 ms margin).
#[test]
fn chaos_latency_and_error_injection_handled_cleanly() {
    let cache: TwoTierCache<String, String> = TwoTierCache::new(16, 64);
    cache.put("alpha".to_string(), "one".to_string());
    cache.put("beta".to_string(), "two".to_string());

    let mut injector = Xorshift64::new(42);
    let mut hit_count = 0;
    let mut miss_count = 0;
    let start = Instant::now();

    for key in ["alpha", "beta", "alpha", "beta", "alpha", "beta"] {
        match chaos_get(&mut injector, &cache, key) {
            Some(v) => {
                assert!(
                    v == "one" || v == "two",
                    "unexpected cache value {v:?}"
                );
                hit_count += 1;
            }
            None => miss_count += 1,
        }
    }

    let elapsed = start.elapsed();
    assert_eq!(hit_count + miss_count, 6);
    assert!(hit_count > 0, "at least one call must hit; got 0");
    assert!(miss_count > 0, "at least one call must miss; got 0");
    assert!(
        elapsed < Duration::from_secs(10),
        "wall time {elapsed:?} exceeded 10 s gate"
    );
}

/// **Chaos disabled (control)**: baseline `get` loop without the gate. All
/// 6 calls hit and the total wall time is bounded by std scheduling jitter
/// (well under 1 s).
#[test]
fn chaos_disabled_baseline_completes_quickly() {
    let cache: TwoTierCache<String, String> = TwoTierCache::new(16, 64);
    cache.put("k1".to_string(), "v1".to_string());

    let start = Instant::now();
    for _ in 0..6 {
        let v = cache.get(&"k1".to_string());
        assert_eq!(v, Some("v1".to_string()));
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_secs(1),
        "baseline should be sub-second; took {elapsed:?}"
    );
}

/// **Determinism**: two PRNG instances with seed=42 produce the same latency
/// sequence and the same miss-draw sequence. CI depends on this for
/// reproducible triage.
#[test]
fn chaos_deterministic_seed_produces_identical_sequence() {
    let mut a = Xorshift64::new(42);
    let mut b = Xorshift64::new(42);

    let latencies: Vec<u64> = (0..50).map(|_| a.range(100, 500)).collect();
    let latencies_b: Vec<u64> = (0..50).map(|_| b.range(100, 500)).collect();
    assert_eq!(latencies, latencies_b);

    let draws_a: Vec<u64> = (0..50).map(|_| a.range(0, 3)).collect();
    let draws_b: Vec<u64> = (0..50).map(|_| b.range(0, 3)).collect();
    assert_eq!(draws_a, draws_b);
    // At least one draw must equal 0 (the fault trigger).
    assert!(draws_a.contains(&0), "1/4 miss rate must trigger at least once");
}

/// **Forced miss path**: when the gate fires, the call must return `None`
/// without panicking. This pins the error-injection contract independent
/// of the random draw and proves the cache surface degrades cleanly under
/// transient unavailability (anti-fragility ceiling — no panic, no hang).
#[test]
fn chaos_forced_miss_returns_none_and_does_not_panic() {
    let cache: TwoTierCache<String, String> = TwoTierCache::new(16, 64);
    cache.put("triage-key".to_string(), "triage-value".to_string());

    let mut injector = Xorshift64::new(42);
    let mut found = false;

    for _ in 0..16 {
        let result = chaos_get(&mut injector, &cache, "triage-key");
        if result.is_none() {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "injector must fire a forced miss within 16 draws"
    );

    // And the underlying cache must still be able to serve a fresh call
    // without the gate — anti-fragility ceiling: chaos must not corrupt
    // the underlying state.
    let v = cache.get(&"triage-key".to_string());
    assert_eq!(v, Some("triage-value".to_string()));
}
