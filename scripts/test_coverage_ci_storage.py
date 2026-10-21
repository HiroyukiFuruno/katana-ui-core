from __future__ import annotations

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[1]
SCRIPT = REPO / "scripts/coverage/prepare-ci-storage.sh"
CONTAINER_ENTRYPOINT = REPO / "scripts/coverage/run-in-container.sh"
RELEASE_PREFLIGHT = REPO / ".github/workflows/release-preflight.yml"
STRICT_COVERAGE = REPO / "scripts/run-strict-coverage.sh"


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

    def test_release_preflight_keeps_failed_coverage_logs(self) -> None:
        source = RELEASE_PREFLIGHT.read_text(encoding="utf-8")
        self.assertIn('COVERAGE_TEST_THREADS: "2"', source)
        self.assertIn(
            "KUC_COVERAGE_LOG_DIR: ${{ runner.temp }}/kuc-coverage-test-logs",
            source,
        )
        self.assertIn("- name: Upload failed coverage test logs", source)
        self.assertIn("if: failure() && steps.release_check.outcome == 'failure'", source)
        self.assertIn(
            "path: ${{ runner.temp }}/kuc-coverage-test-logs",
            source,
        )

    def test_failure_artifact_keeps_coverage_reports_and_storage_state(self) -> None:
        source = CONTAINER_ENTRYPOINT.read_text(encoding="utf-8")
        self.assertIn("kuc-workspace-coverage-summary.json", source)
        self.assertIn("kuc-workspace-coverage.lcov", source)
        self.assertIn("kuc-workspace-coverage-profile-v3.sha256.strict-state", source)
        self.assertIn("failure-metadata.txt", source)

    def test_report_failures_write_phase_specific_diagnostics(self) -> None:
        source = STRICT_COVERAGE.read_text(encoding="utf-8")
        self.assertIn(
            'coverage_diagnostics_dir="${KUC_COVERAGE_LOG_DIR:-${coverage_target_dir}/kuc-coverage-diagnostics}"',
            source,
        )
        self.assertIn("record_coverage_report_failure()", source)
        self.assertIn("run_coverage_report()", source)
        self.assertIn('run_coverage_report "summary-json"', source)
        self.assertIn('run_coverage_report "lcov"', source)
        self.assertIn("phase=%s", source)
        self.assertIn("exit_status=%s", source)
        self.assertIn("target_profraw_count=%s", source)
        self.assertIn('df -Pk "${coverage_storage_dir}"', source)

    def test_report_failure_keeps_exit_status_and_metadata(self) -> None:
        source = STRICT_COVERAGE.read_text(encoding="utf-8")
        helper_start = source.index("record_coverage_report_failure() {")
        helper_end = source.index("\ninvalidate_coverage_profile()", helper_start)
        helpers = source[helper_start:helper_end]
        with tempfile.TemporaryDirectory() as temporary_directory:
            temporary_root = Path(temporary_directory)
            storage_dir = temporary_root / "storage"
            target_dir = storage_dir / "llvm-cov-target"
            diagnostics_dir = temporary_root / "diagnostics"
            target_dir.mkdir(parents=True)
            diagnostics_dir.mkdir()
            shell_test = temporary_root / "report-failure.sh"
            shell_test.write_text(
                "set -euo pipefail\n"
                f"coverage_storage_dir={shlex.quote(str(storage_dir))}\n"
                f"coverage_target_dir={shlex.quote(str(target_dir))}\n"
                f"coverage_diagnostics_dir={shlex.quote(str(diagnostics_dir))}\n"
                'coverage_report_path="${coverage_storage_dir}/kuc-workspace-coverage-summary.json"\n'
                'coverage_lcov_path="${coverage_storage_dir}/kuc-workspace-coverage.lcov"\n'
                'coverage_profile_path="${coverage_storage_dir}/kuc-workspace-coverage-profile-v3.sha256"\n'
                'coverage_strict_state_path="${coverage_profile_path}.strict-state"\n'
                f"{helpers}\n"
                "run_cargo() {\n"
                '  printf "%s\\n" "forced report failure" >&2\n'
                "  return 73\n"
                "}\n"
                "set +e\n"
                'run_coverage_report "summary-json" -p katana-ui-core\n'
                "status=$?\n"
                "set -e\n"
                '[[ "${status}" -eq 73 ]]\n'
                'grep -Fx "phase=summary-json" "${coverage_diagnostics_dir}/coverage-report-summary-json-failure.txt"\n'
                'grep -Fx "exit_status=73" "${coverage_diagnostics_dir}/coverage-report-summary-json-failure.txt"\n'
                'grep -F "forced report failure" "${coverage_diagnostics_dir}/coverage-report-summary-json.log"\n',
                encoding="utf-8",
            )
            result = subprocess.run(
                ["bash", str(shell_test)],
                capture_output=True,
                text=True,
                check=False,
            )
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()
