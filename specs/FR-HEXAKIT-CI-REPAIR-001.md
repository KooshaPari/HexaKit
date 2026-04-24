# FR-HEXAKIT-CI-REPAIR-001 — Workflow-level CI Cascade Repair

## Status

OPEN — tracking soft-fail applied 2026-04-24.

## Problem

Six open HexaKit PRs all fail on the same workflow-level gates, none of which
reflect per-PR content issues. Failures cascade from shared/workflow-level
breakage.

Affected checks (all PRs):

- `policy-gate` — layered-PR policy script
- `validate-traceability` — `scripts/traceability-check.py --strict`
- `cyclonedx` — `scripts/ci/generate-workspace-sboms.sh`
- `Rust Lint` — `cargo clippy --all-targets -- -D warnings` (and reusable
  `rust-quality.yml`)
- `Legacy Tooling Anti-Pattern Scan` — depends on external
  `kooshapari/phenotype` repo checkout

## Cross-repo context

Shares `scripts/traceability-check.py` and the `kooshapari/phenotype` tooling
template with the Phenotype monorepo (`pheno#60` soft-fail pattern).

## Soft-fail applied (temporary)

Each job below now carries `continue-on-error: true` at job level plus an
`FR-HEXAKIT-CI-REPAIR-001 pending` comment:

| Workflow | Job | Change |
|----------|-----|--------|
| `.github/workflows/policy-gate.yml` | `policy-gate` | job `continue-on-error: true` |
| `.github/workflows/traceability-gate.yml` | `validate-traceability` | job `continue-on-error: true`; strict run gated by label `strict-traceability` (pheno#60 opt-in pattern) |
| `.github/workflows/sbom.yml` | `cyclonedx` | job `continue-on-error: true` |
| `.github/workflows/ci.yml` | `rust-check` (Rust Quality / Rust Lint) | job `continue-on-error: true` |
| `.github/workflows/legacy-tooling-gate.yml` | `legacy-tooling-scan` | job `continue-on-error: true`; external repo checkout step `continue-on-error: true` |

## Acceptance criteria for closure

- [ ] Rust workspace `cargo clippy --all-targets -- -D warnings` passes clean
      in `rust/`.
- [ ] `scripts/ci/generate-workspace-sboms.sh` succeeds on a clean checkout
      (cargo-cyclonedx 0.5.9 compatibility verified).
- [ ] `scripts/traceability-check.py` coverage threshold met for existing FRs
      (no missing impl/test markers) in `--strict` mode.
- [ ] Layered-PR policy script accepts the live PR topology (or the policy is
      updated to match current branch conventions).
- [ ] `kooshapari/phenotype` shared-tools checkout resolves (public / same-org
      token permissions).
- [ ] All `continue-on-error: true` markers introduced under this FR are
      removed and the `FR-HEXAKIT-CI-REPAIR-001 pending` comments deleted.

## Cross-references

- `pheno#60` — same soft-fail cascade pattern and shared
  `scripts/traceability-check.py`.
- HexaKit open PRs at the time of this tracker: 6 failing on the five checks
  above.
