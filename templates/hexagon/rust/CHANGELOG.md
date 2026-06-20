# Changelog

All notable changes to this project are documented here.
Format: [Semantic Versioning](https://semver.org) — `MAJOR.MINOR.PATCH`

## [Unreleased]

### Added
- Initial setup and automation configuration
- chore(absorb): merge pheno-cargo-template into HexaKit — see commit body.
  Absorbed 2026-06-20 from `KooshaPari/pheno-cargo-template` (root
  scaffold + `template/` cargo-generate partials) and
  `KooshaPari/Apisync` (`templates/hexagonal/` overlay, commit
  `d981353`). Single cargo-generate target at
  `templates/hexagon/rust/`. `pheno-cargo-template` repo retired and
  replaced with redirect.


---

## 2026-06-20 — Absorbed pheno-cargo-template

- **Source**: `KooshaPari/pheno-cargo-template` (now redirect-only)
- **Destination**: `KooshaPari/HexaKit/templates/hexagon/rust/`
- **Rationale**: DOMAIN_ROLES.md designates HexaKit as the canonical scaffolding/templates owner.

### Added

- `Cargo.toml` — root manifest with `sentry` + `tracing` feature flags and `[package.metadata.phenotype]` (ADR-041..045)
- `src/lib.rs` — library entrypoint with `sentry_config` accessors
- `src/sentry_config.rs` — Sentry integration (Apisync overlay)
- `justfile` — supplementary task runner
- `rust-toolchain.toml` — pinned toolchain
- `.clippy.toml` — project lints
- `llms.txt` — LLM-facing project facts
- `WORKLOG.md` — agent work log
- `.github/workflows/ci.yml` — CI pipeline
- `LICENSE-MIT` + `LICENSE-APACHE` — dual license
- `_partials/` — cargo-generate liquid fragments (preserved from `pheno-cargo-template/template/`)
  - `Cargo.toml`, `src/lib.rs`, `AGENTS.md.additions`, `PREDICTIVE.md`, `PROMOTION.md`, `CONTRIBUTING.md`
  - `.predict.yaml`, `.drift-detector.yaml`, `.framework-lint.yaml`
  - `.github/workflows/predictive-dry-check.yml`
  - `.agileplus/specs/001-core-setup/{meta.json,spec.md,tasks.md}`

### Merged into existing

- `Taskfile.yml` — added `timeout`, `deny`, `hygiene`, `release`, `coverage`, `fmt-fix` recipes
- `README.md` — absorb addendum
- `SPEC.md` — absorb addendum
- `AGENTS.md` — absorb addendum
- `CHANGELOG.md` — this entry

### Preserved (not modified)

- `clippy.toml`, `deny.toml`, `mise.toml`, `nextest.toml`, `rustfmt.toml`, `_typos.toml`, `cliff.toml`, `CONTRIBUTING.md`, `VERSION`, `CLAUDE.md`
