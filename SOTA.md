# State of the Art — HexaKit Technology Landscape

**Version:** 1.0  
**Status:** Active  
**Updated:** 2026-04-04

---

## Overview

This document captures the state-of-the-art technology decisions, competitive landscape analysis, and rationale for key architectural choices in the HexaKit repos shelf.

---

## Technology Adoption Philosophy

**Aggressive adoption of preview/dev versions is encouraged across ALL languages and frameworks.**

### General Principles

- Do NOT gradually migrate — switch completely to new versions
- Do NOT be scared of non-release versions (dev, preview, rc, nightly)
- Always prefer the latest available version, even if preview
- Rollback is always possible if issues arise
- Prefer native/compiler rewrites over legacy implementations

---

## Language Targets

| Language | Current Preview/Nightly | Primary | Legacy Escape |
|----------|------------------------|---------|--------------|
| **TypeScript** | TypeScript 7 native (Go-based) | `tsgo` | `tsc` |
| **Rust** | Nightly compiler, Edition 2024 | Latest nightly | None |
| **Go** | Go 1.24+ | Latest stable | None |
| **Python** | Python 3.14 dev | `uv`, `ruff`, `rye` | `pip` + `venv` |
| **Bun** | Bun 1.2+ | Bun | Node.js |

---

## Core Technology Choices

### 1. Rust as Systems Language

**Decision:** Rust (Edition 2021/2024) for all performance-critical infrastructure.

**Rationale:**
- Memory safety without GC (critical for long-running agent processes)
- Zero-cost abstractions enable high-throughput event processing
- Rich type system enforces correctness at compile time
- Excellent async story with Tokio

**Alternatives Considered:**
- C++: Rejected — no memory safety guarantees, manual resource management
- Zig: Deferred — ecosystem immature, tooling gaps
- Go: Rejected for core — GC pauses unacceptable for real-time agent dispatch

### 2. TypeScript 7 Native (Go-based Compiler)

**Decision:** TypeScript 7 via `tsgo` as primary, `tsc` as legacy escape hatch.

**Rationale:**
- 5-10x faster type checking than tsc
- Native Go implementation aligns with Phenotype Go projects
- Full type-system compatibility with TypeScript 6.x

**Alternatives Considered:**
- TypeScript 6.x only: Rejected — not aggressive enough
- ocaml/purescript: Rejected — ecosystem mismatch

### 3. Python 3.14 with uv

**Decision:** Python 3.14 (dev) with `uv` package manager, `ruff` linter.

**Rationale:**
- `uv` is 10-100x faster than pip
- Python 3.14 brings performance improvements (no GIL removal yet but better async)
- Ruff is 10-100x faster than flake8+black

**Alternatives Considered:**
- Python 3.13: Rejected — 3.14 is available in CI
- Poetry: Rejected — slower than uv, more complex

### 4. Bun as JavaScript Runtime

**Decision:** Bun 1.2+ as primary, Node.js as legacy escape hatch.

**Rationale:**
- Bun is 3-5x faster than Node.js for typical workloads
- Native TypeScript support (no transpilation step)
- Drop-in Node.js compatibility

### 5. SQLite for Local-First Storage

**Decision:** SQLite (bundled) as sole persistence layer for local-first operation.

**Rationale:**
- Zero-dependency deployment
- Full offline operation
- No connection management overhead
- Sufficient for single-developer workloads

**Alternatives Considered:**
- PostgreSQL: Rejected — requires running server, breaks local-first
- SurrealDB: Rejected — immature embedding story
- Redis: Rejected — requires separate process, overhead

---

## Architecture Patterns

### Hexagonal (Ports & Adapters) Architecture

**Decision:** All crates follow hexagonal architecture with port traits and adapter implementations.

**Structure:**
```
Domain (business logic) ← Port Traits (interfaces) ← Adapters (implementations)
```

**Benefits:**
- Domain logic testable with in-memory stubs
- Adapters swappable at runtime
- New backends require only new adapter crate

**Key Ports:**
| Port | Purpose |
|------|---------|
| `StoragePort` | Persistence operations |
| `VcsPort` | Version control operations |
| `AgentPort` | AI agent dispatch |
| `ObservabilityPort` | Tracing, metrics, logging |

### Event Sourcing with Hash Chains

**Decision:** Every state mutation produces an `Event` + `AuditEntry`, both append-only with SHA-256 hash chains.

**Benefits:**
- Tamper detection for any entry
- Full event-sourcing capability
- Reconstruction of state from event stream

### gRPC + Protobuf for Services

**Decision:** All inter-service contracts defined in `.proto` files, generated via `tonic-build` + `prost`.

**Benefits:**
- Strongly-typed, versioned contracts
- Buf linting catches API regressions
- Bidirectional streaming available

---

## Agent Integration Patterns

### MCP (Model Context Protocol)

**Decision:** All AI agent integrations use MCP as the primary protocol.

**Implementation:**
- Python MCP server (`agileplus-mcp`) using FastMCP 3.0
- MCP SDKs in Go, Python, TypeScript (`platforms/thegent/`)
- gRPC bridge between Python MCP and Rust core

**Benefits:**
- Standardized AI agent protocol
- Rich tool/resource/prompt primitives
- Server-initiated sampling support

### Worktree Isolation

**Decision:** Each agent work package runs in an isolated git worktree.

**Benefits:**
- No cross-WP file conflicts
- Clean branch per feature/WP
- Parallel agent execution without interference

---

## Observability Stack

### OpenTelemetry

**Decision:** OpenTelemetry for all observability with OTLP export.

**Components:**
- `tracing` for structured logging and spans
- `opentelemetry-otlp` for trace export
- `tracing-opentelemetry` for span correlation

**Benefits:**
- Backend-agnostic (Jaeger, Grafana Tempo, Honeycomb)
- Automatic span correlation across async boundaries
- Metrics collection per command

---

## Competitive Landscape

### Similar Systems

| System | Comparison | Differentiation |
|--------|------------|-----------------|
| **OpenSpec** | Simple 3-command planning | Lacks AI agent integration, no governance |
| **spec-kitty** | Structured specs, worktree isolation | Lacks hash-chained audit, no MCP |
| **bmad** | Enterprise depth, role-based agents | Heavy, complex, not local-first |
| **GSD** | Automation, parallel execution | Lacks spec-driven development |

### Key Differentiators for HexaKit

1. **Local-first**: No server required, full offline operation
2. **Governance-verified**: Hash-chained audit + evidence-driven transitions
3. **AI-native**: MCP-first integration with Claude Code, Codex, Cursor
4. **Polyglot**: Multi-language support with strong typing across all

---

## Security Posture

### Credentials Management

- OS keychain (macOS Keychain, Linux Secret Service, Windows Credential Manager)
- File-based fallback with `zeroize` for memory sanitization
- No plaintext credentials on disk

### Audit Integrity

- SHA-256 hash chains for tamper detection
- Append-only event store
- Verifiable chain integrity via `validate` command

### SAST/DAST

| Tool | Purpose | Integration |
|------|---------|-------------|
| `cargo-audit` | Dependency vulnerabilities | CI gate |
| `cargo-deny` | License compliance, banned deps | CI gate |
| `semgrep` | Static analysis | CI gate |
| `trivy` | Container scanning | CI gate |
| `gitguardian` | Secrets detection | CI gate |

---

## Future Considerations

### Technology Watch List

| Technology | Status | Evaluation Date |
|------------|--------|-----------------|
| Rust Edition 2024 | Monitor | 2026-Q3 |
| Zig 0.14 | Research | 2026-Q4 |
| Python 3.15 | Monitor | 2026-Q4 |
| TypeScript 8 | Monitor | 2027-Q1 |

### Potential Migrations

| From | To | Trigger |
|------|----|---------|
| `tsc` | `tsgo` | TS7 Go-based stable |
| `pip` | `uv` | Complete (already done) |
| `flake8+black` | `ruff` | Complete (already done) |

---

## References

- [ADR-001: Rust Workspace Monorepo](./ADR.md#adr-001-rust-workspace-monorepo-with-22-crates)
- [ADR-002: Hexagonal Architecture](./ADR.md#adr-002-hexagonal-architecture-with-portadapter-pattern)
- [ADR-003: SQLite Local-First](./ADR.md#adr-003-sqlite-as-local-first-storage-with-optional-external-sync)
- [ADR-004: Hash-Chained Audit Log](./ADR.md#adr-004-sha-256-hash-chained-immutable-audit-log-and-event-store)
- [ADR-005: gRPC Services](./ADR.md#adr-005-grpc-service-layer-with-tonic--protobuf-for-mcp-and-inter-service-communication)

---

**Document Owner:** Architecture Team  
**Last Updated:** 2026-04-04  
**Next Review:** 2026-07-04
