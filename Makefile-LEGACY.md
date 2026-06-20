# Makefile → Taskfile Migration Guide

> **Status:** Makefile is **deprecated** for this repository. New automation
> **must** be added to `Taskfile.yml` first, then mirrored to `justfile`.
> A Makefile is intentionally **not** present in the template.

## Why Makefile is deprecated

- **No cross-platform path safety.** Make has no first-class understanding
  of `$(TASKFILE_DIR)`, Cargo's `workspace` flag, or feature flags, so
  every target ends up duplicating the same `cargo ... --all-targets
  --all-features` string.
- **No recipe composition.** `make ci` cannot depend on `fmt` + `lint`
  + `test` + `deny` cleanly without re-running recipes in sub-shells,
  which silently swallows intermediate exit codes.
- **No native parallel / dry-run support.** `task --dry <recipe>` and
  `task --parallel` are first-class; `make -n` does not always reflect
  variables expanded by `.mk` includes.
- **No typed variables or timeouts.** PlayCua L2 #21 surfaced the
  precedent: `cargo check` did not complete within the L1-bounded
  300s run, so every long-running recipe needs an explicit
  `timeout:`. Taskfile supports it natively; Make needs the
  `timeout(1)` binary bolted on.
- **CI portability.** Many CI runners (GitHub Actions, Buildkite,
  CircleCI) ship `go-task` and `just`; not all of them ship GNU make
  with the same default flags.

## Target mapping

The table below is the **single source of truth** for cross-runner
recipe mapping. If you change a recipe name in `Taskfile.yml`, update
this table and `justfile` in the same commit.

| Legacy `make` target | Canonical `task` target | `just` recipe | What it runs |
| --- | --- | --- | --- |
| `make test`        | `task test`        | `just test`        | `cargo test --workspace` (10m timeout) |
| `make build`       | `task build`       | `just build`       | `cargo build --workspace` (10m timeout) |
| `make lint`        | `task lint`        | `just lint`        | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| `make fmt`         | `task fmt`         | `just fmt`         | `cargo fmt --all -- --check` |
| `make fmt-fix`     | `task fmt-fix`     | `just fmt-fix`     | `cargo fmt --all` (apply) |
| `make deny`        | `task deny`        | `just deny`        | `cargo deny check` (advisories, bans, licenses, sources) |
| `make audit`       | `task audit`       | `just audit`       | `cargo audit` (RustSec advisory database) |
| `make ci`          | `task ci`          | `just ci`          | `lint + fmt + test + build + deny` |
| `make hygiene`     | `task hygiene`     | `just hygiene`     | `fmt + lint + deny + audit` |
| `make release`     | `task release`     | `just release`     | `fmt + lint + test + build + deny + audit` + `cargo package --no-verify --list` |

## Migration steps

1. **Install Task.** `brew install go-task/tap/go-task` (macOS) or
   follow <https://taskfile.dev/installation/> for Linux/Windows.
2. **Replace `make <target>` with `task <target>`** in scripts,
   documentation, and CI workflows.
3. **Update CI workflows** to call `task ci` (or the per-recipe
   targets your CI matrix needs). The Taskfile's `ci` recipe is
   composed of `lint + fmt + test + build + deny`; do not bypass it
   with ad-hoc `cargo` invocations.
4. **Remove project-specific Makefile rules** as their Taskfile
   equivalents land. Do **not** port them: the `Taskfile.yml`
   recipes use `{{.WORKSPACE_FLAG}}` and the `LONG_TIMEOUT` var
   rather than hard-coded flags, so the Makefile version would not
   be a 1:1 drop-in anyway.
5. **Update PR/MR templates** and `CONTRIBUTING.md` to reference
   `task` and `just` recipes; remove `make` mentions.

## Policy

- **Do not** add new Makefile targets. There is intentionally no
  `Makefile` in this template.
- **Do** add or update `Taskfile.yml` first, then mirror the same
  behavior in `justfile` when a `just` recipe is useful for
  developers. Both files must be updated in the same commit.
- **Do** keep this mapping table in sync. If a new recipe is added
  to `Taskfile.yml`, add a row here in the same commit.

## Reference

- PlayCua L2 #21 canonical shape: `worklogs/l2-21-playcua-taskfile-2026-06-11.json`
- Taskfile spec: <https://taskfile.dev/usage/>
- `just` manual: <https://just.systems/man/en/>
