#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import tempfile
import unittest
from pathlib import Path

import os
import sys


class ReleaseCleanupFixture:
    def __init__(self, tmpdir: Path) -> None:
        self.tmpdir = tmpdir
        self.remote = self.tmpdir / "remote.git"
        self.source = self.tmpdir / "source"
        self.fake_bin = self.tmpdir / "fake-bin"
        self.fake_bin.mkdir()
        self._configure_fake_gh()

    def _run(self, cwd: Path, *args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [*args],
            cwd=cwd,
            check=True,
            text=True,
            capture_output=True,
        )

    def _configure_fake_gh(self) -> None:
        script = self.fake_bin / "gh"
        script.write_text(
            "#!/usr/bin/env sh\n"
            'if [ "$1" = "release" ] && [ "$2" = "view" ]; then\n'
            "  exit 0\n"
            "fi\n"
            'if [ "$1" = "pr" ] && [ "$2" = "list" ]; then\n'
            '  if [ "${FAKE_GH_PR_LIST_FAIL:-0}" = "1" ]; then exit 1; fi\n'
            '  case " $* " in\n'
            '    *"--head ${FAKE_GH_MERGED_HEAD:-__none__} "*) printf \'[{"number":1,"mergedAt":"2026-08-31T00:00:00Z","baseRefName":"master","headRefOid":"%s"}]\\n\' "${FAKE_GH_MERGED_OID:-stale}" ;;\n'
            '    *) printf \'[]\\n\' ;;\n'
            "  esac\n"
            "  exit 0\n"
            "fi\n"
            "exit 1\n",
            encoding="utf-8",
        )
        os.chmod(script, 0o755)

    def _write(self, path: Path, text: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")

    def _configure_git(self, repo: Path) -> None:
        self._run(repo, "git", "config", "user.name", "Test Runner")
        self._run(repo, "git", "config", "user.email", "test@example.com")

    def setup_repository(self) -> Path:
        self._run(self.tmpdir, "git", "init", str(self.source))
        self._configure_git(self.source)
        self._write(self.source / "README.md", "base\n")
        self._run(self.source, "git", "add", "README.md")
        self._run(self.source, "git", "commit", "-m", "initial commit")
        self._run(self.tmpdir, "git", "init", "--bare", str(self.remote))
        self._run(self.source, "git", "remote", "add", "origin", str(self.remote))
        self._run(self.source, "git", "push", "-u", "origin", "master")

        local = self.tmpdir / "local"
        self._run(self.tmpdir, "git", "clone", str(self.remote), str(local))
        self._configure_git(local)
        self._run(local, "git", "remote", "set-head", "origin", "-a")
        return local

    def create_release_branch(
        self,
        repo: Path,
        name: str,
        merged_into_default: bool,
        dirty_worktree: bool = False,
        push_default: bool = True,
    ) -> None:
        self._run(repo, "git", "checkout", "-b", name, "master")
        self._write(repo / "CHANGELOG.md", f"{name}\n")
        self._run(repo, "git", "add", "CHANGELOG.md")
        self._run(repo, "git", "commit", "-m", f"{name} commit")

        if merged_into_default:
            self._run(repo, "git", "checkout", "master")
            self._run(repo, "git", "merge", "--ff-only", name)
            if push_default:
                self._run(repo, "git", "push", "--force", "origin", "master")
        else:
            self._run(repo, "git", "checkout", "master")

        self._run(repo, "git", "push", "--force", "-u", "origin", name)

        if dirty_worktree:
            worktree = self.tmpdir / f"{name}-wt"
            self._run(repo, "git", "worktree", "add", str(worktree), name)
            Path(worktree / "dirty.txt").write_text("dirty", encoding="utf-8")

    def create_squash_merged_release_branch(self, repo: Path, name: str) -> None:
        self._run(repo, "git", "checkout", "-b", name, "master")
        self._write(repo / "CHANGELOG.md", f"{name}\n")
        self._run(repo, "git", "add", "CHANGELOG.md")
        self._run(repo, "git", "commit", "-m", f"{name} commit")
        self._run(repo, "git", "checkout", "master")
        self._run(repo, "git", "merge", "--squash", name)
        self._run(repo, "git", "commit", "-m", f"squash merge {name}")
        self._run(repo, "git", "push", "--force", "origin", "master")
        self._run(repo, "git", "push", "--force", "-u", "origin", name)

    def run_cleanup(self, repo: Path, version: str) -> subprocess.CompletedProcess[str]:
        env = os.environ.copy()
        env["GITHUB_REPOSITORY"] = "test-org/test-repo"
        env["PATH"] = f"{self.fake_bin}{os.pathsep}{env['PATH']}"
        script = Path(__file__).with_name("release") / "cleanup-release-branches.py"
        return subprocess.run(
            [sys.executable, str(script), "--version", version],
            cwd=repo,
            text=True,
            env=env,
            capture_output=True,
        )

    def run_cleanup_with_env(
        self, repo: Path, version: str, **extra_env: str
    ) -> subprocess.CompletedProcess[str]:
        env = os.environ.copy()
        env.update(extra_env)
        env["GITHUB_REPOSITORY"] = "test-org/test-repo"
        env["PATH"] = f"{self.fake_bin}{os.pathsep}{env['PATH']}"
        script = Path(__file__).with_name("release") / "cleanup-release-branches.py"
        return subprocess.run(
            [sys.executable, str(script), "--version", version],
            cwd=repo,
            text=True,
            env=env,
            capture_output=True,
        )


class ReleaseCleanupTests(unittest.TestCase):
    def _repo(self) -> tuple[Path, ReleaseCleanupFixture, tempfile.TemporaryDirectory[str]]:
        temporary = tempfile.TemporaryDirectory(prefix="kuc-release-cleanup-")
        tmpdir = Path(temporary.name)
        fixture = ReleaseCleanupFixture(tmpdir)
        repo = fixture.setup_repository()
        return repo, fixture, temporary

    def test_removes_clean_merged_release_branch(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(repo, "release/v0.3.1", merged_into_default=True)
            result = fixture.run_cleanup(repo, "v0.3.2")
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("deleted local branch release/v0.3.1", result.stdout)
            self.assertIn("deleted remote branch origin/release/v0.3.1", result.stdout)
            branches = subprocess.run(
                ["git", "branch"],
                cwd=repo,
                check=True,
                text=True,
                capture_output=True,
            ).stdout
            remotes = subprocess.run(
                ["git", "branch", "-r"],
                cwd=repo,
                check=True,
                text=True,
                capture_output=True,
            ).stdout
            self.assertNotIn("release/v0.3.1", branches)
            self.assertNotIn("origin/release/v0.3.1", remotes)
        finally:
            temporary.cleanup()

    def test_retain_dirty_release_branch(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(
                repo,
                "release/v0.3.1",
                merged_into_default=True,
                dirty_worktree=True,
                push_default=True,
            )
            result = fixture.run_cleanup(repo, "v0.3.2")
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("retain release/v0.3.1: dirty", result.stdout)
            branches = subprocess.run(
                ["git", "branch", "--list", "release/v0.3.1"],
                cwd=repo,
                check=True,
                text=True,
                capture_output=True,
            ).stdout
            self.assertIn("release/v0.3.1", branches)
        finally:
            temporary.cleanup()

    def test_retain_unmerged_release_branch(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(
                repo,
                "release/v0.3.1",
                merged_into_default=False,
                push_default=False,
            )
            result = fixture.run_cleanup(repo, "v0.3.2")
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("retain release/v0.3.1: unmerged", result.stdout)
            branches = subprocess.run(
                ["git", "branch", "--list", "release/v0.3.1"],
                cwd=repo,
                check=True,
                text=True,
                capture_output=True,
            ).stdout
            self.assertIn("release/v0.3.1", branches)
        finally:
            temporary.cleanup()

    def test_removes_squash_merged_release_branch_from_github_pr_status(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_squash_merged_release_branch(repo, "release/v0.3.1")
            tip = subprocess.run(
                ["git", "rev-parse", "release/v0.3.1"],
                cwd=repo,
                check=True,
                text=True,
                capture_output=True,
            ).stdout.strip()
            result = fixture.run_cleanup_with_env(
                repo,
                "v0.3.2",
                FAKE_GH_MERGED_HEAD="release/v0.3.1",
                FAKE_GH_MERGED_OID=tip,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("deleted local branch release/v0.3.1", result.stdout)
            self.assertIn("deleted remote branch origin/release/v0.3.1", result.stdout)
        finally:
            temporary.cleanup()

    def test_retain_same_named_release_branch_when_merged_pr_tip_is_stale(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_squash_merged_release_branch(repo, "release/v0.3.1")
            result = fixture.run_cleanup_with_env(
                repo,
                "v0.3.2",
                FAKE_GH_MERGED_HEAD="release/v0.3.1",
                FAKE_GH_MERGED_OID="old-release-tip",
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("retain release/v0.3.1: unmerged", result.stdout)
        finally:
            temporary.cleanup()

    def test_retain_squash_merged_release_branch_when_pr_status_is_unavailable(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_squash_merged_release_branch(repo, "release/v0.3.1")
            result = fixture.run_cleanup_with_env(
                repo, "v0.3.2", FAKE_GH_PR_LIST_FAIL="1"
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("retain release/v0.3.1: unmerged", result.stdout)
        finally:
            temporary.cleanup()


if __name__ == "__main__":
    unittest.main()
