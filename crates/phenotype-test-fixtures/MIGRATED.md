# Migration: phenotype-test-fixtures → TestingKit

**Date:** 2026-06-17  
**Disposition step:** HexaKit DISPOSITION #41 — Wave B testing lane stub  
**Canonical repo:** https://github.com/KooshaPari/TestingKit

## What changed

- Test fixtures are **already present** in TestingKit at `rust/phenotype-test-fixtures`.
- This HexaKit copy is a **duplicate**; canonical implementation lives in TestingKit.
- This path is a pointer stub until HexaKit workspace membership and deps are cleared.

## For consumers

1. Depend on `phenotype-test-fixtures` from TestingKit, not HexaKit.
2. See [TestingKit wave-b absorption doc](https://github.com/KooshaPari/TestingKit/blob/main/docs/disposition/wave-b-absorption.md).

## For HexaKit maintainers

- Remove this crate from the HexaKit workspace once downstream refs are repointed (follow-up PR).
- Registry row: `disposition-index.json` id **41**, wave **B**, target **TestingKit**.
