#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).parent / "release" / "verify-release-gate.py"
SPEC = importlib.util.spec_from_file_location("verify_release_gate", MODULE_PATH)
assert SPEC is not None
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


SHA = "a" * 40


def manifest(*, sha: str = SHA, unresolved: int = 0, preflight: str = "passed") -> dict[str, object]:
    return {
        "schema_version": 1,
        "sha": sha,
        "review_threads": {"unresolved": unresolved},
        "preflight": {"status": preflight, "sha": sha},
    }


class VerifyReleaseGateTest(unittest.TestCase):
    def test_accepts_clean_sha_bound_manifest(self) -> None:
        self.assertEqual([], MODULE.manifest_failures(manifest(), SHA))

    def test_rejects_sha_mismatch(self) -> None:
        failures = MODULE.manifest_failures(manifest(sha="b" * 40), SHA)
        self.assertTrue(any("does not match expected" in failure for failure in failures))

    def test_rejects_unresolved_review_threads(self) -> None:
        failures = MODULE.manifest_failures(manifest(unresolved=1), SHA)
        self.assertIn("release gate has 1 unresolved review thread(s)", failures)

    def test_rejects_failed_or_mismatched_preflight(self) -> None:
        failures = MODULE.manifest_failures(manifest(preflight="failed"), SHA)
        self.assertIn("release gate preflight must be passed", failures)
        failed_sha = manifest()
        failed_sha["preflight"] = {"status": "passed", "sha": "b" * 40}
        self.assertIn("release gate preflight sha does not match expected sha", MODULE.manifest_failures(failed_sha, SHA))

    def test_completion_rejects_old_state_sha(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            state = root / "state.json"
            output = root / "done.json"
            state.write_text(json.dumps({"schema_version": 1, "phase": "start", "status": "started", "sha": "b" * 40, "started_at": "now"}), encoding="utf-8")
            self.assertEqual(1, MODULE.complete_gate(state, SHA, "passed", output))
            self.assertFalse(output.exists())

    def test_start_and_complete_record_sha_bound_state(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest_path = root / "manifest.json"
            state_path = root / "state.json"
            done_path = root / "done.json"
            manifest_path.write_text(json.dumps(manifest()), encoding="utf-8")
            self.assertEqual(0, MODULE.start_gate(manifest_path, SHA, state_path))
            self.assertEqual(0, MODULE.complete_gate(state_path, SHA, "passed", done_path))
            done = json.loads(done_path.read_text(encoding="utf-8"))
            self.assertEqual("complete", done["phase"])
            self.assertEqual("passed", done["status"])
            self.assertEqual(SHA, done["sha"])
            self.assertEqual(0, done["duration_seconds"])
            self.assertEqual(0, done["review_threads_unresolved"])
            self.assertEqual("passed", done["preflight_status"])


if __name__ == "__main__":
    unittest.main()
