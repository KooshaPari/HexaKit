# Phenotype Kits Workspace

A curated collection of reusable Rust libraries for modern software development with hexagonal architecture.

## Packages

| Package | Description | Crates.io |
|---------|-------------|-----------|
| [clikit](clikit/) | Universal CLI framework | - |
| [agentkit](agentkit/) | Agent framework with skills | - |
| [evalkit](evalkit/) | Evaluation framework | - |
| [taskkit](taskkit/) | Task execution framework | - |
| [configkit](configkit/) | Configuration management | - |
| [authkit](authkit/) | Auth/AuthZ framework | - |
| [cachekit](cachekit/) | Caching abstraction | - |
| [logkit](logkit/) | Structured logging | - |
| [tracingkit](tracingkit/) | Distributed tracing | - |
| [metrickit](metrickit/) | Metrics collection | - |
| [eventkit](eventkit/) | Event-driven architecture | - |

## Architecture

All packages follow hexagonal (ports & adapters) architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                    HEXAGONAL ARCHITECTURE                     │
├─────────────────────────────────────────────────────────────┤
│  Domain Layer (Pure Business Logic)                          │
│  ├── Entities, Value Objects, Ports (interfaces)            │
│  └── Domain Services, Events                                │
├─────────────────────────────────────────────────────────────┤
│  Application Layer (Use Cases)                               │
│  ├── Commands, Queries, Services (CQRS)                    │
│  └── Application Services                                   │
├─────────────────────────────────────────────────────────────┤
│  Adapters Layer (Infrastructure)                            │
│  ├── Primary: CLI, HTTP, GraphQL                             │
│  └── Secondary: Database, Cache, Queue                      │
├─────────────────────────────────────────────────────────────┤
│  Infrastructure Layer                                        │
│  ├── Error Handling, Logging, Observability                 │
│  └── Cross-cutting Concerns                                 │
└─────────────────────────────────────────────────────────────┘
```

## xDD Methodologies (140+)

All packages apply these methodologies:

- **Development**: TDD, BDD, DDD, ATDD, SDD, FDD, CDD, IDD, MDD, RDD
- **Design**: DRY, KISS, YAGNI, SOLID (5), LoD, SoC, PoLA
- **Architecture**: Clean, Hexagonal, Onion, CQRS, Event Sourcing
- **Quality**: Property-Based, Mutation, Contract, Shift-Left, Chaos
- **Process**: DevOps, CI/CD, Agile, Scrum, Kanban, GitOps
- **Documentation**: ADR, RFC, Design Docs, Runbooks, SpecDD

## Getting Started

```bash
# Clone the workspace
git clone https://github.com/KooshaPari/kits.git
cd kits

# Build all packages
cargo build --workspace

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Follow xDD methodologies
4. Add tests
5. Update documentation
6. Submit PR

## License

MIT OR Apache-2.0

---

## Absorb note — 2026-06-20

This cargo-generate target was assembled on 2026-06-20 from:

- **`KooshaPari/pheno-cargo-template`** — root scaffold
  (`Cargo.toml`, `src/lib.rs`, `Taskfile.yml`, `justfile`,
  `LICENSE-{MIT,APACHE}`, `.github/workflows/ci.yml`) plus the
  cargo-generate liquid fragments under `template/` (`Cargo.toml`,
  `src/lib.rs`, `AGENTS.md.additions`, `PREDICTIVE.md`, `PROMOTION.md`,
  `CONTRIBUTING.md`, `.predict.yaml`, `.drift-detector.yaml`,
  `.framework-lint.yaml`, `.github/workflows/predictive-dry-check.yml`).
- **`KooshaPari/Apisync`** (commit `d981353`) — `templates/hexagonal/`
  overlay: Sentry starter (`src/sentry_config.rs`), `.clippy.toml`,
  `rust-toolchain.toml`, and v8 governance pre-fill.

After the absorb, `pheno-cargo-template` is deprecated and redirects
to this subfolder:

```
cargo generate --git https://github.com/KooshaPari/HexaKit \
    --subfolder templates/hexagon/rust
```

Use `cargo generate` with the `--subfolder` flag (or `cp -R templates/hexagon/rust`).
The `template/` cargo-generate partials live in `_partials/`.

---

## Absorbed: pheno-cargo-template (2026-06-20)

This template is the canonical Rust hexagon skeleton for the Phenotype/Hexagon family.
The `pheno-cargo-template` source repo has been absorbed into `KooshaPari/HexaKit` at this path.

### What's merged in

- Root scaffold (`Cargo.toml`, `src/lib.rs`)
- `Taskfile.yml` + `justfile` recipes (timeout, deny, hygiene, release, coverage)
- `LICENSE-MIT` + `LICENSE-APACHE` (dual)
- `.github/workflows/ci.yml` + `.github/workflows/predictive-dry-check.yml`
- cargo-generate liquid fragments under `_partials/`
- Apisync overlay (sentry_config.rs, _typos.toml, .clippy.toml, cliff.toml, deny.toml, mise.toml, nextest.toml, rust-toolchain.toml, rustfmt.toml, .agileplus/)
- Spec & governance: `SPEC.md`, `llms.txt`, `WORKLOG.md`, `CHANGELOG.md`, `AGENTS.md`

### Source

`https://github.com/KooshaPari/pheno-cargo-template` (now redirects to this path).
