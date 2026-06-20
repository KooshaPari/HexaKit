# Contributing — absorbed 2026-06-20 from
# KooshaPari/pheno-cargo-template/template/CONTRIBUTING.md.

## How to contribute

1. Fork & branch from `main`.
2. Make your change. Run `task ci` (or `just ci`) locally.
3. Commit using Conventional Commits; reference a task ID if one
   exists.
4. Open a PR. The CI pipeline enforces: fmt + clippy + test +
   audit + deny + coverage.
5. Wait for one core maintainer review.

## Code style

- rustfmt default.
- `#![warn(missing_docs)]` on the lib root.
- No `unwrap()` / `expect()` outside `#[cfg(test)]`.
- Errors via `thiserror` for libraries, `anyhow` for binaries.

## Reporting issues

- Bug reports: use the bug report template; include a minimal
  reproduction.
- Security: see `SECURITY.md` (TBD).

## Tier promotions

See `PROMOTION.md`.