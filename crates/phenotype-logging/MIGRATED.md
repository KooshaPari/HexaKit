# Migration: phenotype-logging → PhenoObservability

**Date:** 2026-06-17  
**Disposition row:** HexaKit DISPOSITION #26 — `crates/phenotype-logging`  
**Canonical repo:** https://github.com/KooshaPari/PhenoObservability  
**Absorption map:** [wave-a-absorption.md](https://github.com/KooshaPari/PhenoObservability/blob/main/docs/disposition/wave-a-absorption.md)

## What changed

- This path received a **redirect stub** per [crate relocation runbook step 6](../../docs/operations/crate-relocation-runbook.md).
- Canonical logging facade ownership moves to **`PhenoObservability`** (`rust/phenotype-logging`).
- **Source is retained** in HexaKit for this wave — removal follows downstream repoint (runbook steps 4–5, 7).

## For consumers

1. Depend on `phenotype-logging` from **PhenoObservability**, not HexaKit path deps.
2. Do not add new workspace or path dependencies on `HexaKit/crates/phenotype-logging`.

## For HexaKit maintainers

- Wave A observability lane — do not relocate other observability crates in this PR.
- Remove this stub directory once downstream references are cleared (follow-up PR).
