# HexaKit changelog

## [Unreleased]

### Added

- `phenotype-observability` sub-crate per plans/2026-06-09-hexakit-phenotype-observability-plan-v1.md. Provides: OTel init helper (re-exported from phenotype-logging::telemetry::init_otel). The crate is the canonical home for OpenTelemetry init; consumers should migrate from `phenotype-logging::telemetry::init_otel` to `phenotype_observability::init_otel`. The full migration of `phenoObservability/` + `ObservabilityKit/` + `Tracera/internal/tracing/` into this crate is deferred to a follow-up wave.

### Changed

### Deprecated

### Removed

### Fixed

### Security

## 2026-04-29

- Cleaned the project root docs and worklog surfaces.
- Removed stale shelf-catalog references from the active docs.
- Rewrote the local `agileplus` project docs into clean project-root guides.

