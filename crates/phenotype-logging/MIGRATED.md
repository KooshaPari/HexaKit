# Migration: phenotype-logging → PhenoObservability

**Date:** 2026-06-17  
**Disposition row:** HexaKit DISPOSITION #26 — `crates/phenotype-logging`  
**Canonical repo:** https://github.com/KooshaPari/phenoShared  
**Git pin:** HexaKit workspace `phenoShared` branch `main` (wave 3 #258)

## What changed

- Implementation ownership is **phenoShared** (not PhenoObservability path dep).
- Local source **pruned** wave 13 — this directory is a redirect stub only.

## For consumers

1. Depend on `phenotype-logging` from **phenoShared** (git pin), not HexaKit path deps.
2. Do not add new workspace or path dependencies on `HexaKit/crates/phenotype-logging`.

## For HexaKit maintainers

- Wave A observability lane — do not relocate other observability crates in this PR.
- Remove this stub directory once downstream references are cleared (follow-up PR).
