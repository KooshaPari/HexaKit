# CLAUDE.md - repos shelf root

## Identity

This is the `repos` shelf for `CodeProjects/Phenotype/organizational-shelf/repos`.
It is a polyrepo: each top-level directory is an independent project repository.

**NOT AgilePlus.** AgilePlus is one of many projects in this shelf.
Use the target project's `README.md` and `CLAUDE.md` for project-specific truth.

## Shelf Rules

- Identify the project before taking action.
- Do not treat the shelf as a single project.
- Keep work inside the target project directory.
- Use shelf docs for navigation and project docs for implementation details.
- Prefer the project `README.md` and `CLAUDE.md` over shelf-wide guesses.

## Structure

```text
repos/
├── .worktrees/
├── .archive/
├── docs/
├── governance/
├── scripts/
├── projects/
└── [projects]
```

## Quick Reference

| What you need | Where to look |
|---------------|---------------|
| Find a project | `README.md` |
| Agent rules | `AGENTS.md` |
| Shelf governance | `GOVERNANCE.md` |
| Project-specific instructions | target project `CLAUDE.md` |
| Project-specific agent rules | target project `AGENTS.md` |

## Working Pattern

1. Identify the project.
2. `cd` into the project directory.
3. Read that project's docs.
4. Make the change.
5. Verify from inside the project.

## Guidance

- Shelf-level work is for cross-project navigation, governance, and audits.
- Project-level work belongs in the project directory.
- If a project has its own agent instructions, follow them first.
