# Migration: phenotype-http-client-core → ResilienceKit

**Date:** 2026-06-17  
**Disposition step:** HexaKit DISPOSITION #23 — Wave D absorption stub  
**Canonical repo:** https://github.com/KooshaPari/ResilienceKit

## What changed

- Implementation ownership moves to **ResilienceKit**.
- This HexaKit path is a **pointer stub** until downstream references are cleared.
- Do not extend domain logic here; contribute to ResilienceKit instead.

## For consumers

1. Depend on phenotype-http-client-core from ResilienceKit (path or git pin), not HexaKit.
2. See DOMAIN_ROLES and disposition-index row id **23**.

## For HexaKit maintainers

- Remove this crate directory once workspace members and downstream refs are repointed.
