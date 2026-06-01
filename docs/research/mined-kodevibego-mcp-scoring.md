# KodeVibeGo Governance Migration

**Source:** [KodeVibeGo](https://github.com/KooshaPari/KodeVibeGo) (deprecated Go implementation)  
**Successors:** [KodeVibe](https://github.com/KooshaPari/KodeVibe) (Shell CLI) + HexaKit (this repo)  
**Extracted:** 2026-05-31

---

## Overview

KodeVibeGo provided a Go-based "code quality guardian" with a plugin registry of vibe checkers, advanced weighted scoring, and MCP hooks for AI-driven fix loops. These patterns are now distributed across KodeVibe (config + CLI) and HexaKit (governance enforcement).

---

## Checker Registry Pattern

**Source:** `pkg/vibes/registry.go`

```go
type Checker interface {
    Check(ctx context.Context, files []string) ([]models.Issue, error)
    Name() string
    Type() models.VibeType
    Configure(config models.VibeConfig) error
    Supports(filename string) bool
}

type Registry struct {
    checkers map[models.VibeType]Checker
}
```

**HexaKit implementation:** `crates/phenotype-compliance-scanner/src/lib.rs`

| Go VibeType | Rust Struct | Rule IDs |
|-------------|-------------|----------|
| `security` | `KodeVibeCategoryConfig` (level: strict) | Future: KODEVIBE-SEC-* |
| `code` | `KodeVibeCategoryConfig` (max_function_length) | KODEVIBE-001+ |
| `performance` | `KodeVibeCategoryConfig` (max_bundle_size) | Future |
| `file` | `KodeVibeCategoryConfig` (level: strict) | Future |
| `git` | `KodeVibeCategoryConfig` (min_commit_message_length) | Future |
| `dependency` | `KodeVibeCategoryConfig` (check_vulnerabilities) | Future |
| `documentation` | `KodeVibeCategoryConfig` (disabled default) | Future |

Built-in rules today:

- `KODEVIBE-001` — no `console.log` in committed code
- `KODEVIBE-002` — TODO/FIXME/HACK comments flagged

Extending: implement `ComplianceRule` trait and register in `default_kodevibe_rules()`.

---

## Advanced Scoring Engine

**Source:** `pkg/scoring/advanced_scoring.go`

### Default Weights

| Category | Weight |
|----------|--------|
| security | 0.25 |
| performance | 0.20 |
| maintainability | 0.20 |
| readability | 0.15 |
| testing | 0.10 |
| documentation | 0.05 |
| complexity | 0.05 |

### Grade Thresholds

| Grade | Score Range |
|-------|-------------|
| A | 90–100 |
| B | 80–89 |
| C | 70–79 |
| D | 60–69 |
| F | < 60 |

### Penalties

| Condition | Penalty |
|-----------|---------|
| High severity issues | -15.0 |
| Critical vulnerabilities | -25.0 |
| Poor test coverage | -10.0 |
| High complexity | -8.0 |
| Missing documentation | -5.0 |
| Performance bottlenecks | -12.0 |

### Bonuses

| Condition | Bonus |
|-----------|-------|
| Excellent test coverage | +5.0 |
| Comprehensive docs | +3.0 |
| Clean architecture | +4.0 |
| Security best practices | +6.0 |
| Performance optimized | +4.0 |
| Consistent style | +2.0 |

### Trend Analysis

Tracks `HistoricalScore` entries with momentum and trend direction (`improving` | `declining` | `stable`). Used for CI gate trending in quality-gate workflows.

**HexaKit integration:** Quality gates in `.github/workflows/quality-gate.yml` and `gate-check.yml` should reference these thresholds when evaluating KodeVibe scan output.

---

## MCP Context Payload

**Source:** `pkg/mcp/mcp.go`

For AI agent fix loops, attach scan context to MCP requests:

```go
type MCPContext struct {
    ProjectPath    string
    Language       string
    Framework      string
    ScanResults    *models.ScanResult
    Issues         []models.Issue
    QualityTargets *QualityTargets
    AIInstructions string
}

type QualityTargets struct {
    MinScore        float64
    MaxIssues       int
    RequiredGrade   string
    FocusAreas      []string
    CustomRules     map[string]string
    AIOptimizations []AIOptimization
}
```

### MCP Methods

| Method | Purpose |
|--------|---------|
| `analyze_code_quality` | Send scan results + targets for AI analysis |
| `suggest_fixes` | Return `AIOptimization` with `FileChange` suggestions |
| `validate_fix` | Re-scan after AI-applied fix |

### AIOptimization Shape

```json
{
  "type": "security",
  "description": "Replace hardcoded secret with env var",
  "impact": "high",
  "confidence": 0.92,
  "suggestion": "Use os.Getenv('API_KEY')",
  "file_changes": [{
    "file_path": "src/config.go",
    "line_start": 42,
    "line_end": 42,
    "original_code": "apiKey := \"sk-...\"",
    "suggested_code": "apiKey := os.Getenv(\"API_KEY\")",
    "reasoning": "Hardcoded secrets fail security vibe"
  }]
}
```

**HexaKit integration:** PhenoMCP tools should accept this payload shape when wrapping KodeVibe scan output for agent consumption.

---

## Agent Quick Endpoints

**Source:** `pkg/server/server.go`

Minimal HTTP endpoints for CI agents (daemon mode):

| Endpoint | Response |
|----------|----------|
| `GET /quick` | `{ "score": 85.2, "grade": "B", "issues": 3, "top_issues": [...] }` |
| `GET /status/compact` | `{ "healthy": true, "last_scan": "ISO8601" }` |
| `GET /metrics` | Prometheus text format |

Shell CLI equivalent: `kodevibe scan --format json`.

WebSocket endpoint at `/ws` streams live scan progress during watch mode.

---

## Migration Checklist (100% Moved)

| Component | Status |
|-----------|--------|
| Checker registry interface | ✅ `phenotype-compliance-scanner` |
| KodeVibe rule set defaults | ✅ `KodeVibeRuleSet` in lib.rs |
| Scoring weights/thresholds | ✅ Documented here |
| MCP context types | ✅ Documented here |
| Agent quick endpoints | ✅ Documented in KodeVibe schema |
| HTML report templates | ✅ Reference pattern only |
| VS Code extension | ⬜ Dropped (out of scope) |

---

## References

- [KodeVibe Config Schema](https://github.com/KooshaPari/KodeVibe/blob/main/docs/kodevibe-config-schema.md)
- [KodeVibeGo GitHub (archived)](https://github.com/KooshaPari/KodeVibeGo)
- `crates/phenotype-compliance-scanner/src/lib.rs`
