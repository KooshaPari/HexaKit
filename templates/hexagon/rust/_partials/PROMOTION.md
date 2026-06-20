# PROMOTION.md — absorbed 2026-06-20 from
# KooshaPari/pheno-cargo-template/template/PROMOTION.md.
#
# Tier-promotion ladder for scaffolded crates.

## Tier ladder

| Tier | Description | Promotion criteria |
|------|-------------|--------------------|
| `pheno-scaffold` | 0–1 day old, scaffold-only | None (entry tier) |
| `pheno-*-lib` | First feature landed | Tests green; 80% lib coverage |
| `pheno-incubating` | Used by 2+ dependent crates | All quality gates green |
| `pheno-stable` | API frozen for 1 minor cycle | Dependents stable; API review passed |
| `pheno-core` | First-party status | Phenotype core team sign-off |

## Promotion mechanics

1. Edit `[package.metadata.phenotype].tier` in `Cargo.toml`.
2. Mirror the change in `.framework-lint.yaml`.
3. Open a PR labelled `promote:tier/<from>→<to>`.
4. CI validates both files agree; auto-merge if the target tier is
   `pheno-stable` or below.

See `.framework-lint.yaml` for the lint rule that enforces the
single source of truth.