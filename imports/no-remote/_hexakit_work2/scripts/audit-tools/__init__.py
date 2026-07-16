#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""audit-tools: HexaKit governance/audit tooling (L72/L73/L74 absorption).

Three stdlib-only Python CLIs from the deprecated standalone repos:
- pheno_framework_lint (L73, ADR-048): substrate graduation & tier-convention linter
- pheno_drift_detector (L74, ADR-049): app-substrate drift detector (3-pass algorithm)
- pheno_predict (L72, ADR-047): predictive-DRY token-shingle Jaccard scanner

Usage (from HexaKit root):
    python3 scripts/audit-tools/pheno_framework_lint.py check <repo>
    python3 scripts/audit-tools/pheno_drift_detector.py scan <root>
    python3 scripts/audit-tools/pheno_predict.py scan <repo>

See SPEC-{framework-lint,drift-detector,predict}.md for full specs.
"""

__all__ = [
    "pheno_framework_lint",
    "pheno_drift_detector",
    "pheno_predict",
]

__version__ = "0.1.0"
__adr_provenance__ = ("ADR-047 (predict) + ADR-048 (framework_lint) + ADR-049 (drift_detector)")