# Migration: phenotype-config-core → phenoShared

**Date:** 2026-06-19  
**Disposition:** Phase 4 wave 5 — HexaKit task #59  
**Canonical repo:** https://github.com/KooshaPari/phenoShared  
**Terminal owner (H14):** https://github.com/KooshaPari/phenotype-config (pending absorption)  
**Git pin:** `phenoShared` branch `main`

## What changed

- Local `libs/phenotype-config-core` source **pruned** — redirect stub only.
- HexaKit previously shipped a **struct-based** `ConfigLoader`; phenoShared canonical is a **trait** + `Priority` surface.
- `phenotype-core::config` re-exports aligned to phenoShared trait API (task #56 closeout).

## For consumers

```toml
phenotype-config-core = { git = "https://github.com/KooshaPari/phenoShared", branch = "main", package = "phenotype-config-core" }
```

## For HexaKit maintainers

- Do not reintroduce path deps on `libs/phenotype-config-core`.
- Repoint git pin to `phenotype-config` when terminal absorb lands (H14 task #83).
