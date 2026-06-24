# golangci-ext

Extended Go linting and analysis tooling absorbed from [KodeVibeGo](https://github.com/KooshaPari/KodeVibeGo) (Phase-1-Absorption).

## Provenance

- **Source**: `KooshaPari/KodeVibeGo` — HOLD_ARCHIVE → PROTECTED
- **Absorption Date**: 2026-06-23
- **Reason**: KodeVibeGo Go module (14 pkgs, 38 .go files) is a well-audited Go project with zero cross-repo deps. Its tooling config serves as the canonical Go lint/CI template for Phenotype-org.
- **DAG Ref**: `dag-manifest-2026-06-23.json` → L1-Alpha A-02/A-03/A-04

## Contents

| File | Source | Purpose |
|------|--------|---------|
| `.golangci.yml` | KodeVibeGo | Go linter config (golangci-lint) |
| `Makefile` | KodeVibeGo | Build automation |
| `cliff.toml` | KodeVibeGo | Changelog generation (git-cliff) |
| `codecov.yml` | KodeVibeGo | Code coverage thresholds |

## Usage

```bash
cp tools/golangci-ext/.golangci.yml my-go-project/
```

## Audit

- **Zero** cross-repo replace directives in go.mod
- **Zero** KooshaPari org references in source code
- All 14 packages and 38 .go files accounted for
