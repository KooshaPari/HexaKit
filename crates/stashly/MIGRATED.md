# Migration: stashly → resilience role

**Date:** 2026-06-17  
**Disposition row:** HexaKit DISPOSITION #46 — `crates/stashly`  
**ADR:** [ADR-ECO-001](https://github.com/KooshaPari/phenotype-registry/blob/main/docs/adrs/ADR-ECO-001.md)  
**Canonical target:** `phenotype-resilience` Rust workspace (pending) · Py edge in phenotype-python-sdk

## What changed

- Removed `crates/stashly` from HexaKit workspace members (Wave 3 excision).
- **Source retained** as redirect stub until fleet repoint completes.

## For consumers

```toml
# Transitional (until phenotype-resilience ships)
stashly = { git = "https://github.com/KooshaPari/HexaKit", branch = "main", package = "stashly" }
```

Pyron repointed in lockstep — see [wave3-lockstep-repoint](https://github.com/KooshaPari/Pyron/blob/main/docs/migrations/wave3-lockstep-repoint-2026-06-17.md).

## For HexaKit maintainers

- Do not add new cache-domain code under `crates/stashly`.
- Remove stub when zero external path deps remain.
