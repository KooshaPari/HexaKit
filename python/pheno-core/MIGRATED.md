# Migration: pheno-core -> phenoShared

**Date:** 2026-06-17
**Disposition:** Wave F python redirect stub
**Canonical repo:** https://github.com/KooshaPari/phenoShared

## What changed

- Implementation ownership moves to **phenoShared**.
- This HexaKit path is a **pointer stub** until downstream references are cleared.

## For consumers

1. Install from phenoShared canonical repo, not HexaKit python/pheno-core.
2. Registry row: disposition-index **py-pheno-core**.

## For HexaKit maintainers

- Remove this directory after repoint PRs merge (follow-up).
