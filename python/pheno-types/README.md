# pheno-types

Shared Python type definitions for the Phenotype SDK, absorbed from the archived
[KooshaPari/phenoTypes](https://github.com/KooshaPari/phenoTypes) repository.

## Modules

| Module | Description |
|--------|-------------|
| `task` | Task IDs, states, and result types |
| `skill` | Skill manifest and input/output models |
| `research` | Research reports, evidence, citations, confidence scores |
| `schemas` | JSON Schema export registry |
| `legacy` | Legacy TypedDict state types (RFQ, Order, Shipping) |

## Install

```bash
pip install -e pheno-types
```

## Migration note

See [docs/history/phenoTypes.md](../docs/history/phenoTypes.md) for archive absorption details.
