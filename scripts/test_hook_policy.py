#!/usr/bin/env python3
import subprocess
import tempfile
import os
import unittest
import shutil
from pathlib import Path


def run(command: list[str], cwd: Path, input_text: str | None = None, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    current_env = os.environ.copy()
    if env:
        current_env.update(env)

    return subprocess.run(
        command,
        cwd=cwd,
        input=input_text,
        text=True,
        check=False,
        capture_output=True,
        env=current_env,
    )


def git(*args: str, cwd: Path, input_text: str | None = None, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return run(["git", *args], cwd=cwd, input_text=input_text, env=env)


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


SOURCE_ROOT = Path(__file__).resolve().parents[1]


def install_hooks(repo: Path, evidence: str = "") -> dict[str, str]:
    shutil.copytree(SOURCE_ROOT / ".githooks", repo / ".githooks")
    shutil.copytree(SOURCE_ROOT / "scripts" / "hooks", repo / "scripts" / "hooks")
    for path in (repo / ".githooks").iterdir():
        path.chmod(0o755)
    for path in (repo / "scripts" / "hooks").iterdir():
        path.chmod(0o755)
    bin_dir = repo / "test-bin"
    bin_dir.mkdir()
    write_text(
        bin_dir / "gh",
        "#!/usr/bin/env bash\nprintf '%s\\n' \"${FAKE_ISSUE_EVIDENCE:-}\"\n",
    )
    (bin_dir / "gh").chmod(0o755)
    write_text(bin_dir / "just", "#!/usr/bin/env bash\nexit 0\n")
    (bin_dir / "just").chmod(0o755)
    return {
        "PATH": f"{bin_dir}:{os.environ['PATH']}",
        "FAKE_ISSUE_EVIDENCE": evidence,
    }


class KucHookPolicyTest(unittest.TestCase):
    def test_existing_repository_hook_runs_before_dispatcher_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp) / "repo"
            repo.mkdir()
            git("init", "-b", "master", cwd=repo)
            env = install_hooks(repo)
            order = repo / "hook-order"
            write_text(
                repo / "test-bin" / "just",
                "#!/usr/bin/env bash\nprintf 'repository\\n' >> \"$HOOK_ORDER_LOG\"\n",
            )
            (repo / "test-bin" / "just").chmod(0o755)
            write_text(
                repo / "scripts" / "hooks" / "kuc-hook-dispatcher.sh",
                "#!/usr/bin/env bash\nprintf 'dispatcher\\n' >> \"$HOOK_ORDER_LOG\"\n",
            )
            (repo / "scripts" / "hooks" / "kuc-hook-dispatcher.sh").chmod(0o755)

            result = run(
                [str(repo / ".githooks" / "pre-commit")],
                cwd=repo,
                env={**env, "HOOK_ORDER_LOG": str(order)},
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(order.read_text(encoding="utf-8").splitlines(), ["repository", "dispatcher"])

    def test_commit_msg_requires_issue_reference_on_non_default_branch(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            repo = root / "repo"
            repo.mkdir()
            git("init", "-b", "master", cwd=repo)
            git("config", "user.name", "ci", cwd=repo)
            git("config", "user.email", "ci@example.com", cwd=repo)
            write_text(repo / "README.md", "a")
            git("add", "README.md", cwd=repo)
            git("commit", "-m", "initial commit", cwd=repo)

            git("checkout", "-b", "feature/test", cwd=repo)
            env = install_hooks(repo)
            message = repo / ".git" / "COMMIT_EDITMSG"
            message.write_text("update docs", encoding="utf-8")

            result = run(
                [str(repo / ".githooks" / "commit-msg"), str(message)],
                cwd=repo,
                input_text=None,
                env=env,
            )

            self.assertNotEqual(
                result.returncode,
                0,
                result.stdout
                + result.stderr
                + git("show", "--stat", "--oneline", "HEAD", cwd=repo).stdout,
            )

            write_text(message, "feat: add docs #12",)
            result_ok = run(
                [str(repo / ".githooks" / "commit-msg"), str(message)],
                cwd=repo,
                env=env,
            )
            self.assertEqual(result_ok.returncode, 0, result_ok.stdout + result_ok.stderr)

    def test_commit_msg_requires_dependency_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            repo = root / "repo"
            repo.mkdir()
            git("init", "-b", "master", cwd=repo)
            git("config", "user.name", "ci", cwd=repo)
            git("config", "user.email", "ci@example.com", cwd=repo)

            write_text(repo / "Cargo.toml", "[workspace]\n")
            git("add", "Cargo.toml", cwd=repo)
            git("commit", "-m", "initial commit", cwd=repo)
            git("checkout", "-b", "feature/deps", cwd=repo)
            evidence = "upstream published version API migration manifest lock verification"
            env = install_hooks(repo, evidence)

            write_text(repo / "Cargo.toml", "[workspace]\nversion = '0.1.0'\n")
            git("add", "Cargo.toml", cwd=repo)
            msg = repo / "tmpmsg"
            msg.write_text("feat: bump dependency", encoding="utf-8")

            result = run(
                [str(repo / ".githooks" / "commit-msg"), str(msg)],
                cwd=repo,
                env=env,
            )
            self.assertNotEqual(result.returncode, 0)

            msg.write_text(
                "feat: bump dependency for #21",
                encoding="utf-8",
            )
            result_ok = run(
                [str(repo / ".githooks" / "commit-msg"), str(msg)],
                cwd=repo,
                env=env,
            )
            self.assertEqual(result_ok.returncode, 0, result_ok.stdout + result_ok.stderr)

    def test_pre_push_rejects_dependency_update_without_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            repo = root / "repo"
            repo.mkdir()
            git("init", "-b", "master", cwd=repo)
            git("config", "user.name", "ci", cwd=repo)
            git("config", "user.email", "ci@example.com", cwd=repo)
            write_text(repo / "README.md", "a")
            git("add", "README.md", cwd=repo)
            git("commit", "-m", "initial commit", cwd=repo)

            write_text(repo / "Cargo.toml", "[workspace]\n")
            git("add", "Cargo.toml", cwd=repo)
            git("commit", "-m", "#21 initial release", cwd=repo)
            git("checkout", "-b", "feature/push", cwd=repo)
            env = install_hooks(repo)

            write_text(repo / "Cargo.lock", "[metadata]\n")
            git("add", "Cargo.lock", cwd=repo)
            git("commit", "-m", "update lock file", cwd=repo)

            commit_sha = git("rev-parse", "HEAD", cwd=repo).stdout.strip()
            parent = git("rev-parse", "HEAD~1", cwd=repo).stdout.strip()
            push_line = f"refs/heads/feature/push {commit_sha} refs/heads/master {parent}\n"
            self.assertEqual(run(["bash", "-c", "cat"], cwd=repo, input_text=push_line).stdout, push_line)

            result = run(
                [str(repo / ".githooks" / "pre-push"), "origin", "https://github.com/example/repo.git"],
                cwd=repo,
                input_text=push_line,
                env={**env, "KUC_PUSH_CONFIRMED": "1", "KUC_HOOK_TRACE": "1"},
            )
            self.assertNotEqual(
                result.returncode,
                0,
                result.stdout
                + result.stderr
                + git("show", "--stat", "--oneline", "HEAD", cwd=repo).stdout,
            )

            git("commit", "--amend", "-m", "feat: update lock file #21", cwd=repo)
            commit_sha = git("rev-parse", "HEAD", cwd=repo).stdout.strip()
            parent = git("rev-parse", "HEAD~1", cwd=repo).stdout.strip()

            result_ok = run(
                [str(repo / ".githooks" / "pre-push"), "origin", "https://github.com/example/repo.git"],
                cwd=repo,
                input_text=f"refs/heads/feature/push {commit_sha} refs/heads/master {parent}\n",
                env={**env, "KUC_PUSH_CONFIRMED": "1", "FAKE_ISSUE_EVIDENCE": "upstream published version API migration manifest lock verification"},
            )
            self.assertEqual(result_ok.returncode, 0, result_ok.stdout + result_ok.stderr)


if __name__ == "__main__":
    unittest.main()
