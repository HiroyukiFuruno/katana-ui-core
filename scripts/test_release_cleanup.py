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
        local_env = subprocess.run(
            ["git", "rev-parse", "--local-env-vars"],
            cwd=self.tmpdir,
            check=True,
            text=True,
            capture_output=True,
        ).stdout.split()
        self._git_local_env = tuple(local_env)
        self._configure_fake_gh()

    def _git_env(self) -> dict[str, str]:
        env = os.environ.copy()
        for name in self._git_local_env:
            env.pop(name, None)
        return env

    def _run(self, cwd: Path, *args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [*args],
            cwd=cwd,
            env=self._git_env(),
            check=True,
            text=True,
            capture_output=True,
        )

    def _git_run(
        self, cwd: Path, *args: str, check: bool = True
    ) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["git", *args],
            cwd=cwd,
            env=self._git_env(),
            check=check,
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
        self._run(repo, "git", "switch", "-c", name, "master")
        self._write(repo / "CHANGELOG.md", f"{name}\n")
        self._run(repo, "git", "add", "CHANGELOG.md")
        self._run(repo, "git", "commit", "-m", f"{name} commit")

        if merged_into_default:
            self._run(repo, "git", "switch", "master")
            self._run(repo, "git", "merge", "--ff-only", name)
            if push_default:
                self._run(repo, "git", "push", "--force", "origin", "master")
        else:
            self._run(repo, "git", "switch", "master")

        self._run(repo, "git", "push", "--force", "-u", "origin", name)

        if dirty_worktree:
            self._run(repo, "git", "switch", name)
            self._write(repo / "CHANGELOG.md", f"{name} dirty\n")

    def create_squash_merged_release_branch(self, repo: Path, name: str) -> None:
        self._run(repo, "git", "switch", "-c", name, "master")
        self._write(repo / "CHANGELOG.md", f"{name}\n")
        self._run(repo, "git", "add", "CHANGELOG.md")
        self._run(repo, "git", "commit", "-m", f"{name} commit")
        self._run(repo, "git", "switch", "master")
        self._run(repo, "git", "merge", "--squash", name)
        self._run(repo, "git", "commit", "-m", f"squash merge {name}")
        self._run(repo, "git", "push", "--force", "origin", "master")
        self._run(repo, "git", "push", "--force", "-u", "origin", name)

    def run_cleanup(
        self, repo: Path, version: str, *, remote: bool = False
    ) -> subprocess.CompletedProcess[str]:
        env = self._git_env()
        env["GITHUB_REPOSITORY"] = "test-org/test-repo"
        env["PATH"] = f"{self.fake_bin}{os.pathsep}{env['PATH']}"
        script = Path(__file__).with_name("release") / "cleanup-release-branches.py"
        command = [sys.executable, str(script), "--version", version]
        if remote:
            command.append("--remote")
        return subprocess.run(
            command,
            cwd=repo,
            text=True,
            env=env,
            capture_output=True,
        )

    def run_cleanup_with_env(
        self, repo: Path, version: str, *, remote: bool = False, **extra_env: str
    ) -> subprocess.CompletedProcess[str]:
        env = self._git_env()
        env.update(extra_env)
        for name in self._git_local_env:
            env.pop(name, None)
        env["GITHUB_REPOSITORY"] = "test-org/test-repo"
        env["PATH"] = f"{self.fake_bin}{os.pathsep}{env['PATH']}"
        script = Path(__file__).with_name("release") / "cleanup-release-branches.py"
        command = [sys.executable, str(script), "--version", version]
        if remote:
            command.append("--remote")
        return subprocess.run(
            command,
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
            remote_tip = fixture._git_run(
                repo, "--git-dir", str(fixture.remote), "rev-parse", "refs/heads/release/v0.3.1"
            ).stdout.strip()
            result = fixture.run_cleanup(repo, "v0.3.2")
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("deleted local branch release/v0.3.1", result.stdout)
            self.assertNotIn("deleted remote branch origin/release/v0.3.1", result.stdout)
            branches = fixture._git_run(repo, "branch").stdout
            remotes = fixture._git_run(repo, "branch", "-r").stdout
            self.assertNotIn("release/v0.3.1", branches)
            self.assertIn("origin/release/v0.3.1", remotes)
            retained_remote_tip = fixture._git_run(
                repo, "--git-dir", str(fixture.remote), "rev-parse", "refs/heads/release/v0.3.1"
            ).stdout.strip()
            self.assertEqual(retained_remote_tip, remote_tip)
        finally:
            temporary.cleanup()

    def test_default_cleanup_keeps_remote_only_unmerged_branch_without_failure(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(
                repo,
                "release/v0.3.1",
                merged_into_default=False,
                push_default=False,
            )
            fixture._run(repo, "git", "branch", "-D", "release/v0.3.1")
            remote_tip = fixture._git_run(
                repo, "--git-dir", str(fixture.remote), "rev-parse", "refs/heads/release/v0.3.1"
            ).stdout.strip()
            result = fixture.run_cleanup(repo, "v0.3.2")
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            remotes = fixture._git_run(repo, "branch", "-r").stdout
            self.assertIn("origin/release/v0.3.1", remotes)
            retained_remote_tip = fixture._git_run(
                repo, "--git-dir", str(fixture.remote), "rev-parse", "refs/heads/release/v0.3.1"
            ).stdout.strip()
            self.assertEqual(retained_remote_tip, remote_tip)
        finally:
            temporary.cleanup()

    def test_remote_cleanup_is_explicit_and_deletes_local_and_remote_refs(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(repo, "release/v0.3.1", merged_into_default=True)
            result = fixture.run_cleanup(repo, "v0.3.2", remote=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("deleted local branch release/v0.3.1", result.stdout)
            self.assertIn("deleted remote branch origin/release/v0.3.1", result.stdout)
            refs = fixture._git_run(
                repo, "for-each-ref", "--format=%(refname)", "refs/heads/release", "refs/remotes/origin/release"
            ).stdout
            self.assertNotIn("release/v0.3.1", refs)
            self.assertEqual(
                fixture._git_run(
                    repo, "--git-dir", str(fixture.remote), "show-ref", "--verify", "--quiet", "refs/heads/release/v0.3.1", check=False
                ).returncode,
                1,
            )
        finally:
            temporary.cleanup()

    def test_removes_merged_branch_when_origin_head_tracking_ref_is_missing(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(repo, "release/v0.3.1", merged_into_default=True)
            fixture._run(
                repo,
                "git",
                "symbolic-ref",
                "--delete",
                "refs/remotes/origin/HEAD",
            )

            result = fixture.run_cleanup(repo, "v0.3.2")

            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("default branch detected: master", result.stdout)
            self.assertIn("deleted local branch release/v0.3.1", result.stdout)
            self.assertNotIn("deleted remote branch origin/release/v0.3.1", result.stdout)
            remotes = fixture._git_run(repo, "branch", "-r").stdout
            self.assertIn("origin/release/v0.3.1", remotes)
        finally:
            temporary.cleanup()

    def test_retain_dirty_release_branch(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(
                repo,
                "release/v0.3.1",
                merged_into_default=False,
                dirty_worktree=True,
                push_default=True,
            )
            result = fixture.run_cleanup(repo, "v0.3.2")
            self.assertNotEqual(result.returncode, 0)
            branches = fixture._git_run(repo, "branch", "--list", "release/v0.3.1").stdout
            self.assertIn("release/v0.3.1", branches)
            self.assertEqual(
                fixture._git_run(repo, "branch", "--show-current").stdout.strip(),
                "release/v0.3.1",
            )
            self.assertTrue(fixture._git_run(repo, "status", "--porcelain").stdout)
        finally:
            temporary.cleanup()

    def test_production_cleanup_rejects_dirty_primary_checkout(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_release_branch(
                repo,
                "release/v0.3.1",
                merged_into_default=True,
                dirty_worktree=True,
                push_default=True,
            )
            script = Path(__file__).with_name("release") / "cleanup-release-branches.py"
            probe = """
import importlib.util
import pathlib
import sys

script = pathlib.Path(sys.argv[2])
spec = importlib.util.spec_from_file_location("cleanup_release_branches", script)
if spec is None or spec.loader is None:
    raise SystemExit("failed to load cleanup module")
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
repo = pathlib.Path(sys.argv[1])
states = module.collect_branch_states(repo, "master", None)
state = states["release/v0.3.1"]
assert state.in_use and not state.clean, state
raise SystemExit(module.evaluate_and_cleanup(repo, "master", states))
"""
            result = subprocess.run(
                [sys.executable, "-c", probe, str(repo), str(script)],
                cwd=repo,
                env=fixture._git_env(),
                text=True,
                capture_output=True,
            )
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertIn(
                "retain release/v0.3.1: dirty, in-use",
                result.stdout,
                result.stdout + result.stderr,
            )
            self.assertEqual(
                fixture._git_run(repo, "branch", "--show-current").stdout.strip(),
                "release/v0.3.1",
            )
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
            branches = fixture._git_run(repo, "branch", "--list", "release/v0.3.1").stdout
            self.assertIn("release/v0.3.1", branches)
        finally:
            temporary.cleanup()

    def test_removes_squash_merged_release_branch_from_github_pr_status(self) -> None:
        repo, fixture, temporary = self._repo()
        try:
            fixture.create_squash_merged_release_branch(repo, "release/v0.3.1")
            tip = fixture._git_run(repo, "rev-parse", "release/v0.3.1").stdout.strip()
            result = fixture.run_cleanup_with_env(
                repo,
                "v0.3.2",
                FAKE_GH_MERGED_HEAD="release/v0.3.1",
                FAKE_GH_MERGED_OID=tip,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("deleted local branch release/v0.3.1", result.stdout)
            self.assertNotIn("deleted remote branch origin/release/v0.3.1", result.stdout)
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
