# phenotype-observability

Canonical OpenTelemetry observability primitives for the Phenotype ecosystem.

## Role

Re-homes the OTel init shape that currently lives in
`HexaKit/crates/phenotype-logging/src/otel.rs` (the only known-working
OTLP/gRPC bootstrap in the fleet) into a dedicated sub-crate, so
`phenotype-logging` can drop its OTel dependencies and stay focused on
local `tracing-subscriber` configuration. See
`plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md` for the
full plan.

## Status

Scaffold only (plan §10 T1). The crate compiles and exposes a single
`version()` symbol; the OTel init re-export is intentionally deferred
because `phenotype-logging` does not currently publish the OTel
init symbols — see the doc-comment in `src/lib.rs` for the documented
gap.

## Usage (post-scaffold)

Once the OTel init is lifted out of `phenotype-logging` and into this
crate (plan §10 T3), consumers will write:

```rust
use phenotype_observability::init_otel;
init_otel("my-service", "http://127.0.0.1:4317")?;
```
