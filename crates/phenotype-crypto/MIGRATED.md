# Migration: phenotype-crypto → Authvault

**Date:** 2026-06-17  
**Disposition step:** HexaKit DISPOSITION #15 — Wave C absorption stub  
**Canonical repo:** https://github.com/KooshaPari/Authvault

## What changed

- Implementation ownership moves to **Authvault**.
- This HexaKit path is a **pointer stub** until downstream references are cleared.
- Do not extend domain logic here; contribute to Authvault instead.

## For consumers

1. Depend on phenotype-crypto from Authvault (path or git pin), not HexaKit.
2. See DOMAIN_ROLES and disposition-index row id **15**.

## For HexaKit maintainers

- Remove this crate directory once workspace members and downstream refs are repointed.
