# phenotype-observability

Canonical OpenTelemetry observability primitives for the Phenotype ecosystem.
Lives in the HexaKit workspace at `crates/phenotype-observability/`.

## `init_tracing()`

Installs a process-wide `tracing-subscriber` fmt layer. The body is gated by
`std::sync::Once`, so the subscriber is installed **at most once per process** —
every call after the first is a no-op and `init_count()` stays at `1`.

On the first call, it:

1. Builds a `tracing_subscriber::EnvFilter` from the `RUST_LOG` environment
   variable, falling back to `info` when unset.
2. Calls `tracing_subscriber::fmt().with_env_filter(...).try_init()` to
   register the subscriber as the process default. Any `Err` from a foreign
   subscriber already being installed is intentionally swallowed — the
   `Once` guard makes the error non-repeatable.
3. Bumps `init_count()` from `0` to `1`. The count never exceeds `1` because
   the `Once` body runs at most once.

It does **not** touch the OpenTelemetry pipeline — that lives behind
`otel::init_with_otel` / `otel::init_with_resource` (see below).

## `RUST_LOG` behaviour

Honoured via `EnvFilter::try_from_default_env()`. Standard `tracing`-style
directives are accepted; the filter is applied per-target.

| `RUST_LOG`               | Effect                                          |
|--------------------------|-------------------------------------------------|
| unset                    | Defaults to `info` for every target.            |
| `warn`                   | `warn` and above globally.                      |
| `my_crate=debug`         | `debug`+ for `my_crate`, `info` elsewhere.      |
| `my_crate=trace,info`    | Mixed per-target levels.                        |

Production default is to leave `RUST_LOG` unset (so `info` applies); set
`RUST_LOG=debug` for verbose local runs.

## `otel::init_tracing` (OpenTelemetry)

The `otel` module exposes the planned OTel init shape, currently as honest
stubs that return `OTelError::NotWired` (see `src/otel.rs` for the gap
analysis). Two entry points are pre-declared with byte-for-byte signatures
matching `phenotype-logging/src/otel.rs:7,39`:

- `otel::init_with_otel(service_name, otlp_endpoint) -> Result<(), OTelError>`
- `otel::init_with_resource(service_name, otlp_endpoint, attributes) -> Result<(), OTelError>`

Both will eventually bootstrap the OTLP/gRPC exporter via
`opentelemetry_otlp::SpanExporter::builder()`, install the
`tracing-opentelemetry` layer, and return a `TelemetryGuard` whose `Drop` impl
calls `shutdown()`. Until the lift per
`plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md` §10 T3 lands,
the bodies return `Err(OTelError::NotWired(...))` and **no network
connection is opened**.

## Example

```rust
use phenotype_observability::init_tracing;
fn main() {
    init_tracing();
    tracing::info!("service starting");
}
```

Set `RUST_LOG=my_bin=debug` to see debug spans from the binary.

## Consumers

Repositories that depend on `phenotype-observability` (or its planned
successor API) for tracing / OTel bootstrap:

- **phenoShared** — shared utilities; logging façade for fleet-wide code.
- **HeliosLab** — harness / lab runner; emits traces per experiment.
- **PhenoRuntime** — runtime host; per-process init and shutdown guard.
- **PhenoMCP-cheap** — cheap-llm MCP server; uses `RUST_LOG` for
  request-level tracing.
- **PhenoAgent** — agent process; OTel pipeline is the primary use case
  driving the `init_with_otel` lift.

## Status

Scaffold per `plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md`
§10 T1. `init_tracing` and `init_count` are real and idempotent; the OTel
stubs are placeholders pending §10 T3. The `tel` feature flag and the
removal of OTel from `phenotype-logging` are tracked under §10 T6.

## License

MIT OR Apache-2.0 (inherited from the HexaKit workspace).
