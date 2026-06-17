# Migration: phenotype-mcp → McpKit

**Date:** 2026-06-17  
**Disposition step:** HexaKit DISPOSITION #28 — Wave D absorption stub  
**Canonical repo:** https://github.com/KooshaPari/McpKit

## What changed

- Implementation ownership moves to **McpKit**.
- This HexaKit path is a **pointer stub** until downstream references are cleared.
- Do not extend domain logic here; contribute to McpKit instead.

## For consumers

1. Depend on phenotype-mcp from McpKit (path or git pin), not HexaKit.
2. See DOMAIN_ROLES and disposition-index row id **28**.

## For HexaKit maintainers

- Remove this crate directory once workspace members and downstream refs are repointed.
