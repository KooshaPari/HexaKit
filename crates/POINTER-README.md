# HexaKit → phenotype-router pointer (2026-09-01 audit)

HexaKit previously maintained its own `phenotype-router` v0.1.0 at
`crates/phenotype-router/`. After the 2026-09-01 audit, that crate was
deprecated and replaced with a thin re-export pointer to the standalone
canonical home.

## Canonical home

`KooshaPari/phenotype-router` (v0.2.0)

Features that the standalone has that the old HexaKit copy did NOT:
- OpenTelemetry-aware tracing (`tracing-opentelemetry` integration)
- ADR-049 (decision-routing), ADR-050 (rate-limit policy), ADR-051 (fallback chain)
- H11-shaped request/response model
- Typed `Decision` enum + `Delegate` trait
- Health/metrics endpoints
- Multi-modal payload handling

## Migration

In any HexaKit crate that currently depends on the old `phenotype-router`, replace:

```toml
phenotype-router = { path = "../phenotype-router" }
```

with:

```toml
phenotype-router = { git = "https://github.com/KooshaPari/phenotype-router", tag = "v0.2.0" }
```

(Cargo workspace members are unaffected — `crates/phenotype-router/` stays
as a re-export shim to avoid breaking the workspace manifest.)

## Removal

The pointer crate `crates/phenotype-router/` can be deleted once all
HexaKit consumer crates have migrated off the `path = "../phenotype-router"`
form. Target deletion: **2026-12-01** (90-day migration window).

## Audit

- Tier-3 P1 spec: `KooshaPari/phenotype-registry/ecosystem-consolidation/dossier/TIER3-P1-ROUTER-CLUSTER.md`
- Original HexaKit PR: `KooshaPari/HexaKit#345` (which actually ingested `phenotype-bdd` + `phenotype-compliance-scanner` from `phenoRouterMonitor` instead)
- Standalone repo: `KooshaPari/phenotype-router` (v0.2.0, 168 KB)
