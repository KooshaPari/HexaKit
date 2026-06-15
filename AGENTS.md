# AGENTS.md — HexaKit

> **Operative guide for AI agents and human contributors working on the HexaKit repository.**
> The full project specification lives in [`SPEC.md`](./SPEC.md) — read it before making material changes.

## Project Identity

| Field | Value |
|-------|-------|
| Name | HexaKit |
| Description | Phenotype Infrastructure Kit — a Rust workspace of ~45 internal crates for observability, error handling, configuration, persistence, and policy. |
| Repository | <https://github.com/KooshaPari/HexaKit> |
| Canonical branch | `main` |
| Current release | see `VERSION` |
| License | MIT |

## Workspace Layout

```
HexaKit/
├── Cargo.toml              # Pure workspace — [workspace.package], [workspace], [workspace.dependencies]
├── crates/                 # Internal Rust crates (phenotype-*)
├── Traceon/                # tracingkit — distributed tracing (OTel)
├── Metron/                 # metrickit — metrics (Prometheus)
├── forgecode-fork/         # Vendored fork of forgecode
├── libs/                   # Cross-cutting libs (nexus, phenotype-config-core)
├── rust/                   # agileplus-proto — generated gRPC stubs
├── apps/                   # Submodule (byteport) — built independently
├── packages/               # Submodule (pheno-*) — built independently
├── platforms/              # Submodule (thegent) — built independently
├── src/                    # Submodule — built independently
├── docs/                   # Specs, ADRs, journey maps
├── scripts/                # Automation scripts
├── kitty-specs/            # Spec Kitty workflow artifacts
├── worklogs/               # Per-session agent worklogs
└── .github/workflows/      # 20 curated reusable workflows
```

**Workspace rules:**

- `[workspace]` is at the **root only** — sub-crates MUST NOT define their own `[workspace]`.
- New crates go under `crates/phenotype-<name>/` unless they are infrastructure-level (then `libs/`).
- All cross-crate deps use `phenotype-foo = { path = "..." }` from `[workspace.dependencies]`.
- Public re-exports: `phenotype-port-traits` is the SSOT for trait contracts.
- Submodule dirs (`apps/`, `packages/`, `platforms/`, `src/`) are `exclude`d from the workspace and built independently.

## Build & Test

```bash
# Workspace-wide
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
cargo doc --no-deps --workspace

# Or via just
just build
just test
just lint
just audit      # cargo-deny + cargo-audit
just unused     # cargo-machete
just ci         # lint + test + audit + unused
just coverage   # cargo-tarpaulin --workspace --out Html
```

**Toolchain:** pinned in `rust-toolchain.toml`. `Cargo.lock` is committed.

## Engineering Tenets

1. **Local-first.** Every tool must work offline. SQLite + git are the canonical state stores.
2. **AI-native.** MCP-friendly APIs, structured errors (no stringly-typed error chains), hash-chained audit trails.
3. **Type-safe boundaries.** New `From` impls in `phenotype-error-core` for each upstream error type. New trait methods in `phenotype-port-traits`.
4. **Zero-dep domains.** The `phenotype-domain` crate may not import anything outside its allowlist (enforced by `ci.yml::domain-deps-lint`).
5. **Workspace-shared metadata.** Crate versions, editions, lints all flow from `[workspace.package]` / `[workspace.lints]`.

## Git & Branch Discipline

| Rule | Detail |
|------|--------|
| Canonical branch | `main` — never commit directly |
| Feature branches | `feat/<topic>-<date>`, `fix/<topic>-<date>`, `chore/<topic>-<date>`, `docs/<topic>-<date>`, `test/<topic>-<date>`, `bench/<topic>-<date>`, `refactor/<topic>-<date>` |
| Branch retention | Keep **at most 10 local branches** at any time (10 most recent by commit date) |
| Commits | Conventional Commits enforced by `.commitlintrc.yml` |
| Worktrees | Use `git worktree add` for parallel work, prune aggressively |
| Tags | `vX.Y.Z` — automated by `release.yml` |

**Branch lifecycle:**
1. Branch off `main` (or the active release branch).
2. Commit small, atomic changes.
3. Open a PR. CI runs `ci.yml` (lint + test + audit + coverage + commitlint).
4. Squash-merge with a conventional commit message.
5. Delete the branch locally and on origin.

**Cleanup policy:** Branches older than the 10 most recent must be deleted. Stale worktrees (`git worktree list`) must be pruned. Stashes on deleted branches must be dropped.

## AgilePlus Mandate

All work MUST be tracked in AgilePlus. Before starting any material change:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus work-package create --title "<topic>" --branch "feat/<topic>-$(date +%Y%m%d)"
agileplus work-package start
# ... do the work ...
agileplus work-package done
```

Reference: <https://github.com/KooshaPari/AgilePlus>

## CI / CD

20 reusable workflows in `.github/workflows/`:

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | push, PR, dispatch | Main entry — buf, rust quality, coverage, Python, config lint, commitlint |
| `rust-quality.yml` | `workflow_call` | clippy + fmt + build + test (reusable) |
| `rust-release.yml` | `workflow_call` | release build with cross |
| `security-scan.yml` | `workflow_call` | SAST + secrets |
| `security.yml` | push, PR, dispatch | security hub |
| `snyk-scan.yml` | push, PR, dispatch | Snyk dependency vuln |
| `gate-check.yml` | `workflow_call` | quality gates (lint, test, audit) |
| `publish.yml` | `workflow_call` | publish to crates.io / npm / pypi |
| `promote.yml` | `workflow_call` | promote canary → main |
| `release.yml` | push (tag) | release orchestrator (binaries, SBOM, GitHub Release) |
| `release-drafter.yml` | push | draft release notes |
| `deploy.yml` | push (tag) | deploy |
| `benchmark.yml` | push, PR | benchmarks |
| `fuzzing.yml` | push, PR | cargo-fuzz |
| `docs.yml` | push, PR | build & deploy docs |
| `changelog.yml` | `workflow_call` | git-cliff changelog |
| `spec-validation.yml` | push, PR | validate SPEC.md, ADRs, journeys |
| `journey-gate.yml` | push, PR | user-journey gate |
| `audit.yml` | `workflow_call` | org-wide `pheno audit` |
| `ai-testing-orchestration.yml` | push, PR | AI-driven test orchestration |

**Removed (consolidated or superseded):** `cargo-audit`, `cargo-deny`, `cargo-machete`, `cargo-semver-checks`, `codeql`, `codeql-rust`, `evidence-capture`, `example-reusable`, `iac-scan`, `legacy-tooling-gate`, `libs-activation-ci`, `license-compliance`, `policy-gate`, `quality-gate`, `sast-full`, `sast-quick`, `sbom`, `scorecard`, `security-guard`, `security-guard-hook-audit`, `self-merge-gate`, `sonarcloud`, `sync-canary`, `tag-automation`, `traceability-gate`, `trivy-scan`, `trufflehog`, `workflow-maintenance`, `workflow-sync`, `zap-dast`, `alert-sync-issues`.

## Security & Compliance

- `SECURITY.md` documents the coordinated-disclosure process.
- `deny.toml` enforces license allowlist, advisory checks, duplicate detection, and source verification.
- All workflows use environment variables for shell interpolation to prevent injection.
- Pre-commit hook (`pre-commit-config.yaml`) runs gitleaks, typos, actionlint, and rustfmt.

## References

| Document | Purpose |
|----------|---------|
| [`SPEC.md`](./SPEC.md) | Full project specification (mission, tenets, architecture, governance) |
| `ADR-001.md`, `ADR-002.md`, `ADR-003.md`, `ADR.md`, `ADR_REGISTRY.md` | Architecture decision records |
| `GOVERNANCE.md` | Org governance model |
| `FUNCTIONAL_REQUIREMENTS.md` | FR traceability |
| `USER_JOURNEYS.md` | User journey maps |
| `SOTA.md` | State-of-the-art references |
| `WORKTREES.md` | Worktree conventions |
| `CONTRIBUTING.md` | Contribution guide |
| Parent workspace | `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md` |

## Quick Reference: Adding a New Crate

1. Create `crates/phenotype-<name>/` with `Cargo.toml`, `src/lib.rs`.
2. Add `"crates/phenotype-<name>"` to `[workspace.members]` in root `Cargo.toml`.
3. Add `phenotype-<name> = { path = "crates/phenotype-<name>" }` to `[workspace.dependencies]`.
4. Inherit `version`, `edition`, `license` from `[workspace.package]` via `version.workspace = true`, etc.
5. Add the crate's test surface to the relevant `gate-check` profile.
6. Update `SPEC.md` (project taxonomy) and `crates/phenotype-<name>/README.md`.

## Quick Reference: Adding a Reusable Workflow

1. Create `.github/workflows/<name>.yml` with `on.workflow_call` (and any push/PR triggers you need).
2. Declare `inputs:` and `secrets:` with explicit types.
3. Use `${{ inputs.foo }}` — never `secrets.*` directly in shell.
4. Document the workflow in `.github/workflows/README.md`.
5. Wire it into `ci.yml` or whichever orchestrator calls it.
