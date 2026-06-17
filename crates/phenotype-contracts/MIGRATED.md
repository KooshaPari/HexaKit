# Migration: phenotype-contracts → TestingKit

**Date:** 2026-06-17  
**Disposition step:** HexaKit DISPOSITION #11 — Wave B testing lane stub  
**Canonical repo:** https://github.com/KooshaPari/TestingKit

## What changed

- Contract value types ownership moves to **TestingKit** (`rust/phenotype-contracts` when landed).
- This HexaKit path is a **pointer stub** until downstream references are cleared.
- Relocate together with `phenotype-contract` and `phenotype-contract-tests`.

## For consumers

1. Depend on `phenotype-contracts` from TestingKit, not HexaKit.
2. See [TestingKit wave-b absorption doc](https://github.com/KooshaPari/TestingKit/blob/main/docs/disposition/wave-b-absorption.md).

## For HexaKit maintainers

- Remove this crate directory once workspace members and downstream refs are repointed (follow-up PR).
- Registry row: `disposition-index.json` id **11**, wave **B**, target **TestingKit**.
