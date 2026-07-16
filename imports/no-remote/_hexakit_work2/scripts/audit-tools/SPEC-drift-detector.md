# SPEC — `pheno-drift-detector`

**Version:** 0.1.0
**Date:** 2026-06-19
**Status:** ACTIVE
**L5 ID:** L5-111

---

## 1. Purpose

`pheno-drift-detector` scans PAUSED / CONDITIONAL / CAPSTONE app repos (per
ADR-023 "App-level repo triage" and the AGENTS.md `PAUSED APPs` table) for
**2+ non-trivial capabilities** that match a substrate pattern. When detected,
it emits structured drift hits as JSON / Markdown / GitHub-issue-ready output.

The tool is the **L74 governance tool** of the v1.1 71-pillar forward-looking
cross-reference set (see `findings/71-pillar-2026-06-17-schema.md §4`). It
runs weekly on the heavy-runner (per ADR-023 device-fit gate).

## 2. The 3 buckets × capability-extraction rules

### 2.1 The 3 buckets (per ADR-023)

| Bucket | Examples (from AGENTS.md `PAUSED APPs`) | Action on hit |
|---|---|---|
| `paused` | `focalpoint`, `QuadSGM`, `WSM`, `*fitness*` | Extract → `pheno-*-lib` (defer) |
| `conditional` | `Dino`, `HwLedger` | Extract → `phenotype-*-sdk` or `phenotype-*-framework` (act) |
| `capstone` | `AtomsBot`, `AtomsBot-2nd..5th` | Reference-only, do not extract (legally mined) |

### 2.2 The capability-extraction rules

A repo is flagged as a drift hit when:

1. **Bucket match:** the repo name matches one of the 3 bucket sets (with glob support).
2. **Capability threshold:** ≥ 2 non-trivial capabilities (each ≥ 3 source files, ≥ 5 KB total).
3. **Drift score ≥ 1.5** (ADR-043 §4): `1.0 × n_caps + 0.4 × n_ports + 0.3 × n_adapters + 0.3 × n_tests`.

### 2.3 The substrate-placement rule

| Capability profile | Suggested substrate |
|---|---|
| ≥ 2 Ports + ≥ 2 Adapters | `phenotype-*-framework` |
| ≥ 1 Port + ≥ 1 Adapter | `phenotype-*-sdk` |
| ≥ 1 Port only | `pheno-*-lib` |
| No Port found | `pheno-*-lib` (TBD — manual review) |

## 3. CLI surface (3 commands)

```text
pheno-drift-detector scan      --root PATH [--format json|md|gh-issues] [--out FILE]
pheno-drift-detector validate  --hit HIT_JSON [--yes]
pheno-drift-detector --help
pheno-drift-detector --version  (via setuptools-scm, future)
```

Exit codes:

- `0` — no drift hits
- `1` — scan error (bad path, bad args)
- `2` — drift hits found (CI can use this to fail PRs or open issues)

## 4. Output schemas

### 4.1 `json` — machine-readable

```json
[
  {
    "repo": "HwLedger",
    "bucket": "conditional",
    "capabilities": [
      {"dir": "apps/macos", "file_count": 12, "total_bytes": 18432,
       "has_port": true, "has_adapter": true, "has_test": true, "ports": ["src/lib.rs:42: trait CapacityEstimator {"]}
    ],
    "drift_score": 2.4,
    "candidate_paths": ["apps/macos"],
    "target_substrate": "phenotype-*-framework",
    "rationale": "Found 1 non-trivial capabilities, 1 with Port trait, ...",
    "suggested_action": "Extract 'apps/macos' (and related) from HwLedger into a new ...",
    "matched_files": ["apps/macos/*"],
    "detected_at": "2026-06-19T12:00:00+00:00"
  }
]
```

### 4.2 `md` — human-readable summary table

See `render_md()` in `pheno_drift_detector.py:292-312`.

### 4.3 `gh-issues` — `gh issue create --body-file` ready

See `render_gh_issues()` in `pheno_drift_detector.py:261-289`. 5 sections per hit: header, score, target, rationale, capabilities table, suggested action, candidate paths.

## 5. Non-goals

- **No AST parsing.** Heuristic, regex-based. False positives are fine; false negatives are not.
- **No bus factor analysis.** The `bus_factor_penalty` term in the original docstring is stale and not implemented. P1 cleanup.
- **No `--since` filter.** The README's `--since 7d` example is stale. P1 cleanup.

## 6. Test coverage

- 12 in-process unit tests (rule coverage per ADR-023 placement)
- 5 subprocess E2E tests (`--help`, version/runtime, scan PAUSED, scan CAPSTONE, JSON output)

## 7. Quality bar (Rule 3.1)

| Artifact | Status |
|---|---|
| `AGENTS.md` | ✓ |
| `LICENSE-MIT` | ✓ (MIT, KooshaPari 2026) |
| `pyproject.toml` | ✓ (v0.1.0, console_script) |
| `deny.toml` + `.safety-policy.yml` | ✓ (stub for Python; pip-audit in CI) |
| `SPEC.md` | ✓ (this file, 1-page) |
| `CHANGELOG.md` | ✓ (v0.1.0) |
| `.github/workflows/ci.yml` | ✓ (Python 3.10/3.11/3.12 matrix, pytest, pip-audit) |
| `tests/test_smoke.py` | ✓ (17 tests) |
| `WORKLOG.md` (v2.1) | (deferred to fleet-wide migration; ADR-025) |

---

**Related:** `AGENTS.md` § 2, `findings/71-pillar-2026-06-17-schema.md §3.10`, ADR-023, ADR-024, ADR-035, `ops/heavy-runner-cron/INSTALL.md`.
