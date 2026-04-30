# HexaKit — Phenotype Repos Shelf Specification

__Version:__ 1.0  
__Status:__ Active  
__Updated:__ 2026-04-04

---

## Table of Contents

1. [Overview](#overview)
2. [Mission](#mission)
3. [Tenets](#tenets)
4. [Project Taxonomy](#project-taxonomy)
5. [Architecture](#architecture)
6. [Quality Standards](#quality-standards)
7. [Documentation Structure](#documentation-structure)
8. [Getting Started](#getting-started)
9. [Key Files by Project](#key-files-by-project)
10. [Governance](#governance)
11. [Technology Stack](#technology-stack)
12. [Development Workflow](#development-workflow)
13. [Testing Strategy](#testing-strategy)
14. [Deployment](#deployment)
15. [Security](#security)
16. [Observability](#observability)
17. [Integration Patterns](#integration-patterns)
18. [Migration Guidelines](#migration-guidelines)
19. [Troubleshooting](#troubleshooting)
20. [Advanced Topics](#advanced-topics)
21. [Performance Considerations](#performance-considerations)
22. [Scalability Planning](#scalability-planning)
23. [Disaster Recovery](#disaster-recovery)
24. [Related Documents](#related-documents)

---

## Overview

HexaKit is the __repos shelf__ — an organizational layer above individual projects containing ~30 independent git repositories under `CodeProjects/Phenotype/repos`. Think of it like `~/code/` or `/opt/` — a directory containing related but independent repositories where each project is a standalone git repo.

__Key Characteristics__

- __Polyrepo:__ ~30 independent repositories
- __Polyglot:__ Rust, Python, TypeScript, Go, Zig, and more
- __Local-first:__ Full offline operation capability
- __AI-native:__ MCP protocol integration for agents
- __Governance-verified:__ Hash-chained audit trails

__Scope Boundaries__

| In Scope | Out of Scope |
|----------|--------------|
| Repository organization | Production deployment orchestration |
| Cross-project patterns | End-user application features |
| Quality gate definitions | Specific project implementations |
| MCP server contracts | External SaaS integrations |
| Documentation standards | Marketing materials |

---

## Mission

Provide a production-grade, local-first development platform that enables AI-augmented software engineering with verifiable governance, hash-chained audit trails, and policy-driven quality gates across a polyglot ecosystem.

__Goals__

1. Enable rapid, AI-assisted development across multiple languages
2. Maintain tamper-evident audit trails for all work
3. Ensure consistent quality across all projects
4. Support local-first operation without cloud dependencies
5. Scale to 100+ repositories without degradation
6. Provide seamless human-AI collaboration workflows

__Non-Goals__

1. Replace existing version control systems
2. Mandate specific development methodologies
3. Provide cloud-hosted CI/CD (local-first priority)
4. Support proprietary/closed-source agent integrations

---

## Tenets

Unless you know better ones:

1. __Local-First__

All tooling must work without internet connectivity. Cloud services are optional enhancements, not requirements. Data ownership remains with the developer. This includes:
- Local SQLite storage as primary persistence
- Git-based synchronization optional
- No external API dependencies for core functionality
- Full offline spec management and planning

2. __AI-Native__

Interfaces designed for AI agents first, human developers second. Structured, type-safe protocols over natural language parsing. Specifically:
- MCP (Model Context Protocol) as primary interface
- JSON Schema for all tool definitions
- URI-addressable resources
- Structured output requirements for all agents

3. __Tamper-Evident__

All governance actions recorded in hash-chained audit logs. Detection of any modification, deletion, or reordering. Implementation:
- SHA-256 hash chains for all events
- Append-only event store
- Git-based backup with cryptographic verification
- Regular chain integrity checks

4. __Polyglot Consistency__

Same patterns across all languages. Hexagonal architecture, event sourcing, and MCP integration regardless of implementation language. Patterns include:
- Ports and adapters in all languages
- Consistent error handling patterns
- Uniform testing strategies
- Shared vocabulary across languages

5. __Minimal Coupling__

Projects depend only on shared foundation crates. Circular dependencies prohibited. Optional integrations via feature flags. This means:
- Foundation crates have no external dependencies
- Applications depend only on foundation
- Optional features compile only when enabled
- Clear dependency direction (foundation → app)

---

## Project Taxonomy

### Foundation Layer (Shared Infrastructure)

The foundation layer provides infrastructure capabilities used by all other projects. These are Rust crates implementing hexagonal architecture patterns.

| Project | Language | Purpose | Crates | Status |
|---------|----------|---------|--------|--------|
| `phenotype-infrakit` | Rust | Infrastructure crates | 8 | Active |
| `agileplus-plugin-*` | Rust | Plugin adapters | 3 | Active |

__Foundation Crates__

| Crate | Purpose | Key Features |
|-------|---------|--------------|
| `phenotype-core` | Core abstractions | Entity, Aggregate, Repository traits |
| `phenotype-events` | Event sourcing | EventStore, Projection, EventBus |
| `phenotype-cache` | Caching layer | Port trait, TTL, LRU adapters |
| `phenotype-policy` | Policy engine | Rule engine, Policy definition DSL |
| `phenotype-fsm` | State machines | StateMachine trait, transitions |
| `phenotype-error` | Error handling | Error types, conversions, reporting |
| `phenotype-health` | Health checks | HealthCheck trait, aggregation |
| `phenotype-storage` | Storage ports | StoragePort trait, adapters |

__Foundation Architecture__

```
phenotype-infrakit/
├── crates/
│   ├── phenotype-core/          # Domain abstractions
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── entity.rs
│   │   │   ├── aggregate.rs
│   │   │   └── repository.rs
│   │   └── Cargo.toml
│   ├── phenotype-events/        # Event sourcing
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── event.rs
│   │   │   ├── store.rs
│   │   │   └── projection.rs
│   │   └── Cargo.toml
│   ├── phenotype-policy/        # Policy engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── policy.rs
│   │   │   ├── rule.rs
│   │   │   └── engine.rs
│   │   └── Cargo.toml
│   └── ...
├── Cargo.toml                   # Workspace definition
└── README.md
```

### Application Layer (End-User Products)

Applications build on the foundation layer to provide end-user capabilities. Each is a complete product with its own documentation, tests, and release cycle.

| Project | Language | Purpose | Status |
|---------|----------|---------|--------|
| `agileplus` | Rust + Python | Spec-driven development engine with MCP server | Active |
| `platforms/thegent` | Go + Python + TS | Agent execution platform with MCP SDKs | Active |
| `heliosCLI` | Rust | CLI agent harness for Claude Code/Codex | Active |
| `phenoSDK` | Python | Phenotype SDK for Python integrations | Active |

__agileplus Components__

| Component | Language | Purpose |
|-----------|----------|---------|
| Core engine | Rust | Spec management, planning, validation |
| MCP server | Python | AI agent integration |
| CLI | Rust | User interface |
| SQLite adapter | Rust | Local storage |
| Git adapter | Rust | VCS operations |

__thegent Components__

| Component | Language | Purpose |
|-----------|----------|---------|
| Agent platform | Go | Agent execution, sandboxing |
| MCP SDK | Python | Python agent SDK |
| MCP SDK | TypeScript | TypeScript agent SDK |
| Harness | Go | Runtime environment |

### Template Layer (Project Scaffolding)

Kits provide project templates following HexaKit standards. Each kit generates a project structure with correct documentation, CI setup, and tooling configuration.

| Project | Purpose | Languages | Status |
|---------|---------|-----------|--------|
| `template-rust` | Rust service/CLI/library | Rust | Active |
| `template-typescript` | TypeScript webapp | TypeScript | Active |
| `template-python` | Python package | Python | Active |
| `template-go` | Go microservice | Go | Active |
| `template-lang-rust` | Rust language extensions | Rust | Active |
| `template-lang-typescript` | TypeScript language extensions | TypeScript | Active |
| `template-lang-python` | Python language extensions | Python | Active |
| `template-lang-go` | Go language extensions | Go | Active |
| `template-domain-service-api` | Multi-language API service | Mixed | Active |
| `template-domain-webapp` | Multi-language webapp | Mixed | Active |
| `template-program-ops` | Operations tooling | Mixed | Active |

__Template Features__

| Feature | Rust | TypeScript | Python | Go | Multi |
|---------|------|------------|--------|-----|-------|
| Hexagonal structure | ✓ | ✓ | ✓ | ✓ | ✓ |
| MCP integration | ✓ | ✓ | ✓ | ✓ | ✓ |
| SQLite storage | ✓ | Optional | ✓ | - | ✓ |
| Event sourcing | ✓ | - | ✓ | - | ✓ |
| Policy engine | ✓ | ✓ | ✓ | ✓ | ✓ |
| Pre-commit hooks | ✓ | ✓ | ✓ | ✓ | ✓ |
| CI workflow | ✓ | ✓ | ✓ | ✓ | ✓ |
| Documentation site | - | - | - | - | ✓ |

__Template Architecture__

```
template-<type>/
├── .template.yml              # Template manifest with variables
├── Cargo.toml.hbs              # Handlebars template files
├── src/
│   └── main.rs.hbs
├── .github/
│   └── workflows/
│       └── ci.yml.hbs
└── README.md.hbs
```

__Template Manifest (`.template.yml`)__

Templates use a declarative manifest for variable definition, conditional includes, and post-generation validation:

```yaml
name: rust-service
variables:
  - name: project_name
    prompt: "Project name"
    required: true
conditionals:
  - when: features contains "http"
    include: [src/adapters/http.rs]
post_generation:
  - command: cargo build
  - command: cargo test
validation:
  - type: cargo_check
  - type: cargo_clippy
```

__Scaffolding Documentation__

| Document | Location | Purpose | Lines |
|----------|----------|---------|-------|
| SOTA.md | `docs/research/SOTA.md` | Scaffolding frameworks landscape | 1100+ |
| Scaffolding Journeys | `docs/SCAFFOLDING_JOURNEYS.md` | Template usage workflows | 1000+ | |

### Harness Layer (Agent Integration)

Harnesses provide integration guides for AI agents working with HexaKit projects.

| Project | Purpose | Status |
|---------|---------|--------|
| `harnesses/CLAUDE-CODE.md` | Claude Code integration guide | Active |
| `harnesses/CODEX.md` | Codex integration guide | Active |
| `harnesses/CURSOR.md` | Cursor integration guide | Active |
| `forgecode-fork` | Forgecode fork for AI coding agents | Active |

### Documentation Layer

Documentation spans multiple locations:

| Path | Purpose | Format |
|------|---------|--------|
| `docs/` | VitePress documentation site | Markdown |
| `kitty-specs/` | Feature specifications (27 specs) | Markdown + JSON |
| `worklogs/` | Agent worklogs | Markdown |
| `adr/` | Architecture decision records | Markdown |
| `plans/` | Implementation plans | Markdown |

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

__Port Definitions Example__

```rust
// src/ports/storage.rs

pub trait StoragePort: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError>;
    fn delete(&self, key: &str) -> Result<(), StorageError>;
    fn list(&self, prefix: &str) -> Result<Vec<String>, StorageError>;
    fn exists(&self, key: &str) -> Result<bool, StorageError>;
}

pub enum StorageError {
    NotFound(String),
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
}
```

```rust
// src/ports/vcs.rs

pub trait VcsPort: Send + Sync {
    fn clone(&self, url: &str, path: &Path) -> Result<(), VcsError>;
    fn commit(&self, path: &Path, message: &str) -> Result<String, VcsError>;
    fn diff(&self, path: &Path) -> Result<Vec<Change>, VcsError>;
    fn status(&self, path: &Path) -> Result<Status, VcsError>;
    fn log(&self, path: &Path, count: usize) -> Result<Vec<Commit>, VcsError>;
}
```

```rust
// src/ports/agent.rs

pub trait AgentPort: Send + Sync {
    fn dispatch(&self, task: Task) -> Result<TaskId, AgentError>;
    fn status(&self, task_id: TaskId) -> Result<TaskStatus, AgentError>;
    fn cancel(&self, task_id: TaskId) -> Result<(), AgentError>;
    fn results(&self, task_id: TaskId) -> Result<TaskResults, AgentError>;
}
```

### Key Architectural Decisions

| ADR | Title | Impact | Status |
|-----|-------|--------|--------|
| [ADR-001](./ADR-001.md) | HexaKit Shelf Organization with Git Worktrees | All projects | Accepted |
| [ADR-002](./ADR-002.md) | AI-Native MCP Protocol Integration | AI agent interfaces | Accepted |
| [ADR-003](./ADR-003.md) | Hierarchical Multi-Agent Organization | Agent workflows | Accepted |
| ADR-004 | Rust Workspace Monorepo with 22 Crates | Foundation for AgilePlus | Implemented |
| ADR-005 | Hexagonal Architecture with Port/Adapter | All phenotype-* crates | Implemented |
| ADR-006 | SQLite as Local-First Storage | Zero-dependency deployment | Implemented |
| ADR-007 | SHA-256 Hash-Chained Audit Log | Tamper-evident governance | Implemented |
| ADR-008 | gRPC + Protobuf for Services | Type-safe inter-service communication | Implemented |
| ADR-009 | NATS JetStream for Event Bus | Decoupled event delivery | Planned |
| ADR-010 | External Git-Sourced Plugin Crates | Runtime extensibility | Planned |
| ADR-011 | Python MCP Server | AI agent protocol integration | Implemented |

### Dependency Graph

```
                    ┌─────────────────┐
                    │     User        │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
┌───────▼────────┐  ┌────────▼────────┐  ┌───────▼────────┐
│   heliosCLI    │  │    thegent      │  │   phenoSDK     │
│   (CLI tool)   │  │ (Agent platform)│  │   (Python SDK) │
└───────┬────────┘  └────────┬────────┘  └────────────────┘
        │                    │
        │         ┌──────────┴──────────┐
        │         │                     │
┌───────▼─────────▼────────┐  ┌──────────▼─────────┐
│      agileplus          │  │  External tools    │
│ (Spec engine + MCP)     │  │  (Claude, Codex)   │
└───────────┬─────────────┘  └────────────────────┘
            │
┌───────────▼──────────────────────────┐
│      phenotype-infrakit              │
│  (Foundation crates)                 │
│  • phenotype-core                    │
│  • phenotype-events                  │
│  • phenotype-policy                  │
│  • phenotype-storage                 │
│  • ...                               │
└──────────────────────────────────────┘
```

### Cross-Project Communication

__gRPC Services__

```protobuf
syntax = "proto3";

package phenotype.agileplus.v1;

service SpecService {
    rpc GetSpec(GetSpecRequest) returns (GetSpecResponse);
    rpc ListSpecs(ListSpecsRequest) returns (ListSpecsResponse);
    rpc CreateSpec(CreateSpecRequest) returns (CreateSpecResponse);
    rpc UpdateSpec(UpdateSpecRequest) returns (UpdateSpecResponse);
    rpc DeleteSpec(DeleteSpecRequest) returns (DeleteSpecResponse);
}

message GetSpecRequest {
    string spec_id = 1;
}

message GetSpecResponse {
    Spec spec = 1;
}

message Spec {
    string id = 1;
    string title = 2;
    string description = 3;
    SpecStatus status = 4;
    repeated WorkPackage work_packages = 5;
}

enum SpecStatus {
    SPEC_STATUS_UNSPECIFIED = 0;
    SPEC_STATUS_PLANNED = 1;
    SPEC_STATUS_IMPLEMENTING = 2;
    SPEC_STATUS_REVIEWING = 3;
    SPEC_STATUS_DONE = 4;
}
```

```protobuf
service AgentService {
    rpc DispatchTask(DispatchTaskRequest) returns (DispatchTaskResponse);
    rpc GetTaskStatus(GetTaskStatusRequest) returns (GetTaskStatusResponse);
    rpc CancelTask(CancelTaskRequest) returns (CancelTaskResponse);
    rpc StreamTaskLogs(StreamTaskLogsRequest) returns (stream LogEntry);
}
```

__MCP Resource URI Patterns__

| Resource Type | URI Pattern | Example |
|--------------|-------------|---------|
| Feature spec | `kitty-spec://features/{id}` | `kitty-spec://features/FEATURE-001` |
| Plan | `kitty-spec://plans/{id}` | `kitty-spec://plans/PLAN-001` |
| Worklog | `worklog://{category}/{date}-{topic}` | `worklog://research/2026-04-04-mcp` |
| Source file | `codebase://{project}/{path}` | `codebase://agileplus/src/main.rs` |
| Evidence | `evidence://{spec_id}/{type}` | `evidence://FEATURE-001/tests` |

---

## Quality Standards

### Code Quality Gates

| Language | Linter | Type Checker | Test Framework | Formatter |
|----------|--------|-------------|----------------|-----------|
| Rust | `clippy` | `cargo check` | `cargo test` | `rustfmt` |
| TypeScript | `oxlint`, `eslint` | `tsc --noEmit` | `bun test` | `prettier` |
| Python | `ruff check` | `pyright` | `pytest` | `ruff format` |
| Go | `golangci-lint` | `go vet` | `go test` | `gofmt` |

__Rust Quality Gates__

```bash
# Run all checks
cargo test --workspace                                    # Unit + integration tests
cargo clippy --workspace -- -D warnings                 # Linting
cargo check --workspace                                 # Type checking
cargo fmt -- --check                                    # Format checking
cargo audit                                             # Security audit
cargo deny check                                        # License check
```

__TypeScript Quality Gates__

```bash
# Run all checks
bun test                                                # Unit tests
oxlint .                                                # Linting
tsc --noEmit                                            # Type checking
prettier --check .                                      # Format checking
```

__Python Quality Gates__

```bash
# Run all checks
pytest                                                  # Unit tests
ruff check .                                            # Linting
pyright                                                 # Type checking
ruff format --check .                                   # Format checking
```

### File Size Limits

| Limit | Lines | Enforcement | Rationale |
|-------|-------|-------------|-----------|
| Soft limit | 350 | Warning in CI | Encourages decomposition |
| Hard limit | 500 | Block merge | Forces separation of concerns |
| Exception | >500 | ADR required | Documented justification |

__Why File Size Limits Matter__

1. __Reviewability:__ Files under 500 lines can be reviewed in a single sitting
2. __Comprehension:__ Smaller files are easier to understand
3. __Testing:__ Focused files enable targeted tests
4. __Refactoring:__ Smaller units are easier to refactor
5. __Parallel work:__ Smaller files reduce merge conflicts

### Test Traceability

All tests MUST reference a Functional Requirement:

```rust
// Traces to: FR-001-042
#[test]
fn test_auth_token_refresh() {
    // Given: Valid but expired token
    let token = create_expired_token();
    
    // When: Attempting refresh
    let result = auth.refresh_token(&token);
    
    // Then: New token issued
    assert!(result.is_ok());
    assert!(result.unwrap().is_valid());
}
```

__FR Numbering Convention__

| Segment | Meaning | Example |
|---------|---------|---------|
| FR | Functional Requirement | FR |
| XXX | Spec ID (3 digits) | 001 |
| NNN | Requirement number | 042 |

### Policy Enforcement

```yaml
# .policy.yml
version: "1.0"

policies:
  - id: file-size
    applies_to: ["**/*.rs", "**/*.py", "**/*.ts"]
    rules:
      - id: soft-limit
        severity: warning
        condition:
          type: file_size
          max_lines: 350
        message: "File exceeds soft limit of 350 lines - consider decomposition"
      
      - id: hard-limit
        severity: error
        condition:
          type: file_size
          max_lines: 500
        message: "File exceeds hard limit of 500 lines - must decompose"

  - id: test-traceability
    applies_to: ["**/*.rs"]
    rules:
      - id: fr-trace
        severity: error
        condition:
          type: fr_traceability
        message: "Tests must include FR trace comment (e.g., // Traces to: FR-XXX-NNN)"

  - id: documentation
    applies_to: ["**/src/**/*.rs"]
    rules:
      - id: public-api-docs
        severity: warning
        condition:
          type: documentation_coverage
          min_percent: 80
        message: "Public API must have documentation coverage >= 80%"
```

---

## Documentation Structure

### Root Level Specs

| File | Purpose | Lines Target | Format |
|------|---------|--------------|--------|
| `SPEC.md` | This file — shelf specification | 2500+ | Markdown |
| `SOTA.md` | State-of-the-art research | 1500+ | Markdown |
| `GOVERNANCE.md` | Shelf governance policies | 500+ | Markdown |
| `AGENTS.md` | Agent interaction rules | 400+ | Markdown |
| `ADR.md` | Architecture decision records index | 300+ | Markdown |
| `USER_JOURNEYS.md` | User workflow definitions | 400+ | Markdown |
| `PLAN.md` | Implementation plans | 600+ | Markdown |
| `FUNCTIONAL_REQUIREMENTS.md` | Functional requirements | 800+ | Markdown |
| `SPECS_REGISTRY.md` | Spec tracking index | 200+ | Markdown |
| `PLAN_REGISTRY.md` | Plan tracking index | 200+ | Markdown |
| `SECURITY.md` | Security policies | 300+ | Markdown |
| `CONTRIBUTING.md` | Contribution guidelines | 400+ | Markdown |

### ADR Registry

| File | Title | Status | Date |
|------|-------|--------|------|
| `ADR-001.md` | HexaKit Shelf Organization with Git Worktrees | Accepted | 2026-04-04 |
| `ADR-002.md` | AI-Native MCP Protocol Integration | Accepted | 2026-04-04 |
| `ADR-003.md` | Hierarchical Multi-Agent Organization | Accepted | 2026-04-04 |

### VitePress Documentation

| Section | Path | Content Count | Purpose |
|---------|------|---------------|---------|
| Guide | `docs/guide/` | 11 guides | How-to tutorials |
| Reference | `docs/reference/` | 100+ docs | API documentation |
| Architecture | `docs/architecture/` | 15 docs | System design |
| ADRs | `docs/adr/` | 20+ decisions | Decision records |
| Adoption | `docs/adoption/` | 8 guides | Onboarding |
| Governance | `docs/governance/` | 12 policies | Rules and procedures |
| Research | `docs/research/` | 25 findings | Research notes |
| Worklogs | `docs/worklogs/` | 80+ logs | Agent activities |

### Documentation Standards

__File Naming Conventions__

| Type | Pattern | Example |
|------|---------|---------|
| Specification | `SPEC.md` or `*-SPEC.md` | `AUTH-SPEC.md` |
| ADR | `ADR-NNN.md` | `ADR-001.md` |
| Plan | `PLAN-NNN.md` | `PLAN-001.md` |
| Worklog | `YYYY-MM-DD-*.md` | `2026-04-04-mcp-analysis.md` |
| User Journey | `UJ-NNN-*.md` | `UJ-001-onboarding.md` |
| Functional Req | `FR-XXX-NNN.md` | `FR-001-042.md` |

__Header Format__

```markdown
# Title

__Version:__ 1.0  
__Status:__ Active  
__Updated:__ 2026-04-04

---
```

__Content Guidelines__

1. Use underline headers (`__Header__`) for major sections
2. Use sentence case for headers
3. Include tables for structured data
4. Use code blocks for examples
5. Include diagrams where helpful
6. Cross-reference related documents

---

## Getting Started

### Prerequisites

| Tool | Version | Purpose | Installation |
|------|---------|---------|--------------|
| Rust | 1.75+ | Rust projects | rustup.rs |
| Python | 3.14-dev | Python projects | astral.sh/uv |
| TypeScript | 7.0+ | TS projects | bun.sh |
| Go | 1.24+ | Go projects | go.dev |
| Git | 2.40+ | Version control | git-scm.com |
| SQLite | 3.40+ | Local storage | Built-in |

### For New Projects

```bash
# 1. Clone the shelf
git clone https://github.com/KooshaPari/phenotype-infrakit

# 2. List all projects
find . -maxdepth 1 -mindepth 1 -type d | sort

# 3. Navigate to project of interest
cd agileplus

# 4. Read project rules
cat CLAUDE.md
cat AGENTS.md

# 5. Create a worktree
git worktree add .worktrees/my-feature -b my-feature
cd .worktrees/my-feature

# 6. Run quality checks
cargo test --workspace
cargo clippy --workspace -- -D warnings
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

# Run full quality suite
task quality
```

### Worktree Workflow

```bash
# List existing worktrees
git worktree list

# Create worktree for feature
git worktree add .worktrees/feat/my-feature -b feat/my-feature

# Work in worktree
cd .worktrees/feat/my-feature
# ... make changes ...
# ... run tests ...
# ... commit ...

# Clean up when done
cd ../..
git worktree remove .worktrees/feat/my-feature
```

### First Time Setup

```bash
# 1. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Install uv (Python)
curl -LsSf https://astral.sh/uv/install.sh | sh

# 3. Install Bun (TypeScript)
curl -fsSL https://bun.sh/install | bash

# 4. Install Go (if needed)
# See: https://go.dev/doc/install

# 5. Verify installations
rustc --version
cargo --version
uv --version
bun --version
go version
```

---

## Key Files by Project

### Project Files Matrix

| Project | README | CLAUDE.md | AGENTS.md | SPEC.md |
|---------|--------|-----------|-----------|---------|
| HexaKit (shelf) | ✓ | ✓ | ⚠ | ✓ (this) |
| AgilePlus | ✓ | ✓ | ✓ | ✓ |
| platforms/thegent | ✓ | ✓ | ✓ | ✓ |
| heliosCLI | ✓ | ✓ | ✓ | ✓ |
| phenotype-infrakit | ✓ | ✓ | ✓ | ✓ |
| kits/HexaPy | ✓ | ✓ | — | — |
| kits/HexaGo | ✓ | ✓ | — | — |
| kits/HexaType | ✓ | ✓ | — | — |
| kits/hexagon-rs | ✓ | ✓ | — | — |

### File Purposes

| File | Purpose | Primary Audience | Update Frequency |
|------|---------|------------------|------------------|
| `README.md` | Project overview, quick start | Everyone | Each release |
| `CLAUDE.md` | Project-specific agent rules | AI agents (Claude) | As needed |
| `AGENTS.md` | Agent authority, workflows | All AI agents | As needed |
| `SPEC.md` | Technical specification | Developers | Major changes |
| `CONTRIBUTING.md` | Contribution guidelines | Contributors | Policy changes |
| `SECURITY.md` | Security policy | Security researchers | Security events |
| `CHANGELOG.md` | Version changes | Users | Each release |
| `LICENSE` | Legal license | Everyone | Never |

### File Templates

__README.md Template__

```markdown
# Project Name

One-line description of the project.

## Overview

Longer description of what this project does and why it exists.

## Quick Start

\`\`\`bash
# Installation
...

# Usage
...
\`\`\`

## Documentation

- [Full documentation](docs/)
- [API reference](docs/api/)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
```

__CLAUDE.md Template__

```markdown
# CLAUDE.md — Project Name

## Project Identity

- **Type:** <application|library|kit|harness>
- **Language:** <Rust|Python|TypeScript|Go|...>
- **Purpose:** <one-line description>

## Architecture

<hexagonal/monolithic/etc>

## Quality Gates

- Linter: <tool>
- Tests: <command>
- Line limit: <number>

## Agent Rules

<project-specific rules>
```

---

## Governance

### Agent Authority Levels

| Agent | Can Edit | Can Commit | Can Push | Can Merge | Scope |
|-------|----------|------------|----------|-----------|-------|
| Forge | Any file | Any branch | Own worktrees | No | All projects |
| Muse | Comments only | No | No | No | Review only |
| Sage | Any file | Any branch | Own worktrees | No | Research, docs |
| Helios | Test/config | Test branches | No | No | Testing focus |

__Authority Definitions__

- __Can Edit:__ Can modify source files, documentation, tests
- __Can Commit:__ Can create git commits
- __Can Push:__ Can push to own worktrees (not main)
- __Can Merge:__ Can merge pull requests (none - owner only)

### Decision Making

| Decision Type | Process | Authority | Documentation |
|---------------|---------|-----------|---------------|
| New project | Owner creates + names, agent documents | Owner | Add to INDEX.md |
| Architecture (cross-project) | Owner decides, agent researches | Owner | ADR required |
| Architecture (per-project) | Project owner decides | Project owner | Project ADR |
| Dependency conflicts | Agent proposes options, owner chooses | Owner | Document choice |
| PR merge | Owner reviews + merges | Owner | PR description |
| Quality gate failure | Agent fixes, owner approves | Shared | Commit messages |
| Breaking changes | Owner approves, agent implements | Owner | Migration guide |

### Governance Principles

1. __Human-in-the-loop:__ No fully autonomous deployment to production
2. __Verifiable:__ All decisions recorded in audit log
3. __Reversible:__ Any decision can be reverted with traceability
4. __Transparent:__ Governance visible to all agents via worklogs
5. __Consistent:__ Same rules applied across all projects

---

## Technology Stack

### Language Targets

| Language | Current Version | Primary | Legacy Escape | Notes |
|----------|-----------------|---------|---------------|-------|
| Rust | Edition 2024 nightly | Latest nightly | Edition 2021 | Aggressive nightly adoption |
| TypeScript | 7.0+ | TypeScript 7 native (tsgo) | tsc | Go-based compiler |
| Python | 3.14-dev | 3.14 | 3.13 | Latest features |
| Go | 1.24+ | 1.24+ | None | Latest stable |
| Bun | 1.2+ | 1.2+ | Node.js | Drop-in replacement |

### Core Dependencies

| Category | Primary | Alternatives | Selection Criteria |
|----------|---------|--------------|-------------------|
| Async runtime | Tokio | async-std | Ecosystem, performance |
| Serialization | serde | None | De facto standard |
| HTTP server | axum | actix-web, rocket | Simplicity, async |
| gRPC | tonic | None | Native Rust |
| Database | SQLite | PostgreSQL (optional) | Local-first |
| Event bus | NATS JetStream | Custom | Scalability |
| CLI | clap v4 | argh | Mature, flexible |
| Testing | built-in | None | Native support |
| HTTP client | reqwest | hyper | Ergonomics |
| JSON | serde_json | simd-json | Compatibility |

### Tooling

| Purpose | Tool | Command | Notes |
|---------|------|---------|-------|
| Rust build | cargo | `cargo build` | Native |
| Python build | uv | `uv build` | Fast |
| TS/JS build | bun | `bun build` | Native bundler |
| Rust lint | clippy | `cargo clippy` | Comprehensive |
| Python lint | ruff | `ruff check` | 10-100x faster |
| TS lint | oxlint | `oxlint .` | Fast |
| Go lint | golangci-lint | `golangci-lint run` | Standard |
| Rust format | rustfmt | `cargo fmt` | Native |
| Python format | ruff | `ruff format` | Unified tool |
| TS format | prettier | `prettier --write` | Standard |
| Go format | gofmt | `gofmt -w` | Native |

---

## Development Workflow

### Spec-Driven Development

```
┌────────────┐    ┌────────────┐    ┌────────────┐    ┌────────────┐
│  Specify   │───▶│   Plan     │───▶│  Implement │───▶│  Validate  │
│            │    │            │    │            │    │            │
│ Create spec│    │ Create WPs │    │ Agent work │    │ Tests pass │
└────────────┘    └────────────┘    └────────────┘    └─────┬──────┘
                                                            │
┌────────────┐    ┌──────────────────────────────────────────┘
│   Merge    │◀───┘
│            │
│ Owner      │
│ reviews    │
└────────────┘
```

__Step-by-Step Workflow__

```bash
# 1. SPECIFY: Create feature specification
agileplus specify \
  --title "User Authentication System" \
  --description "Implement OAuth2-based authentication..." \
  --id FEATURE-001

# 2. PLAN: Create work packages
agileplus plan FEATURE-001 \
  --work-packages "WP-001: OAuth integration,WP-002: Session management"

# 3. IMPLEMENT: Agent works in worktree
git worktree add .worktrees/feat/FEATURE-001 -b feat/FEATURE-001
cd .worktrees/feat/FEATURE-001
# ... implementation ...

# 4. VALIDATE: Run tests and quality checks
cargo test --workspace
cargo clippy --workspace -- -D warnings
task quality

# 5. DOCUMENT: Update worklog
# Automatic via git commits

# 6. MERGE: Owner reviews and merges
gh pr create --title "feat: auth system" --body "..."
# Owner reviews and merges
```

### Branch Naming

Format: `<project>/<type>/<description>`

| Type | Use Case | Example | Description |
|------|----------|---------|-------------|
| `feat/` | New feature | `agileplus/feat/auth-refactor` | User-facing feature |
| `fix/` | Bug fix | `agileplus/fix/connection-leak` | Bug correction |
| `chore/` | Maintenance | `agileplus/chore/update-deps` | No user-visible change |
| `docs/` | Documentation | `agileplus/docs/api-reference` | Doc changes only |
| `refactor/` | Code restructuring | `agileplus/refactor/storage-layer` | No behavior change |
| `test/` | Test changes | `agileplus/test/auth-coverage` | Test-only changes |
| `ci/` | CI/CD changes | `agileplus/ci/arm64-builds` | Build/CI changes |

### Commit Format

Format: `<type>(<scope>): <description>`

| Type | Use Case | Example |
|------|----------|---------|
| `feat` | New feature | `feat(auth): add token refresh` |
| `fix` | Bug fix | `fix(storage): handle null values` |
| `chore` | Maintenance | `chore(deps): update tokio to 1.35` |
| `docs` | Documentation | `docs(api): add examples` |
| `refactor` | Restructuring | `refactor(cache): simplify eviction` |
| `test` | Test changes | `test(auth): add edge cases` |
| `ci` | CI/CD changes | `ci(test): add coverage reporting` |
| `perf` | Performance | `perf(cache): reduce allocations` |
| `security` | Security fix | `security(auth): validate tokens` |

---

## Testing Strategy

### Test Levels

| Level | Tool | Coverage Target | Purpose |
|-------|------|-----------------|---------|
| Unit | Built-in | 80% | Individual functions |
| Integration | cargo test | 60% | Component interaction |
| E2E | custom | Critical paths | Full workflows |

### Test Organization

```
crates/my-crate/
├── src/
│   └── lib.rs
├── tests/
│   ├── unit/              # Unit tests
│   │   ├── test_entity.rs
│   │   └── test_aggregate.rs
│   ├── integration/         # Integration tests
│   │   ├── test_storage.rs
│   │   └── test_events.rs
│   └── e2e/                 # End-to-end tests
│       └── test_workflow.rs
└── benches/                 # Benchmarks
    └── bench_operations.rs
```

### Test-First Mandate

| Scenario | Requirement | Verification |
|----------|-------------|--------------|
| New modules | Test file MUST exist before implementation | CI check |
| Bug fixes | Failing test MUST be written before fix | PR review |
| Refactors | Existing tests must pass before AND after | CI gate |

### FR Traceability

```rust
// Traces to: FR-AUTH-042
#[test]
fn test_auth_token_refresh_with_expired_token() {
    // Given: Valid but expired token
    let token = create_expired_token();
    
    // When: Attempting refresh
    let result = auth.refresh_token(&token);
    
    // Then: New token issued
    assert!(result.is_ok());
    assert!(result.unwrap().is_valid());
}
```

---

## Deployment

### Local Development Setup

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Install uv for Python
curl -LsSf https://astral.sh/uv/install.sh | sh

# 3. Install Bun for TypeScript
curl -fsSL https://bun.sh/install | bash

# 4. Clone and build
git clone <repo>
cd <repo>
cargo build --workspace
```

### CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      
      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --check
      
      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
      
      - name: Run tests
        run: cargo test --workspace
      
      - name: Security audit
        run: cargo audit
      
      - name: License check
        run: cargo deny check
```

### Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Create git tag: `git tag -a v1.0.0 -m "Release v1.0.0"`
4. Push tag: `git push origin v1.0.0`
5. CI builds and publishes to crates.io
6. Create GitHub release with notes

---

## Security

### Credentials Management

| Storage | Use Case | Implementation |
|---------|----------|----------------|
| OS keychain | Primary | `keyring` crate |
| File-based | Fallback | `zeroize` for clearing |
| Environment | CI/CD | GitHub Secrets |

__Principles:__

- No plaintext credentials in code
- No credentials in logs
- Memory sanitization on drop
- Rotation support

### Audit Integrity

- SHA-256 hash chains for all events
- Append-only event store
- Verifiable chain integrity via `validate` command
- Git-based backup with signatures

### SAST/DAST Tools

| Tool | Purpose | Integration | Severity |
|------|---------|-------------|----------|
| `cargo-audit` | Dependency vulnerabilities | CI gate | Block merge |
| `cargo-deny` | License compliance | CI gate | Block merge |
| `semgrep` | Static analysis | CI gate | Warning |
| `trivy` | Container scanning | CI gate | Block merge |
| `gitguardian` | Secrets detection | CI gate | Block merge |

### Security Principles

1. __Defense in depth:__ Multiple security layers
2. __Least privilege:__ Minimal permissions required
3. __Fail secure:__ Secure defaults
4. __No secrets in code:__ Environment-based configuration
5. __Audit everything:__ Log all security events

---

## Observability

### Metrics

| Metric | Source | Destination | Alert Threshold |
|--------|--------|-------------|-----------------|
| Build time | cargo | Prometheus | > 10 min |
| Test duration | cargo test | Prometheus | > 5 min |
| Lint violations | clippy | Prometheus | > 0 |
| Agent actions | MCP server | SQLite + Git | N/A |
| Error rate | Application | Prometheus | > 1% |

### Tracing

```rust
use tracing::{info, span, Level};

let span = span!(Level::INFO, "specify", feature_id = %id);
let _enter = span.enter();

info!("Creating feature specification");

// Structured fields
info!(target: "agileplus", user_id = %user.id, "Feature created");
```

### Logging Levels

| Level | Use Case | Example |
|-------|----------|---------|
| ERROR | Unrecoverable errors | Database connection failed |
| WARN | Degraded operation | Cache miss, fallback used |
| INFO | Normal operation | Feature specification created |
| DEBUG | Detailed diagnostics | SQL query executed |
| TRACE | Very detailed | Function entry/exit |

### Health Checks

```rust
pub trait HealthCheck {
    fn check(&self) -> HealthStatus;
}

pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

impl HealthCheck for MyService {
    fn check(&self) -> HealthStatus {
        if !self.db.is_connected() {
            return HealthStatus::Unhealthy("Database disconnected".into());
        }
        if self.cache.hit_rate() < 0.5 {
            return HealthStatus::Degraded("Low cache hit rate".into());
        }
        HealthStatus::Healthy
    }
}
```

---

## Integration Patterns

### MCP Integration

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct SpecifyArgs {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub priority: Priority,
}

pub struct SpecifyTool {
    engine: Arc<SpecEngine>,
}

#[async_trait]
impl Tool for SpecifyTool {
    fn name(&self) -> String {
        "agileplus_specify".into()
    }
    
    fn schema(&self) -> serde_json::Value {
        serde_json::to_value(SpecifyArgs::schema()).unwrap()
    }
    
    async fn call(&self, args: serde_json::Value) -> Result<serde_json::Value, ToolError> {
        let args: SpecifyArgs = serde_json::from_value(args)?;
        let spec = self.engine.specify(args.title, args.description).await?;
        Ok(serde_json::to_value(spec)?)
    }
}
```

### gRPC Integration

```protobuf
syntax = "proto3";

package phenotype.v1;

service StorageService {
    rpc Get(GetRequest) returns (GetResponse);
    rpc Set(SetRequest) returns (SetResponse);
    rpc Delete(DeleteRequest) returns (DeleteResponse);
}

message GetRequest {
    string key = 1;
}

message GetResponse {
    bytes value = 1;
    bool found = 2;
}
```

```rust
use tonic::{Request, Response, Status};

pub struct StorageServiceImpl {
    storage: Arc<dyn StoragePort>,
}

#[tonic::async_trait]
impl StorageService for StorageServiceImpl {
    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        let key = request.into_inner().key;
        match self.storage.get(&key).await {
            Ok(Some(value)) => Ok(Response::new(GetResponse { value, found: true })),
            Ok(None) => Ok(Response::new(GetResponse { value: vec![], found: false })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
```

### Event Bus Integration

```rust
use async_nats::Client;

pub struct EventBus {
    client: Client,
}

impl EventBus {
    pub async fn publish(&self, event: &Event) -> Result<()> {
        let subject = format!("events.{}.{}", event.aggregate_type, event.event_type);
        let payload = serde_json::to_vec(event)?;
        self.client.publish(subject, payload.into()).await?;
        Ok(())
    }
    
    pub async fn subscribe(&self, pattern: &str) -> Result<Subscriber> {
        let sub = self.client.subscribe(pattern).await?;
        Ok(Subscriber { inner: sub })
    }
}
```

---

## Migration Guidelines

### Adding a New Project

1. __Create project directory:__ `mkdir newproject`
2. __Add entry to INDEX.md:__ Document project purpose, language, status
3. __Create CLAUDE.md:__ Define project-specific agent rules
4. __Create README.md:__ Project overview and quick start
5. __Initialize git:__ `git init && git remote add origin <url>`
6. __Add worktree guidance:__ Document in project README
7. __Add to CI:__ Create `.github/workflows/ci.yml`

### Migrating to HexaKit

1. __Audit existing structure:__ Document current patterns
2. __Add required files:__ CLAUDE.md, AGENTS.md
3. __Align with quality standards:__ Add file size limits, test traceability
4. __Update dependencies:__ Use HexaKit foundation crates where applicable
5. __Add to INDEX.md:__ Document in projects registry

### Technology Migrations

| From | To | Trigger | Rollback |
|------|----|---------|----------|
| TypeScript 6.x | TypeScript 7 (tsgo) | Stable release | `tsc` |
| Node.js | Bun | Compatibility verified | `run:node` script |
| pip | uv | Performance testing passed | `pip` fallback |
| Rust 2021 | Rust 2024 | Nightly stable | Edition 2021 |

---

## Troubleshooting

### Common Issues

__Build Failures__

```bash
# Clear build cache
cargo clean

# Update dependencies
cargo update

# Check for lockfile issues
rm Cargo.lock && cargo build

# Verbose build for debugging
cargo build -vv
```

__Worktree Issues__

```bash
# List all worktrees
git worktree list

# Remove stale worktree
git worktree remove <path> --force

# Prune lost worktrees
git worktree prune

# Clean up all stale refs
git gc --prune=now
```

__Quality Gate Failures__

```bash
# Run quality checks locally
task quality

# Auto-fix formatting
cargo fmt
ruff format .
prettier --write .

# Check specific file
cargo clippy --package my-crate

# Run tests for specific package
cargo test --package my-crate
```

### Debug Commands

```bash
# Verbose build
cargo build -vv

# Backtrace on panic
RUST_BACKTRACE=1 cargo test

# Specific test with output
cargo test test_name -- --nocapture

# Check dependencies
cargo tree

# Check for outdated deps
cargo outdated
```

### Getting Help

| Resource | Location | Response Time |
|----------|----------|---------------|
| Documentation | `docs/` | Immediate |
| ADRs | `docs/adr/` | Immediate |
| Worklogs | `worklogs/` | Immediate |
| Issues | GitHub Issues | 24-48 hours |
| Discussions | GitHub Discussions | 24-72 hours |

---

## Advanced Topics

### Cross-Repository Refactoring

When a change affects multiple repositories:

1. Create tracking issue in HexaKit
2. Create worktrees in each affected repo
3. Make coordinated changes
4. Create PRs with cross-references
5. Merge in dependency order

### Performance Optimization

__Profiling__

```bash
# CPU profiling
cargo flamegraph

# Memory profiling
cargo heaptrack

# Benchmark comparison
cargo bench
```

__Optimization Targets__

| Metric | Target | Tool |
|--------|--------|------|
| Binary size | < 10MB | `cargo bloat` |
| Startup time | < 100ms | Custom timing |
| Memory usage | < 100MB | `heaptrack` |
| Request latency | < 10ms p99 | Tracing |

### Advanced Git Worktree Patterns

```bash
# Create worktree from existing branch
git worktree add -b feature-branch ../feature-branch origin/main

# Create bare worktree (for CI)
git worktree add --detach ../ci-build HEAD

# List worktrees with paths and branches
git worktree list --porcelain
```

---

## Performance Considerations

### Build Performance

| Optimization | Impact | Implementation |
|--------------|--------|----------------|
| sccache | 50-80% faster | Shared compilation cache |
| Mold linker | 2-5x faster link | `RUSTFLAGS="-C link-arg=-fuse-ld=mold"` |
| Parallel front-end | Faster compiles | `RUSTFLAGS="-Z threads=8"` |
| Profile-guided | 10-20% faster runtime | `cargo pgo` |

### Runtime Performance

| Technique | Use Case | Implementation |
|-----------|----------|----------------|
| Async/await | I/O bound | Tokio |
| Rayon | CPU parallel | `par_iter()` |
| Lock-free | High contention | `crossbeam` |
| SIMD | Data parallel | `std::simd` |

---

## Scalability Planning

### Repository Scaling

| Scale | Repositories | Strategy |
|-------|--------------|----------|
| Current | 30 | Shelf + worktrees |
| Medium | 100 | Automated worktree management |
| Large | 500 | Distributed caching |
| Enterprise | 1000+ | Federation |

### Performance Targets

| Metric | Current | Target | Scale Factor |
|--------|---------|--------|--------------|
| Build time | 5 min | < 10 min | 100 repos |
| Test time | 2 min | < 5 min | 100 repos |
| Query time | 100ms | < 500ms | 10k specs |
| Storage | 1GB | < 10GB | 100 repos |

---

## Disaster Recovery

### Backup Strategy

| Component | Frequency | Method | Retention |
|-----------|-----------|--------|-----------|
| Git repos | Continuous | Git remotes | Infinite |
| SQLite DB | Hourly | `cp *.db backup/` | 7 days |
| Worktrees | On commit | Git push | Infinite |
| Specs | On change | Git commit | Infinite |

### Recovery Procedures

__Git Repository Corruption__

```bash
# Clone fresh from remote
git clone <remote-url> fresh-clone
cp fresh-clone/.git .git

# Verify
git fsck --full
```

__SQLite Database Corruption__

```bash
# Restore from backup
cp backup/agileplus-2026-04-04.db agileplus.db

# Verify integrity
sqlite3 agileplus.db "PRAGMA integrity_check;"
```

---

## Related Documents

| Document | Location | Purpose | Lines |
|----------|----------|---------|-------|
| SOTA Research | [SOTA.md](./SOTA.md) | Technology landscape | 1900+ |
| ADR-001 | [ADR-001.md](./ADR-001.md) | Shelf organization | 117 |
| ADR-002 | [ADR-002.md](./ADR-002.md) | MCP integration | 131 |
| ADR-003 | [ADR-003.md](./ADR-003.md) | Agent hierarchy | 134 |
| Governance | `GOVERNANCE.md` | Policies and procedures | 500+ |
| User Journeys | `USER_JOURNEYS.md` | Workflow definitions | 400+ |
| Contributing | `CONTRIBUTING.md` | Contribution guidelines | 400+ |
| Security | `SECURITY.md` | Security policies | 300+ |

---

## Appendix A: Project Index

### Foundation Projects

| Project | Path | Language | Status | Crates |
|---------|------|----------|--------|--------|
| phenotype-infrakit | `phenotype-infrakit/` | Rust | Active | 8 |
| agileplus-plugin-core | `agileplus-plugin-core/` | Rust | Active | 1 |
| agileplus-plugin-git | `agileplus-plugin-git/` | Rust | Active | 1 |
| agileplus-plugin-sqlite | `agileplus-plugin-sqlite/` | Rust | Active | 1 |

### Application Projects

| Project | Path | Language | Status |
|---------|------|----------|--------|
| agileplus | `agileplus/` | Rust + Python | Active |
| thegent | `platforms/thegent/` | Go + Python + TS | Active |
| heliosCLI | `heliosCLI/` | Rust | Active |
| phenoSDK | `phenoSDK/` | Python | Active |

### Kit Projects

| Project | Path | Language | Status |
|---------|------|----------|--------|
| HexaPy | `kits/HexaPy/` | Python | Active |
| HexaGo | `kits/HexaGo/` | Go | Active |
| HexaType | `kits/HexaType/` | TypeScript | Active |
| hexagon-rs | `kits/hexagon-rs/` | Rust | Active |
| hexagon-ts | `kits/hexagon-ts/` | TypeScript | Active |
| hexagon-python | `kits/hexagon-python/` | Python | Active |

---

## Appendix B: File Templates

### CLAUDE.md Full Template

```markdown
# CLAUDE.md — Project Name

## Project Identity

- **Type:** application|library|kit|harness
- **Language:** Rust|Python|TypeScript|Go
- **Purpose:** One-line description
- **Owner:** Name or role

## Architecture

### Pattern

Hexagonal/Monolithic/Layered

### Structure

\`\`\`
src/
├── ports/          # Interfaces
├── domain/         # Business logic
├── adapters/       # Implementations
└── lib.rs
\`\`\`

## Quality Gates

| Gate | Tool | Command |
|------|------|---------|
| Tests | cargo | `cargo test` |
| Lint | clippy | `cargo clippy` |
| Format | rustfmt | `cargo fmt` |

## Agent Rules

### Can Edit

Any file except:
- SECURITY.md
- LICENSE

### Must Follow

- File size limits (350/500)
- Test traceability
- FR references

### Cannot

- Push to main
- Merge PRs
- Modify CI secrets
```

### AGENTS.md Full Template

```markdown
# AGENTS.md — Project Name

## This Project Uses AgilePlus

All work tracked via:
- Specs: `kitty-specs/`
- CLI: `agileplus <command>`

## Agent Roles

| Agent | Can Edit | Can Commit | Scope |
|-------|----------|------------|-------|
| Forge | All files | All branches | Implementation |
| Muse | Comments | No | Review |
| Sage | Docs/Research | All branches | Investigation |
| Helios | Tests only | Test branches | Testing |

## Branch Naming

Format: `<type>/<description>`

Types: feat, fix, chore, docs, refactor, test

## Quality Requirements

- [ ] All tests pass
- [ ] All linters pass
- [ ] File size < 500 lines
- [ ] FR traceability on tests
```

---

## Appendix C: Command Reference

### AgilePlus CLI

| Command | Purpose | Example |
|---------|---------|---------|
| `agileplus specify` | Create feature specification | `agileplus specify --title "Auth"` |
| `agileplus plan` | Create implementation plan | `agileplus plan FEATURE-001` |
| `agileplus status` | Update work package status | `agileplus status F-001 --wp WP-001 --state done` |
| `agileplus validate` | Validate spec/plan | `agileplus validate FEATURE-001` |
| `agileplus list` | List features/plans | `agileplus list --status implementing` |
| `agileplus show` | Show spec details | `agileplus show FEATURE-001` |
| `agileplus evidence` | Attach evidence | `agileplus evidence F-001 --file test-results.txt` |

### Git Worktrees

| Command | Purpose | Example |
|---------|---------|---------|
| `git worktree add <path> -b <branch>` | Create worktree | `git worktree add .worktrees/feat -b feat/x` |
| `git worktree list` | List worktrees | `git worktree list` |
| `git worktree remove <path>` | Remove worktree | `git worktree remove .worktrees/feat` |
| `git worktree prune` | Clean stale refs | `git worktree prune` |
| `git worktree move <old> <new>` | Rename worktree | `git worktree move old new` |
| `git worktree lock <path>` | Lock worktree | `git worktree lock .worktrees/feat` |
| `git worktree unlock <path>` | Unlock worktree | `git worktree unlock .worktrees/feat` |

### Quality Commands

| Command | Purpose | Scope |
|---------|---------|-------|
| `cargo test --workspace` | Run all tests | All crates |
| `cargo clippy --workspace` | Run linter | All crates |
| `cargo fmt --check` | Check formatting | All files |
| `cargo audit` | Security audit | Dependencies |
| `cargo deny check` | License check | Dependencies |
| `task quality` | Full quality suite | Defined checks |
| `task quality:full` | Full + format check | All checks |

---

## Appendix D: Glossary

| Term | Definition | Context |
|------|------------|---------|
| __ADR__ | Architecture Decision Record | Documentation |
| __HexaKit__ | This repos shelf | Organization |
| __Kit__ | Project scaffolding template | Templates |
| __MCP__ | Model Context Protocol | AI Integration |
| __Port__ | Interface in hexagonal architecture | Architecture |
| __Shelf__ | Organizational layer for polyrepo | Organization |
| __Spec__ | Feature specification | Planning |
| __Worktree__ | Git feature for multiple working directories | Development |
| __WP__ | Work Package | Planning |
| __FR__ | Functional Requirement | Requirements |
| __FSM__ | Finite State Machine | Architecture |
| __SOTA__ | State of the Art | Research |

---

## Appendix F: Code Examples

### Rust Hexagonal Implementation

__Port Definition__

```rust
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[async_trait]
pub trait StoragePort: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>, StorageError>;
}
```

__Domain Implementation__

```rust
pub struct SpecEngine {
    storage: Arc<dyn StoragePort>,
}

impl SpecEngine {
    pub fn new(storage: Arc<dyn StoragePort>) -> Self {
        Self { storage }
    }
    
    pub async fn create_spec(&self, spec: &Spec) -> Result<SpecId, DomainError> {
        self.validate_spec(spec)?;
        let id = SpecId::generate();
        let data = serde_json::to_vec(spec)?;
        self.storage.set(&id.to_string(), data).await?;
        Ok(id)
    }
    
    fn validate_spec(&self, spec: &Spec) -> Result<(), DomainError> {
        if spec.title.is_empty() {
            return Err(DomainError::InvalidTitle);
        }
        if spec.description.len() < 10 {
            return Err(DomainError::DescriptionTooShort);
        }
        Ok(())
    }
}
```

__SQLite Adapter__

```rust
use sqlx::SqlitePool;

pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS kv (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL
            )"
        )
        .execute(&pool)
        .await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl StoragePort for SqliteStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let row = sqlx::query("SELECT value FROM kv WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get(0)))
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        sqlx::query(
            "INSERT INTO kv (key, value) VALUES (?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value"
        )
        .bind(key)
        .bind(&value)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM kv WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    async fn list(&self, prefix: &str) -> Result<Vec<String>, StorageError> {
        let pattern = format!("{}%", prefix);
        let rows = sqlx::query("SELECT key FROM kv WHERE key LIKE ?")
            .bind(&pattern)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(|r| r.get(0)).collect())
    }
}
```

### Python MCP Tool Implementation

```python
from mcp.server.fastmcp import FastMCP
from pydantic import BaseModel
from typing import Optional

mcp = FastMCP("agileplus")

class SpecifyArgs(BaseModel):
    title: str
    description: str
    priority: Optional[str] = "medium"

@mcp.tool()
async def specify(args: SpecifyArgs) -> dict:
    """Create a new feature specification."""
    engine = get_spec_engine()
    spec = await engine.specify(
        title=args.title,
        description=args.description,
        priority=args.priority
    )
    return {
        "spec_id": spec.id,
        "title": spec.title,
        "status": spec.status
    }

@mcp.resource("kitty-spec://features/{spec_id}")
async def get_spec(spec_id: str) -> str:
    """Get a feature specification as markdown."""
    engine = get_spec_engine()
    spec = await engine.get_spec(spec_id)
    return render_spec_markdown(spec)

if __name__ == "__main__":
    mcp.run(transport="stdio")
```

### Event Sourcing Implementation

```rust
use sha2::{Sha256, Digest};
use uuid::Uuid;
use chrono::Utc;

pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub aggregate_id: String,
    pub sequence: u64,
    pub timestamp: chrono::DateTime<Utc>,
    pub payload: serde_json::Value,
    pub hash: String,
    pub previous_hash: Option<String>,
}

impl Event {
    pub fn new(
        event_type: &str,
        aggregate_id: &str,
        sequence: u64,
        payload: serde_json::Value,
        previous_hash: Option<String>,
    ) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let hash = compute_hash(&id, event_type, aggregate_id, sequence, &payload, &previous_hash);
        
        Self {
            id,
            event_type: event_type.to_string(),
            aggregate_id: aggregate_id.to_string(),
            sequence,
            timestamp,
            payload,
            hash,
            previous_hash,
        }
    }
}

fn compute_hash(
    id: &Uuid,
    event_type: &str,
    aggregate_id: &str,
    sequence: u64,
    payload: &serde_json::Value,
    previous_hash: &Option<String>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(id.as_bytes());
    hasher.update(event_type.as_bytes());
    hasher.update(aggregate_id.as_bytes());
    hasher.update(sequence.to_le_bytes());
    hasher.update(payload.to_string().as_bytes());
    if let Some(prev) = previous_hash {
        hasher.update(prev.as_bytes());
    }
    hex::encode(hasher.finalize())
}
```

---

## Appendix G: Testing Patterns

### Unit Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Traces to: FR-001-042
    #[test]
    fn test_spec_creation_with_valid_data() {
        // Given
        let storage = Arc::new(InMemoryStorage::new());
        let engine = SpecEngine::new(storage);
        let spec = Spec {
            title: "Test Feature".to_string(),
            description: "This is a test description".to_string(),
        };
        
        // When
        let result = engine.create_spec(&spec);
        
        // Then
        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(!id.to_string().is_empty());
    }
    
    // Traces to: FR-001-043
    #[test]
    fn test_spec_creation_fails_with_empty_title() {
        // Given
        let storage = Arc::new(InMemoryStorage::new());
        let engine = SpecEngine::new(storage);
        let spec = Spec {
            title: "".to_string(),
            description: "Description".to_string(),
        };
        
        // When
        let result = engine.create_spec(&spec);
        
        // Then
        assert!(matches!(result, Err(DomainError::InvalidTitle)));
    }
}
```

### Integration Test Pattern

```rust
// tests/integration/test_storage.rs

use agileplus::adapters::sqlite::SqliteStorage;
use agileplus::ports::StoragePort;

#[tokio::test]
async fn test_sqlite_storage_roundtrip() {
    // Given
    let storage = SqliteStorage::new(":memory:").await.unwrap();
    let key = "test-key";
    let value = b"test-value".to_vec();
    
    // When
    storage.set(key, value.clone()).await.unwrap();
    let retrieved = storage.get(key).await.unwrap();
    
    // Then
    assert_eq!(retrieved, Some(value));
}

#[tokio::test]
async fn test_sqlite_storage_delete() {
    // Given
    let storage = SqliteStorage::new(":memory:").await.unwrap();
    let key = "test-key";
    storage.set(key, b"value".to_vec()).await.unwrap();
    
    // When
    storage.delete(key).await.unwrap();
    let retrieved = storage.get(key).await.unwrap();
    
    // Then
    assert_eq!(retrieved, None);
}
```

### Mock Adapter Pattern

```rust
use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryStorage {
    data: Mutex<HashMap<String, Vec<u8>>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl StoragePort for InMemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let data = self.data.lock().unwrap();
        Ok(data.get(key).cloned())
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        let mut data = self.data.lock().unwrap();
        data.insert(key.to_string(), value);
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let mut data = self.data.lock().unwrap();
        data.remove(key);
        Ok(())
    }
    
    async fn list(&self, prefix: &str) -> Result<Vec<String>, StorageError> {
        let data = self.data.lock().unwrap();
        Ok(data
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect())
    }
}
```

---

## Appendix H: Configuration Reference

### Cargo.toml Template

```toml
[package]
name = "phenotype-example"
version = "0.1.0"
edition = "2021"
authors = ["Koosha Paridehpour <koosha@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/KooshaPari/phenotype-example"
description = "Example phenotype crate"
rust-version = "1.75"

[dependencies]
# Async
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Database
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }

# Testing
tempfile = "3.8"

[dev-dependencies]
tokio-test = "0.4"
pretty_assertions = "1.4"

[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite"]
postgres = ["sqlx/postgres"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 0
debug = true
```

### GitHub Actions CI Template

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, nightly]
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
          components: clippy, rustfmt
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run doc tests
        run: cargo test --doc --all-features

  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

### Policy Configuration Template

```yaml
# .policy.yml
version: "1.0"

policies:
  - id: file-size-limits
    name: File Size Limits
    description: Enforce maximum file sizes for maintainability
    applies_to: ["**/*.rs", "**/*.py", "**/*.ts"]
    rules:
      - id: soft-limit
        name: Soft Limit Warning
        severity: warning
        condition:
          type: file_size
          max_lines: 350
        message: |
          File exceeds soft limit of 350 lines.
          Consider decomposing into smaller modules.
      
      - id: hard-limit
        name: Hard Limit Enforcement
        severity: error
        condition:
          type: file_size
          max_lines: 500
        message: |
          File exceeds hard limit of 500 lines.
          Must be decomposed before merge.

  - id: test-traceability
    name: Test Traceability
    description: All tests must trace to functional requirements
    applies_to: ["**/*.rs"]
    excludes: ["**/tests/integration/**"]
    rules:
      - id: fr-trace-required
        name: FR Trace Required
        severity: error
        condition:
          type: fr_traceability
          pattern: "// Traces to: FR-\d{3}-\d{3}"
        message: |
          Test must include FR trace comment.
          Format: // Traces to: FR-XXX-NNN

  - id: documentation-coverage
    name: Documentation Coverage
    description: Public API must be documented
    applies_to: ["**/src/**/*.rs"]
    rules:
      - id: public-api-docs
        name: Public API Documentation
        severity: warning
        condition:
          type: documentation_coverage
          min_percent: 80
          include_public_only: true
        message: |
          Public API documentation coverage below 80%.
          Add rustdoc comments to public items.
```

---

## Appendix I: Migration Guides

### Adding a New Crate to Workspace

1. Create crate directory:
   ```bash
   cd crates
   cargo new --lib my-crate
   ```

2. Update workspace Cargo.toml:
   ```toml
   [workspace]
   members = ["crates/*"]
   resolver = "2"
   ```

3. Add standard dependencies:
   ```toml
   [dependencies]
   thiserror = "1.0"
   serde = { version = "1.0", features = ["derive"] }
   tracing = "0.1"
   ```

4. Create port trait if needed:
   ```rust
   // src/ports/my_port.rs
   #[async_trait]
   pub trait MyPort: Send + Sync {
       async fn do_something(&self) -> Result<(), Error>;
   }
   ```

5. Add tests:
   ```bash
   touch tests/test_my_port.rs
   ```

6. Run quality checks:
   ```bash
   cargo check
   cargo clippy
   cargo test
   ```

### Converting Existing Code to Hexagonal

1. Identify external dependencies (database, HTTP, filesystem)
2. Create port trait for each dependency
3. Move business logic to domain module
4. Implement adapters for each port
5. Update tests to use mock adapters
6. Verify all tests pass
7. Update documentation

---

## Appendix J: Troubleshooting Guide

### Build Issues

| Symptom | Cause | Solution |
|---------|-------|----------|
| `linker not found` | Missing system deps | Install build-essential |
| `feature not found` | Old Rust version | Run `rustup update` |
| `conflicting versions` | Dependency conflict | Run `cargo update` |
| `disk space` | Large target dir | Run `cargo clean` |

### Test Failures

| Symptom | Cause | Solution |
|---------|-------|----------|
| `test timeout` | Async deadlock | Check for blocking calls |
| `db locked` | SQLite concurrent access | Use connection pooling |
| `port in use` | Previous test didn't cleanup | Use random ports |

### Worktree Issues

| Symptom | Cause | Solution |
|---------|-------|----------|
| `already checked out` | Branch in another worktree | Use `git worktree list` |
| `permission denied` | Locked worktree | Run `git worktree unlock` |
| `path not found` | Stale worktree | Run `git worktree prune` |

---

## Appendix K: Performance Benchmarks

### Target Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Spec creation | < 100ms | End-to-end |
| Query response | < 50ms | p99 latency |
| Build time | < 5 min | Clean build |
| Test time | < 2 min | Full suite |
| Binary size | < 10MB | Release build |

### Profiling Commands

```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin my-app

# Memory profiling
cargo install dhat
cargo run --features dhat-heap

# Benchmark comparison
cargo bench > baseline.txt
# ... make changes ...
cargo bench > new.txt
```

---

## Appendix L: Contributing Guidelines

### Before Submitting

- [ ] All tests pass locally
- [ ] Clippy reports no warnings
- [ ] Code is formatted
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] FR traceability verified

### PR Template

```markdown
## Summary

Brief description of changes.

## Changes

- Change 1
- Change 2

## Testing

How changes were tested.

## Checklist

- [ ] Tests pass
- [ ] Linting passes
- [ ] Documentation updated
- [ ] FR traceability verified
```

---

## Appendix M: External References

### Standards

| Standard | URL | Usage |
|----------|-----|-------|
| MCP | modelcontextprotocol.io | AI agent protocol |
| gRPC | grpc.io | Service communication |
| OpenTelemetry | opentelemetry.io | Observability |
| Conventional Commits | conventionalcommits.org | Commit format |
| Semantic Versioning | semver.org | Version format |

### Tools

| Tool | URL | Purpose |
|------|-----|---------|
| Rust | rust-lang.org | Systems language |
| Cargo | doc.rust-lang.org/cargo | Build tool |
| Clippy | github.com/rust-lang/rust-clippy | Linter |
| SQLx | github.com/launchbadge/sqlx | Database |
| Tonic | github.com/hyperium/tonic | gRPC |
| Tokio | tokio.rs | Async runtime |

---

## Appendix N: Changelog

### 2026-04-04

- Created nanovms-level documentation
- Added comprehensive SOTA research (1900+ lines)
- Added 3 ADRs following nanovms format
- Expanded SPEC to 2500+ lines
- Aligned all documentation with nanovms minimalist style
- Added comprehensive appendices (A-N)

### 2026-04-03

- Initial SPEC creation (274 lines)
- Established project taxonomy
- Defined quality standards

---

__Spec Owner:__ Platform Architect  
__Last Updated:__ 2026-04-04  
__Status:__ Active
