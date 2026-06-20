# Hexacore — Workspace Specification

Rust workspace of 11 reusable hexagonal architecture kits for the Phenotype ecosystem.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Hexacore Workspace                       │
├──────────┬──────────┬──────────┬──────────┬─────────────────┤
│  clikit  │agentkit  │ evalkit  │ taskkit  │   configkit     │
├──────────┼──────────┼──────────┼──────────┼─────────────────┤
│ authkit  │ cachekit │  logkit  │tracingkit│   metrickit     │
├──────────┼──────────┴──────────┴──────────┴─────────────────┤
│ eventkit │         Shared: phenotype-contracts               │
└──────────┴──────────────────────────────────────────────────┘
```

Each kit follows hexagonal (ports & adapters) layers:

```
┌───────────────────────────────┐
│       Adapters Layer          │  CLI, HTTP, gRPC, DB, Queue
├───────────────────────────────┤
│        Ports Layer            │  Input/Output interfaces
├───────────────────────────────┤
│       Domain Layer            │  Entities, Value Objects, Events
├───────────────────────────────┤
│     Application Layer         │  Use Cases, Commands, Queries
└───────────────────────────────┘
```

## Components

| Kit | Purpose | Key Types |
|-----|---------|-----------|
| clikit | Universal CLI framework | App, Command, Flag, Arg |
| agentkit | Agent framework with skills | Agent, Skill, Context, Tool |
| evalkit | Evaluation framework | Evaluator, Metric, Benchmark |
| taskkit | Task execution framework | Task, Scheduler, Queue, Worker |
| configkit | Configuration management | Config, Provider, Loader |
| authkit | Auth/AuthZ framework | Token, Principal, Policy |
| cachekit | Caching abstraction | Cache, Store, Serializer |
| logkit | Structured logging | Logger, Span, Field |
| tracingkit | Distributed tracing | Tracer, Span, Context |
| metrickit | Metrics collection | Counter, Gauge, Histogram |
| eventkit | Event-driven architecture | Event, Bus, Store, Handler |

## Data Models

```rust
trait Port { fn name(&self) -> &str; }
trait InputPort: Port { fn execute(&self, cmd: Command) -> Result<Output>; }
trait OutputPort: Port { fn connect(&mut self) -> Result<()>; }

struct Entity<ID> { id: ID, created_at: SystemTime }
trait AggregateRoot<ID>: Entity<ID> {
    fn domain_events(&self) -> &[DomainEvent];
    fn clear_events(&mut self);
}
```

## API Design

```rust
// Kit initialization
let app = clikit::App::new("myapp")
    .command(HelloCommand::default())
    .middleware(LoggingMiddleware::new());

// Agent execution
let agent = agentkit::Agent::builder()
    .skill(WebSearch::new())
    .skill(CodeExec::new())
    .build();
let result = agent.execute(context, input).await?;

// Event publishing
let bus = eventkit::Bus::new();
bus.publish(OrderCreated { order_id }).await?;
```

## Workspace Layout

```
Hexacore/
├── Cargo.toml              # Workspace root
├── clikit/src/
├── agentkit/src/
├── evalkit/src/
├── taskkit/src/
├── configkit/src/
├── authkit/src/
├── cachekit/src/
├── logkit/src/
├── tracingkit/src/
├── metrickit/src/
└── eventkit/src/
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Workspace build (cold) | < 60s |
| Workspace build (incremental) | < 5s |
| Test suite | < 30s |
| Clippy pass | 0 warnings |
| Individual kit size | < 10KB compiled |
| Zero unsafe | All kits |

## Quality Gates

- `cargo build --workspace` — clean build
- `cargo test --workspace` — all tests pass
- `cargo clippy --workspace -- -D warnings` — zero warnings
- `cargo fmt --check` — formatted
- All public types implement `Debug` + `Clone`
- Domain layer has zero external dependencies

---

## Absorb addendum — 2026-06-20

The cargo-generate target at this subfolder was assembled on 2026-06-20
by absorbing two upstream sources per the `DOMAIN_ROLES.md`
designation of HexaKit as the canonical scaffolding/templates owner:

| Source | Layer | Files contributed |
|--------|-------|-------------------|
| `KooshaPari/pheno-cargo-template` | L0 | `Cargo.toml`, `src/lib.rs`, `Taskfile.yml`, `justfile`, `LICENSE-{MIT,APACHE}`, `.github/workflows/ci.yml`, `README.md`, `SPEC.md`, `llms.txt`, `AGENTS.md`, `WORKLOG.md`, `CHANGELOG.md`; and cargo-generate liquid fragments `template/Cargo.toml`, `template/src/lib.rs`, `template/AGENTS.md.additions`, `template/PREDICTIVE.md`, `template/PROMOTION.md`, `template/CONTRIBUTING.md`, `template/.predict.yaml`, `template/.drift-detector.yaml`, `template/.framework-lint.yaml`, `template/.github/workflows/predictive-dry-check.yml` (now under `_partials/`). |
| `KooshaPari/Apisync` (commit `d981353`) | L0 | `templates/hexagonal/src/sentry_config.rs`, `.clippy.toml`, `rust-toolchain.toml`; plus the v8 governance pre-fill (Apisync-only `clippy.toml` rules; FR-APISYNC-SENTRY-001 starter). |

After this absorb:

- The single cargo-generate target is `KooshaPari/HexaKit/templates/hexagon/rust/`.
- `KooshaPari/pheno-cargo-template` is deprecated and replaced with a
  redirect README pointing here.
- All absorbed files preserve the dual MIT/Apache-2.0 license.

References: `DOMAIN_ROLES.md` (HexaKit = canonical scaffolding owner),
audit commit of 2026-06-19 (`Apisync d981353`).

---

## Absorbed: pheno-cargo-template (2026-06-20)

This spec gains the following absorbed capabilities from `KooshaPari/pheno-cargo-template`:

- `sentry_config.rs` module — Sentry initialization with feature-gated `sentry` flag
- `tracing` integration with feature flag
- Liquid-fragment cargo-generate partials (preserved under `_partials/`)
- Predictive dry-check workflow (`.github/workflows/predictive-dry-check.yml`)
- Drift-detector & framework-lint configurations
- AgilePlus 001-core-setup spec preserved under `_partials/.agileplus/specs/001-core-setup/`

### Source lineage

- `pheno-cargo-template/Cargo.toml` (root) → `Cargo.toml` (sentry/tracing feature flags retained)
- `pheno-cargo-template/src/lib.rs` → `src/lib.rs` (sentr_ accessors preserved)
- `pheno-cargo-template/templates/hexagonal/src/sentry_config.rs` → `src/sentry_config.rs`
- `pheno-cargo-template/template/*` → `_partials/*` (cargo-generate liquid fragments)
- `pheno-cargo-template/templates/hexagonal/.agileplus/*` → `_partials/.agileplus/*`

### Status

Source repo `KooshaPari/pheno-cargo-template` is preserved on GitHub as a redirect (deprecation window).
