from __future__ import annotations

import os
from pathlib import Path
import subprocess
import unittest


REPO = Path(__file__).resolve().parents[1]
SCRIPT = REPO / "scripts/coverage/prepare-ci-storage.sh"


class CoverageCiStorageTests(unittest.TestCase):
    def run_guard(self, **settings: str) -> subprocess.CompletedProcess[str]:
        environment = dict(os.environ)
        for name in (
            "KUC_COVERAGE_EPHEMERAL_CLEANUP", "GITHUB_ACTIONS",
            "RUNNER_ENVIRONMENT", "RUNNER_OS", "GITHUB_WORKSPACE", "CARGO_TARGET_DIR",
        ):
            environment.pop(name, None)
        environment.update(settings)
        return subprocess.run(
            ["bash", str(SCRIPT), str(REPO)], cwd=REPO, env=environment,
            capture_output=True, text=True, check=False,
        )

    def test_default_does_not_clean(self) -> None:
        result = self.run_guard()
        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout, "")
        self.assertEqual(result.stderr, "")

    def test_invalid_flag_is_rejected(self) -> None:
        result = self.run_guard(KUC_COVERAGE_EPHEMERAL_CLEANUP="invalid")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must be 0 or 1", result.stderr)

    def test_local_cleanup_is_rejected(self) -> None:
        result = self.run_guard(KUC_COVERAGE_EPHEMERAL_CLEANUP="1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("GitHub-hosted Linux", result.stderr)

    def test_custom_target_is_rejected(self) -> None:
        result = self.run_guard(
            KUC_COVERAGE_EPHEMERAL_CLEANUP="1", GITHUB_ACTIONS="true",
            RUNNER_ENVIRONMENT="github-hosted", RUNNER_OS="Linux",
            CARGO_TARGET_DIR="/tmp/unrelated-build",
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("default target directory", result.stderr)

    def test_other_workspace_is_rejected(self) -> None:
        result = self.run_guard(
            KUC_COVERAGE_EPHEMERAL_CLEANUP="1", GITHUB_ACTIONS="true",
            RUNNER_ENVIRONMENT="github-hosted", RUNNER_OS="Linux",
            GITHUB_WORKSPACE="/tmp/unrelated-repository",
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("current GitHub workspace", result.stderr)


if __name__ == "__main__":
    unittest.main()
