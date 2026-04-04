# HexaKit — Phenotype Repos Shelf Specification

**Version:** 1.0  
**Status:** Active  
**Updated:** 2026-04-04

---

## Overview

HexaKit is the **repos shelf** — an organizational layer above individual projects containing ~30 independent git repositories under `CodeProjects/Phenotype/repos`. Think of it like `~/code/` or `/opt/` — a directory containing related but independent repositories where each project is a standalone git repo.

---

## Mission

Provide a production-grade, local-first development platform that enables AI-augmented software engineering with verifiable governance, hash-chained audit trails, and policy-driven quality gates across a polyglot ecosystem.

---

## Project Taxonomy

### Foundation Layer (Shared Infrastructure)

| Project | Language | Purpose |
|---------|----------|---------|
| `crates/phenotype-*` | Rust | Infrastructure crates (event-sourcing, cache, policy, FSM, error, health) |
| `agileplus-plugin-*` | Rust | Plugin adapters (core, git, sqlite) |

### Application Layer (End-User Products)

| Project | Language | Purpose |
|---------|----------|---------|
| `agileplus/` | Rust + Python | Spec-driven development engine with MCP server |
| `platforms/thegent/` | Go + Python + TS | Agent execution platform with MCP SDKs |
| `heliosCLI/` | Rust | CLI agent harness for Claude Code/Codex |
| `phenoSDK/` | Python | Phenotype SDK for Python integrations |

### Template Layer (Project Scaffolding)

| Project | Purpose |
|---------|---------|
| `kits/` | Language kits: HexaPy, HexaGo, HexaType, hexagon-rs, hexagon-ts, hexagon-python |
| `templates/` | Project templates: Python, Rust, TypeScript, Go, Zig, Kotlin, Swift, Mojo, Elixir |
| `template-*` | Domain templates: service-api, webapp, program-ops |

### Harness Layer (Agent Integration)

| Project | Purpose |
|---------|---------|
| `harnesses/CLAUDE-CODE.md` | Claude Code integration guide |
| `harnesses/CODEX.md` | Codex integration guide |
| `harnesses/CURSOR.md` | Cursor integration guide |
| `forgecode-fork/` | Forgecode fork for AI coding agents |

### Documentation Layer

| Path | Purpose |
|------|---------|
| `docs/` | VitePress documentation site |
| `kitty-specs/` | Feature specifications (27 specs) |
| `worklogs/` | Agent worklogs |

---

## Architecture

### Hexagonal Architecture

All Rust crates follow hexagonal (ports and adapters) architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Core                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Domain Logic                        │   │
│  │  (Business rules, state machines, event sourcing)   │   │
│  └─────────────────────────────────────────────────────┘   │
│                            │                                │
│  ┌─────────────────────────┼─────────────────────────┐   │
│  │              Port Traits (Interfaces)              │   │
│  │  StoragePort │ VcsPort │ AgentPort │ Observability │   │
│  └─────────────────────────┼─────────────────────────┘   │
└────────────────────────────┼────────────────────────────────┘
                             │
┌────────────────────────────┼────────────────────────────────┐
│               Adapter Implementations                       │
│  agileplus-sqlite │ agileplus-git │ agileplus-github │ ... │
└─────────────────────────────────────────────────────────────┘
```

### Key Architectural Decisions

| ADR | Title | Impact |
|-----|-------|--------|
| ADR-001 | Rust Workspace Monorepo with 22 Crates | Foundation for AgilePlus |
| ADR-002 | Hexagonal Architecture with Port/Adapter | All phenotype-* crates |
| ADR-003 | SQLite as Local-First Storage | Zero-dependency deployment |
| ADR-004 | SHA-256 Hash-Chained Audit Log | Tamper-evident governance |
| ADR-005 | gRPC + Protobuf for Services | Type-safe inter-service communication |
| ADR-006 | NATS JetStream for Event Bus | Decoupled event delivery |
| ADR-007 | External Git-Sourced Plugin Crates | Runtime extensibility |
| ADR-008 | Python MCP Server | AI agent protocol integration |

### Dependency Graph

```
phenotype-infrakit (Foundation)
  ├─ ADR-001: Workspace structure
  ├─ ADR-002: Hexagonal architecture
  ├─ ADR-003: SQLite local-first
  └─ ADR-004: Hash-chained audit

AgilePlus (extends phenotype-infrakit)
  ├─ ADR-001 → extends phenotype-infrakit ADR-001
  └─ ADR-002 → extends phenotype-infrakit ADR-002

platforms/thegent (builds on phenotype-infrakit + AgilePlus)
  ├─ ADR-001: Agent platform (depends on ADR-002)
  └─ ADR-002: MCP SDKs (depends on ADR-005)

heliosCLI (depends on all three)
  ├─ ADR-001: Harness (depends on thegent ADR-001)
  └─ ADR-002: Sandboxing (depends on thegent ADR-006)
```

---

## Quality Standards

### Code Quality Gates

| Language | Linter | Type Checker | Test Framework |
|----------|--------|-------------|----------------|
| Rust | `cargo clippy -- -D warnings` | `cargo check` | `cargo test` |
| TypeScript | `oxlint`, `eslint` | `tsc --noEmit` | `bun test` |
| Python | `ruff check` | `pyright` | `pytest` |
| Go | `golangci-lint` | `go vet` | `go test` |

### File Size Limits

- **Soft limit:** 350 lines per source file
- **Hard limit:** 500 lines per source file
- **Rationale:** Enforces decomposition, improves reviewability

### Test Traceability

All tests MUST reference a Functional Requirement:

```rust
// Traces to: FR-XXX-NNN
#[test]
fn test_feature_name() {
    // Test body
}
```

---

## Documentation Structure

### Root Level Specs

| File | Purpose |
|------|---------|
| `SPEC.md` | This file — shelf specification |
| `SOTA.md` | State-of-the-art research |
| `GOVERNANCE.md` | Shelf governance policies |
| `AGENTS.md` | Agent interaction rules |
| `ADR.md` | Architecture decision records |
| `USER_JOURNEYS.md` | User workflow definitions |
| `PLAN.md` | Implementation plans |
| `FUNCTIONAL_REQUIREMENTS.md` | Functional requirements |
| `SPECS_REGISTRY.md` | Spec tracking index |
| `PLAN_REGISTRY.md` | Plan tracking index |

### VitePress Documentation

| Section | Path | Content |
|---------|------|---------|
| Guide | `docs/guide/` | How-to guides (11 guides) |
| Reference | `docs/reference/` | Quick references (100+ docs) |
| Architecture | `docs/architecture/` | Architecture docs |
| ADRs | `docs/adr/` | Architecture decisions |
| Adoption | `docs/adoption/` | Crate adoption guides |
| Governance | `docs/governance/` | Governance policies |
| Research | `docs/research/` | Research findings |
| Worklogs | `docs/worklogs/` | Agent worklogs (80+) |

---

## Getting Started

### For New Projects

```bash
# Clone the shelf
git clone https://github.com/KooshaPari/phenotype-infrakit

# List all projects
cat projects/INDEX.md

# Create a worktree
git worktree add .worktrees/my-feature -b my-feature
cd .worktrees/my-feature
```

### For Existing Projects

```bash
# Navigate to project
cd <project-name>

# Read project rules
cat CLAUDE.md
cat AGENTS.md

# Run quality checks
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

---

## Key Files by Project

| Project | README | CLAUDE.md | AGENTS.md | SPEC.md |
|---------|--------|-----------|-----------|---------|
| HexaKit (shelf) | ✅ | ✅ | ⚠️ conflicted | ✅ (this) |
| AgilePlus | ✅ | ✅ | ✅ | ✅ |
| platforms/thegent | ✅ | ✅ | ✅ | ✅ |
| heliosCLI | ✅ | ✅ | ✅ | ✅ |
| phenotype-infrakit | ✅ | ✅ | ✅ | ✅ |

---

## Governance

### Agent Authority Levels

| Agent | Can Edit | Can Commit | Can Push | Can Merge |
|-------|----------|------------|----------|-----------|
| Forge | Any file | Any branch | Own worktrees | No |
| Muse | Comments only | No | No | No |
| Sage | Any file | Any branch | Own worktrees | No |
| Helios | Test/config | Test branches | No | No |

### Decision Making

| Decision Type | Process |
|---------------|---------|
| New project | Owner creates + names, agent documents |
| Architecture (cross-project) | Owner decides, agent researches |
| Architecture (per-project) | Project owner decides |
| Dependency conflicts | Agent proposes options, owner chooses |
| PR merge | Owner reviews + merges |

---

## Related Documents

| Document | Location |
|----------|----------|
| ADR Registry | `ADR_REGISTRY.md` |
| Specs Registry | `SPECS_REGISTRY.md` |
| Plan Registry | `PLAN_REGISTRY.md` |
| User Journeys | `USER_JOURNEYS.md` |
| Contributing | `CONTRIBUTING.md` |
| Security Policy | `SECURITY.md` |

---

**Spec Owner:** Platform Architect  
**Last Updated:** 2026-04-04
