# phenotype-observability

Canonical OpenTelemetry observability primitives for the Phenotype ecosystem.

## Status

Scaffold (`plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md`
§10 T1) plus the `otel` module placeholder. The `init_with_otel` and
`init_with_resource` symbols are present with byte-for-byte signatures
matching `HexaKit/crates/phenotype-logging/src/otel.rs:7,39`, but the
bodies are **honest stubs** that return `OTelError::NotWired` until the
OTel init is lifted into this crate per plan §10 T3. The gap is
machine-checked in `src/otel.rs::tests::gap_is_documented`.

See `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md` for
the full migration plan (init shape, `TelemetryGuard`, propagator,
sampler, span helpers, metrics + logs paths).

## Usage

### Current (scaffold + `otel` placeholder)

```rust
use phenotype_observability::{init_with_otel, OTelError, version};

// Stub call — returns `OTelError::NotWired` until plan §10 T3 lands.
// Does not spawn a Tokio runtime or open any network connection.
let r: Result<(), OTelError> =
    init_with_otel("my-service", "http://127.0.0.1:4317");
assert!(r.is_err());

// `version()` is the only real symbol today; it is retained as
// `CARGO_PKG_VERSION` re-export after the OTel init lands.
assert!(!version().is_empty());
```

### Post-migration (plan §10 T3 + T6)

```rust
use phenotype_observability::{init, ObservabilityConfig, Sampler};

let _guard = init(&ObservabilityConfig {
    service_name: "my-service".into(),
    service_version: env!("CARGO_PKG_VERSION").into(),
    otlp_endpoint: "http://127.0.0.1:4317".into(),
    environment: "production".into(),
    sampler: Sampler::ParentBasedRatio(0.1), // mirrors Tracera's tracer.go:81-86
    ..Default::default()
})?;
// `_guard` is a `TelemetryGuard`; its `Drop` impl calls `shutdown()`.
```

## Related crates

- [`phenotype-logging`](../phenotype-logging) — local `tracing-subscriber` config; the **source crate** for the OTel init that this crate re-homes (plan §3).
- [`phenotype-telemetry`](../phenotype-telemetry) — trait-only abstractions (`MetricsRecorder`, `Tracer`, `Logger`); the natural sibling for the OTel-backed implementations living here.
- [`phenotype-errors`](../phenotype-errors) — fleet-wide error types; `ObservabilityError` interoperates with this surface.
- [`phenotype-config-core`](../../libs/phenotype-config-core) — `ObservabilityConfig` is sourced from the shared config layer.
- [`phenotype-health`](../phenotype-health) — health-check facade that consumes observability signals.
- External substrate: `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-otlp`, `tracing-opentelemetry`, `tracing-subscriber` (versions pinned per `HexaKit/Cargo.toml:152-161`).
- Out-of-tree companion: `phenotype-otel-go` (Go module, planned) — extracted from `Tracera/backend/internal/tracing/tracer.go` per plan §6 / §8.3 Q3.5.

## FR traceability

| ID | Source | Description |
|---|---|---|
| **§8.3 Q3.1** | `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:354` | This crate is the **canonical observability home** inside HexaKit. |
| **§8.3 Q3.5** | `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:367` | Companion to: extract `phenotype-otel-go` for non-Tracera Go consumers. |
| **§9.2 task 3** | `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:503` | Scaffold sub-crate (Cargo.toml, src/lib.rs, README, single test) — **this README is the §9.2 T3 doc deliverable**. |
| **§9.2 task 4** | `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:506` | Port the working OTel init from `phenotype-logging/src/otel.rs:7-36` into this crate. |
| **§9.2 task 9** | `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:518` | First conformance test (PhenoAgent Targets table pattern). |
| **§9.2 task 10** | `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:520` | Cargo workspace wiring (add to `HexaKit/Cargo.toml` `members`, `cargo test --workspace` green). |
| **§10 T1** | `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md:700` | Scaffold (empty crate with one passing test). |
| **§10 T3** | `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md:725` | Port OTel init shape + add `TelemetryGuard` + first integration test. |
| **§10 T6** | `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md:768` | Delete OTel from `phenotype-logging` (drop OTel deps, remove `otel.rs`). |

## License

MIT OR Apache-2.0 (inherited from the HexaKit workspace per
`HexaKit/AGENTS.md`).
