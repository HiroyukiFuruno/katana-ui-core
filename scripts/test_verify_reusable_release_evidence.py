#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import sys
import subprocess
import tempfile
import unittest
from datetime import datetime, timedelta, timezone
from pathlib import Path

MODULE_PATH = Path(__file__).parent / "release" / "verify-reusable-release-evidence.py"
SPEC = importlib.util.spec_from_file_location("verify_reusable_release_evidence", MODULE_PATH)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


NOW = datetime(2026, 9, 20, tzinfo=timezone.utc)
ENVIRONMENT = {
    "os": "Linux",
    "arch": "x86_64",
    "rustc": "rustc 1",
    "runner_image_os": "ubuntu22",
    "runner_image_version": "20260920.1",
    "os_release": "ubuntu:22.04",
    "xvfb_path": "/usr/bin/Xvfb",
    "ffmpeg_version": "ffmpeg version 7.1",
    "native_packages": "ffmpeg=7.1;fonts-noto-cjk=1;fonts-noto-color-emoji=1;fonts-noto-mono=1;xauth=1;xvfb=1",
    "emoji_font_sha256": "emoji",
    "mono_font_sha256": "mono",
    "coverage_image_runtime_id": "runtime-v1:sha256:" + "a" * 64,
}


def candidate(**overrides: object) -> dict[str, object]:
    value: dict[str, object] = {"schema_version": 3, "issuer": "github-actions", "workflow": ".github/workflows/release-preflight.yml", "repository": "owner/repo", "run_id": "123", "source_sha": "a" * 40, "content_digest": "digest", "environment": ENVIRONMENT, "result": "passed", "expires_at": (NOW + timedelta(hours=1)).isoformat()}
    value.update(overrides)
    return value


def source_run(**overrides: object) -> dict[str, object]:
    value: dict[str, object] = {"id": 123, "event": "workflow_dispatch", "head_branch": "master", "status": "completed", "conclusion": "success", "repository": {"full_name": "owner/repo"}, "head_sha": "a" * 40, "path": ".github/workflows/release-preflight.yml"}
    value.update(overrides)
    return value


class ReusableEvidenceTest(unittest.TestCase):
    def test_accepts_identical_trusted_ci_evidence(self) -> None:
        self.assertEqual([], MODULE.reuse_failures(candidate(), source_run(), "digest", "digest", ENVIRONMENT, "owner/repo", NOW))

    def test_rejects_untrusted_or_incomplete_source(self) -> None:
        cases = (candidate(issuer="local"), candidate(workflow="other.yml"), candidate(content_digest="other"), candidate(environment={}), candidate(expires_at=NOW.isoformat()), candidate())
        runs = (source_run(), source_run(), source_run(), source_run(), source_run(), source_run(conclusion="failure"))
        for evidence, run in zip(cases, runs, strict=True):
            self.assertTrue(MODULE.reuse_failures(evidence, run, "digest", "digest", ENVIRONMENT, "owner/repo", NOW))

    def test_rejects_mismatched_run_sha_repository_and_workflow(self) -> None:
        for run in (source_run(id=456), source_run(event="pull_request"), source_run(head_branch="release/v0.3.12"), source_run(head_sha="b" * 40), source_run(repository={"full_name": "attacker/repo"}), source_run(path=".github/workflows/other.yml"), source_run(path=".github/workflows/release-preflight.yml@refs/heads/main")):
            self.assertTrue(MODULE.reuse_failures(candidate(), run, "digest", "digest", ENVIRONMENT, "owner/repo", NOW))

    def test_rejects_self_reported_digest_that_is_not_the_source_tree_digest(self) -> None:
        self.assertTrue(
            MODULE.reuse_failures(candidate(), source_run(), "digest", "untrusted-source-digest", ENVIRONMENT, "owner/repo", NOW)
        )

    def test_rejects_runner_native_or_coverage_container_environment_drift(self) -> None:
        for key in (
            "runner_image_version",
            "os_release",
            "xvfb_path",
            "ffmpeg_version",
            "native_packages",
            "coverage_image_runtime_id",
        ):
            changed_environment = dict(ENVIRONMENT)
            changed_environment[key] = "changed"
            self.assertTrue(
                MODULE.reuse_failures(
                    candidate(environment=changed_environment),
                    source_run(),
                    "digest",
                    "digest",
                    ENVIRONMENT,
                    "owner/repo",
                    NOW,
                ),
                key,
            )

    def test_rejects_missing_coverage_container_runtime_identity(self) -> None:
        incomplete_environment = dict(ENVIRONMENT)
        del incomplete_environment["coverage_image_runtime_id"]
        self.assertTrue(
            MODULE.reuse_failures(
                candidate(environment=incomplete_environment),
                source_run(),
                "digest",
                "digest",
                ENVIRONMENT,
                "owner/repo",
                NOW,
            )
        )

    def test_content_digest_rejects_dirty_or_untracked_inputs(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            subprocess.run(("git", "init", "--quiet"), cwd=root, check=True)
            (root / "Cargo.lock").write_text("locked\n", encoding="utf-8")
            subprocess.run(("git", "add", "Cargo.lock"), cwd=root, check=True)
            subprocess.run(("git", "-c", "user.email=test@example.invalid", "-c", "user.name=test", "commit", "--quiet", "-m", "fixture"), cwd=root, check=True)
            self.assertEqual(64, len(MODULE.content_digest(root)))
            (root / "untracked-test-fixture").write_text("dirty\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "dirty tree"):
                MODULE.content_digest(root)

    def test_content_digest_at_revision_reads_the_requested_immutable_tree(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            subprocess.run(("git", "init", "--quiet"), cwd=root, check=True)
            (root / "Cargo.lock").write_text("first\n", encoding="utf-8")
            subprocess.run(("git", "add", "Cargo.lock"), cwd=root, check=True)
            subprocess.run(("git", "-c", "user.email=test@example.invalid", "-c", "user.name=test", "commit", "--quiet", "-m", "first"), cwd=root, check=True)
            first = subprocess.check_output(("git", "rev-parse", "HEAD"), cwd=root, text=True).strip()
            (root / "Cargo.lock").write_text("second\n", encoding="utf-8")
            subprocess.run(
                (
                    "git",
                    "-c",
                    "user.email=test@example.invalid",
                    "-c",
                    "user.name=test",
                    "commit",
                    "-am",
                    "second",
                    "--quiet",
                ),
                cwd=root,
                check=True,
            )
            self.assertNotEqual(MODULE.content_digest_at_revision(root, first), MODULE.content_digest_at_revision(root, "HEAD"))


if __name__ == "__main__":
    unittest.main()
