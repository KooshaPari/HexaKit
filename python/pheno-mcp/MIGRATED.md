# Migration: pheno-mcp -> PhenoMCP

**Date:** 2026-06-17
**Disposition:** Wave F python redirect stub
**Canonical repo:** https://github.com/KooshaPari/PhenoMCP

## What changed

- Implementation ownership moves to **PhenoMCP**.
- This HexaKit path is a **pointer stub** until downstream references are cleared.

## For consumers

1. Install from PhenoMCP canonical repo, not HexaKit python/pheno-mcp.
2. Registry row: disposition-index **py-pheno-mcp**.

## For HexaKit maintainers

- Remove this directory after repoint PRs merge (follow-up).
