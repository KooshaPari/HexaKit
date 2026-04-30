# AGENTS.md - repos shelf agent rules

## Shelf Identity

This is the `repos` shelf for `CodeProjects/Phenotype/organizational-shelf/repos`.
Each top-level directory is a separate project repository.

**Never treat this shelf as a single project.** Always identify which project
is being worked on before taking action.

## Agent Rules

- Use the target project's `README.md` and `CLAUDE.md` as the primary truth surfaces.
- Run commands from inside the project directory, not the shelf root.
- Treat shelf-level docs as navigation and governance, not implementation guidance.
- Do not overwrite project-specific instructions with shelf defaults.

## Project Navigation

1. If the user names a project, use it.
2. If a path mentions a project directory, use that project.
3. If you are unsure, inspect the shelf root and choose the smallest relevant project.
4. Verify with `pwd` after `cd` into the project.

## Work Boundaries

- Shelf-level work: navigation, governance, cross-project audits, and repo organization.
- Project-level work: code changes, tests, docs, and project-specific workflows.
- Use the project's own `AGENTS.md` or `CLAUDE.md` if present.

## Quick Reference

- Find projects: use `README.md`.
- Shelf governance: use `GOVERNANCE.md`.
- Project docs: use the target project directory.
- Avoid assuming old shelf catalogs exist.
