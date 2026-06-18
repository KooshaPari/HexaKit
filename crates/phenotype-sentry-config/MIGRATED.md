# Migration: phenotype-sentry-config → PhenoObservability

**Date:** 2026-06-18  
**Disposition row:** HexaKit DISPOSITION #35 — Wave A  
**Canonical repo:** https://github.com/KooshaPari/PhenoObservability  
**Git pin:** `PhenoObservability` branch `main` (PhenoObservability#168)

## What changed

- Local source **pruned** Phase 3 — this directory is a redirect stub only.
- Canonical implementation at `PhenoObservability/rust/phenotype-sentry-config`.

## For consumers

```toml
phenotype-sentry-config = { git = "https://github.com/KooshaPari/PhenoObservability", branch = "main", package = "phenotype-sentry-config" }
```
