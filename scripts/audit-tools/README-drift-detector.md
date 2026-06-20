# pheno-drift-detector

**App-substrate drift detector.**

`pheno-drift-detector` scans **PAUSED / CONDITIONAL / CAPSTONE** app repos
for **2+ non-trivial capabilities** that match the substrate pattern (per
[ADR-023 Rule 3](AGENTS.md)). When detected, it outputs
GitHub-issue-ready JSON or Markdown for the weekly heavy-runner cron →
issue auto-creation.

This is one of three companion substrate-scanners in the v8 weekly
heavy-runner cron bundle:

- `pheno-predict` — similar-code scanner (companion)
- `pheno-drift-detector` — **app-substrate drift detector** (this repo)
- `pheno-framework-lint` — tier-convention enforcer (companion)

## Install

```bash
chmod +x pheno_drift_detector.py
ln -s "$(pwd)/pheno_drift_detector.py" /usr/local/bin/pheno-drift-detector
./pheno_drift_detector.py --help
```

Or via `pip` (PEP 621):

```bash
pip install -e ".[test]"
pheno-drift-detector --help
```

## Usage

### Scan the fleet for drift hits

```bash
pheno-drift-detector scan \
    --root .. \
    --format gh-issues \
    --out drift-hits.md
```

`--root` is the directory containing the app repos. The detector walks
each subdirectory, infers its ADR-023 bucket from the repo name
(PAUSED / CONDITIONAL / CAPSTONE), and applies the 4-criterion candidate
profile.

### Validate a single hit (HITL gate)

```bash
pheno-drift-detector validate --hit drift-hits/hit-0.json --yes
```

HITL gate: human must confirm before extraction PR is opened.

## Algorithm (3 passes per ADR-023 Rule 3)

### Pass 1 — Discover app repos
Walk `--root`; for each subdirectory, check if its name matches an ADR-023
bucket (see `PAUSED_APPS`, `CONDITIONAL_APPS`, `CAPSTONE_APPS` in
`pheno_drift_detector.py:47-55`). If yes, schedule for scanning.

### Pass 2 — Find non-trivial capabilities
For each candidate app repo, group source files by top-level directory.
A "non-trivial capability" must have:
- ≥ 3 source files
- ≥ 5 KB total
- at least one file matching a Port trait pattern
  (`trait Foo {`, `interface Foo {`, `protocol Foo`, `impl X for Y`, etc.)

### Pass 3 — Score + suggest
Drift score = `1.0·n + 0.4·n_ports + 0.3·n_adapters + 0.3·n_tests`.
Threshold: **1.5** (per `DRIFT_THRESHOLD`). Hits above the threshold get:
- **Target substrate**: `pheno-*-lib` (Port only) / `phenotype-*-sdk` (Port + Adapter)
  / `phenotype-*-framework` (≥ 2 Ports + ≥ 2 Adapters) / federated-service.
- **Suggested action**: extract `cap[0].dir` (and related) into the suggested substrate.

## Output formats

- **`json`** — raw `DriftHit` objects, machine-readable
- **`md`** — human-readable summary table
- **`gh-issues`** — Markdown formatted for `gh issue create --body-file -`

## Cron integration

The canonical cron recipe lives in
[`ops/heavy-runner-cron/INSTALL.md`](../../ops/heavy-runner-cron/INSTALL.md)
(scheduled for first run on **2026-06-23 09:00 PDT** on the heavy-runner).
This tool does **not** itself post to GitHub; it produces an issue-ready
Markdown render for the consumer (`phenotype-org-audits`) to file.

## Exit codes

- **0** — no drift hits
- **1** — scan error
- **2** — drift hits found (CI can fail on this)

## Schema

The drift detector implements the **substrate-extraction signal** from
[ADR-023 Rule 3](AGENTS.md). The scoring rubric is documented in
`pheno_drift_detector.py:82-87` (`W_CAPABILITY`, `W_PORT_MATCH`,
`W_ADAPTER_MATCH`, `W_TEST_MATCH`, `DRIFT_THRESHOLD`).

The 71-pillar framework is described in
`findings/71-pillar-2026-06-17-schema.md`. This tool is a **companion to
the framework**, not a pillar of it (the pillars are L1–L71).

## Related tools

- [`pheno-predict`](https://github.com/KooshaPari/pheno-predict) — companion
  similar-code scanner.
- [`pheno-framework-lint`](https://github.com/KooshaPari/pheno-framework-lint) —
  companion tier-convention enforcer.

## License

MIT — see [`LICENSE`](LICENSE).
