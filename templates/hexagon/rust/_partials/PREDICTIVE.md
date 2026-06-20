# PREDICTIVE.md — absorbed 2026-06-20 from
# KooshaPari/pheno-cargo-template/template/PREDICTIVE.md.
#
# Predictive-DRY lint policy that this scaffold encodes. Generated
# crates inherit this policy verbatim unless they override it.

## Predictive-DRY lint (per ADR-042)

A scaffolded crate must not introduce a new dependency, file path, or
naming convention that contradicts the canonical patterns below:

| Concern | Canonical pattern | Reference |
|---------|-------------------|-----------|
| Logging | `tracing::info!`, `tracing::error!` (no `println!` in `src/`) | ADR-043 |
| Errors | `thiserror` for libs, `anyhow` for binaries | ADR-044 |
| Async | `tokio` runtime, `async-trait` only at edges | ADR-045 |
| Config | `figment` + `serde`, env-var fallback | ADR-041 |
| Tests | `proptest` for property-based; `insta` for snapshots | ADR-042 |
| Schema | `schemars::JsonSchema` on all public types | ADR-041 |

The `predictive-dry-check` CI job fails the build if any of the
canonical patterns is violated; see
`.github/workflows/predictive-dry-check.yml`.

## Drift detector

`.drift-detector.yaml` runs once per release and compares this crate's
patterns against the canonical repo. Drift > 5% triggers an auto-PR
to reconcile.