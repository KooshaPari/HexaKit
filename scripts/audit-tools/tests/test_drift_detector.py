"""
Smoke tests for pheno-drift-detector.

These tests cover the 3 bucket-detection rules + the scoring function + the
output renderers, plus 5 subprocess-level end-to-end smoke tests against
the installed console script.

Per ADR-023 Rule 3.1, target coverage for a pheno-*-lib is 80%. The tests
below cover the public API surface; deeper coverage (false-positive rejection,
edge cases in regex matching across languages) is deferred to follow-up P1 work.
"""
from __future__ import annotations

import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

# ---------------------------------------------------------------------------
# Module loader — the source is a single-file script, not a package, so we
# import it as a module by file path.
# ---------------------------------------------------------------------------

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "pheno_drift_detector.py"

_spec = importlib.util.spec_from_file_location("pheno_drift_detector", SRC)
assert _spec and _spec.loader, f"could not load {SRC}"
_mod = importlib.util.module_from_spec(_spec)
sys.modules["pheno_drift_detector"] = _mod
_spec.loader.exec_module(_mod)

detect_buckets = _mod.detect_buckets
find_capability_dirs = _mod.find_capability_dirs
score_drift = _mod.score_drift
render_json = _mod.render_json
render_md = _mod.render_md
render_gh_issues = _mod.render_gh_issues
PAUSED_APPS = _mod.PAUSED_APPS
CONDITIONAL_APPS = _mod.CONDITIONAL_APPS
CAPSTONE_APPS = _mod.CAPSTONE_APPS
DriftHit = _mod.DriftHit
Capability = _mod.Capability
DRIFT_THRESHOLD = _mod.DRIFT_THRESHOLD


# ---------------------------------------------------------------------------
# In-process unit tests (12 total)
# ---------------------------------------------------------------------------

class TestDetectBuckets(unittest.TestCase):
    """3 bucket-detection rules (per ADR-023)."""

    def test_paused_bucket(self):
        # A PAUSED repo name → ['paused']
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "focalpoint"
            root.mkdir()
            buckets = detect_buckets(root)
        self.assertIn("paused", buckets)

    def test_conditional_bucket(self):
        # A CONDITIONAL repo name → ['conditional']
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "HwLedger"
            root.mkdir()
            buckets = detect_buckets(root)
        self.assertIn("conditional", buckets)

    def test_capstone_bucket(self):
        # A CAPSTONE repo name → ['capstone']
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "AtomsBot"
            root.mkdir()
            buckets = detect_buckets(root)
        self.assertIn("capstone", buckets)

    def test_unknown_bucket(self):
        # An unknown repo name → []
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "pheno-otel"  # not a PAUSED/CONDITIONAL/CAPSTONE app
            root.mkdir()
            buckets = detect_buckets(root)
        self.assertEqual(buckets, [])

    def test_glob_paused(self):
        # *fitness* is a glob in PAUSED_APPS → matches e.g. "myfitness"
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "myfitness"
            root.mkdir()
            buckets = detect_buckets(root)
        self.assertIn("paused", buckets)


class TestScoreDrift(unittest.TestCase):
    """Scoring function + substrate placement (per ADR-043 §4)."""

    def test_zero_caps_returns_zero(self):
        score, target, rationale = score_drift([])
        self.assertEqual(score, 0.0)
        self.assertEqual(target, "")

    def test_one_cap_returns_zero(self):
        # Need ≥ 2 caps for non-zero score
        cap = Capability(
            dir="apps/foo", file_count=10, total_bytes=20000,
            has_port=True, has_adapter=True, has_test=True, ports=["trait X"],
        )
        score, target, rationale = score_drift([cap])
        self.assertEqual(score, 0.0)

    def test_two_caps_with_ports_and_adapters_is_framework(self):
        # ≥ 2 caps + ≥ 2 ports + ≥ 2 adapters → phenotype-*-framework
        caps = [
            Capability(dir="a", file_count=10, total_bytes=20000,
                       has_port=True, has_adapter=True, has_test=True, ports=["trait A"]),
            Capability(dir="b", file_count=10, total_bytes=20000,
                       has_port=True, has_adapter=True, has_test=True, ports=["trait B"]),
        ]
        score, target, rationale = score_drift(caps)
        self.assertGreater(score, 1.5)
        self.assertEqual(target, "phenotype-*-framework")

    def test_two_caps_with_port_only_is_lib(self):
        # ≥ 2 caps + ≥ 1 port but no adapters → pheno-*-lib
        caps = [
            Capability(dir="a", file_count=10, total_bytes=20000,
                       has_port=True, has_adapter=False, has_test=True, ports=["trait A"]),
            Capability(dir="b", file_count=10, total_bytes=20000,
                       has_port=True, has_adapter=False, has_test=False, ports=["trait B"]),
        ]
        score, target, rationale = score_drift(caps)
        self.assertGreater(score, 1.0)
        self.assertEqual(target, "pheno-*-lib")


class TestRenderers(unittest.TestCase):
    """3 output renderers (json / md / gh-issues)."""

    def test_json_output(self):
        hit = DriftHit(
            repo="HwLedger", bucket="conditional",
            capabilities=[], drift_score=2.4,
            candidate_paths=["apps/macos"],
            target_substrate="phenotype-*-framework",
            rationale="test rationale",
            suggested_action="extract X",
        )
        out = render_json([hit])
        data = json.loads(out)
        self.assertEqual(len(data), 1)
        self.assertEqual(data[0]["repo"], "HwLedger")
        self.assertEqual(data[0]["drift_score"], 2.4)

    def test_md_output_no_hits(self):
        out = render_md([])
        self.assertIn("No drift hits", out)

    def test_gh_issues_output_empty(self):
        out = render_gh_issues([])
        self.assertEqual(out, "")


# ---------------------------------------------------------------------------
# 5 subprocess-level end-to-end smoke tests (per spec)
# ---------------------------------------------------------------------------

@unittest.skipUnless(shutil.which("pheno-drift-detector"), "console script not installed")
class TestSubprocessSmoke(unittest.TestCase):
    """End-to-end tests that invoke the installed `pheno-drift-detector` script.

    CLI surface (see pheno_drift_detector.py:393-409):
      pheno-drift-detector scan      --root PATH   (--format json|md|gh-issues; exit 0=ok, 2=hits)
      pheno-drift-detector validate  --hit HIT_JSON (--yes to confirm)
    Requires the package to be installed (`pip install -e .[test]`).
    """

    def test_01_help(self):
        # --help exits 0 and prints usage
        r = subprocess.run(
            ["pheno-drift-detector", "--help"],
            capture_output=True, text=True, timeout=30,
        )
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("usage", r.stdout.lower())
        self.assertIn("scan", r.stdout)
        self.assertIn("validate", r.stdout)

    def test_02_version_smoke(self):
        # No --version flag exists. Verify the script is executable and emits
        # a valid scan (even for an empty root) — this is the "version/runtime"
        # smoke check.
        with tempfile.TemporaryDirectory() as tmp:
            r = subprocess.run(
                ["pheno-drift-detector", "scan", "--root", tmp],
                capture_output=True, text=True, timeout=30,
            )
            # Empty root: no hits → exit 0, stdout is valid JSON
            self.assertEqual(r.returncode, 0, f"stderr={r.stderr}\nstdout={r.stdout}")
            data = json.loads(r.stdout)
            self.assertIsInstance(data, list)

    def test_03_paused_repo_no_hit(self):
        # A PAUSED repo with no Port/Adapter/Test patterns → no hit (exit 0)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "focalpoint"
            root.mkdir()
            (root / "src").mkdir()
            (root / "src" / "app.py").write_text("# no Port patterns here\n")
            r = subprocess.run(
                ["pheno-drift-detector", "scan", "--root", tmp, "--format", "json"],
                capture_output=True, text=True, timeout=30,
            )
            # No drift → exit 0
            self.assertEqual(r.returncode, 0, f"stderr={r.stderr}\nstdout={r.stdout}")
            data = json.loads(r.stdout)
            self.assertEqual(data, [])

    def test_04_capstone_repo_no_hit(self):
        # A CAPSTONE repo with no Port patterns → no hit
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "AtomsBot"
            root.mkdir()
            (root / "src").mkdir()
            (root / "src" / "capstone.py").write_text("# capstone code, no Port\n")
            r = subprocess.run(
                ["pheno-drift-detector", "scan", "--root", tmp],
                capture_output=True, text=True, timeout=30,
            )
            self.assertEqual(r.returncode, 0, f"stderr={r.stderr}\nstdout={r.stdout}")

    def test_05_json_output_format(self):
        # scan with --format json always emits a valid JSON array
        with tempfile.TemporaryDirectory() as tmp:
            # A CONDITIONAL repo with a Port pattern → 1 hit expected
            root = Path(tmp) / "HwLedger"
            root.mkdir()
            (root / "apps").mkdir()
            (root / "apps" / "macos").mkdir()
            (root / "apps" / "macos" / "lib1.py").write_text("trait Foo: pass\n")
            (root / "apps" / "macos" / "lib2.py").write_text("class XAdapter: pass\n")
            (root / "apps" / "macos" / "lib3.py").write_text("def test_x(): pass\n")
            r = subprocess.run(
                ["pheno-drift-detector", "scan", "--root", tmp, "--format", "json"],
                capture_output=True, text=True, timeout=30,
            )
            # HwLedger has 1 cap → 0 hits (need ≥ 2 caps). But the JSON is valid either way.
            self.assertIn(r.returncode, (0, 2), f"unexpected exit: {r.returncode}\nstderr={r.stderr}")
            data = json.loads(r.stdout)
            self.assertIsInstance(data, list)
            # Each report (if any) has the required fields
            for hit in data:
                self.assertIn("repo", hit)
                self.assertIn("bucket", hit)
                self.assertIn("drift_score", hit)
                self.assertIn("target_substrate", hit)


if __name__ == "__main__":
    unittest.main()
