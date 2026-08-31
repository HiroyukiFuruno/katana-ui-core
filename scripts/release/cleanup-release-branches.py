#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


RELEASE_BRANCH_RE = re.compile(r"^release/v\d+\.\d+\.\d+$")


def run_git(args: list[str], *, cwd: Path, check: bool = True) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=cwd,
        text=True,
        capture_output=True,
        check=False,
    )
    if check and result.returncode != 0:
        print(result.stdout, end="")
        print(result.stderr, end="", file=sys.stderr)
        raise RuntimeError(f"git {' '.join(args)} failed with status {result.returncode}")
    return result.stdout.strip()


def run_gh(args: list[str], *, cwd: Path, check: bool = True) -> str:
    cmd = ["gh", *args]
    result = subprocess.run(
        cmd,
        cwd=cwd,
        text=True,
        capture_output=True,
        check=False,
    )
    if check and result.returncode != 0:
        print(result.stdout, end="")
        print(result.stderr, end="", file=sys.stderr)
        raise RuntimeError(f"{' '.join(cmd)} failed with status {result.returncode}")
    return result.stdout.strip()


def default_branch(repo_root: Path) -> str:
    ref = run_git(
        ["symbolic-ref", "refs/remotes/origin/HEAD"],
        cwd=repo_root,
    )
    if not ref.startswith("refs/remotes/origin/"):
        raise RuntimeError(f"unexpected origin HEAD reference: {ref}")
    return ref.rsplit("/", 1)[1]


def release_branches_local(repo_root: Path) -> set[str]:
    names = run_git(
        ["for-each-ref", "--format=%(refname:short)", "refs/heads/release/v*"],
        cwd=repo_root,
    )
    return {name for name in names.splitlines() if RELEASE_BRANCH_RE.fullmatch(name)}


def release_branches_remote(repo_root: Path) -> set[str]:
    names = run_git(
        ["for-each-ref", "--format=%(refname:short)", "refs/remotes/origin/release/v*"],
        cwd=repo_root,
    )
    return {
        name.removeprefix("origin/")
        for name in names.splitlines()
        if name.startswith("origin/") and RELEASE_BRANCH_RE.fullmatch(name.removeprefix("origin/"))
    }


def worktree_usage(repo_root: Path) -> dict[str, Path]:
    usage: dict[str, Path] = {}
    listing = run_git(["worktree", "list", "--porcelain"], cwd=repo_root).splitlines()
    current_path: Path | None = None
    for line in listing:
        if not line:
            current_path = None
            continue
        if line.startswith("worktree "):
            current_path = Path(line.split(" ", 1)[1])
        elif line.startswith("branch ") and current_path is not None:
            branch = line.removeprefix("branch ").strip()
            if branch.startswith("refs/heads/"):
                usage[branch.removeprefix("refs/heads/")] = current_path
    return usage


def is_merged(repo_root: Path, branch: str, base: str) -> bool:
    return (
        subprocess.run(
            ["git", "merge-base", "--is-ancestor", branch, base],
            cwd=repo_root,
            check=False,
        ).returncode
        == 0
    )


def github_pr_merged(
    repo_root: Path, branch: str, base: str, tip: str, repo: str | None
) -> bool:
    args = [
        "pr",
        "list",
        "--state",
        "merged",
        "--head",
        branch,
        "--base",
        base,
        "--json",
        "number,mergedAt,baseRefName,headRefOid",
    ]
    if repo:
        args.extend(["--repo", repo])
    try:
        matches = run_gh(args, cwd=repo_root)
        records = json.loads(matches)
    except (RuntimeError, ValueError, TypeError):
        return False
    if not isinstance(records, list):
        return False
    return any(
        isinstance(record, dict)
        and record.get("baseRefName") == base
        and record.get("mergedAt")
        and record.get("headRefOid") == tip
        for record in records
    )


def is_remote_merged(repo_root: Path, branch: str, base: str, repo: str | None) -> bool:
    tip = run_git(["rev-parse", f"origin/{branch}"], cwd=repo_root)
    return (
        subprocess.run(
            ["git", "merge-base", "--is-ancestor", f"origin/{branch}", f"origin/{base}"],
            cwd=repo_root,
            check=False,
        ).returncode
        == 0
    ) or github_pr_merged(repo_root, branch, base, tip, repo)


def is_branch_merged(repo_root: Path, branch: str, base: str, repo: str | None) -> bool:
    tip = run_git(["rev-parse", branch], cwd=repo_root)
    return is_merged(repo_root, branch, base) or github_pr_merged(
        repo_root, branch, base, tip, repo
    )


def worktree_is_clean(repo_root: Path, path: Path) -> bool:
    status = run_git(
        ["-C", str(path), "status", "--porcelain"],
        cwd=repo_root,
        check=False,
    )
    return status == ""


def verify_release_published(repo_root: Path, version: str, repo: str | None) -> None:
    args = ["release", "view", version]
    if repo:
        args.extend(["--repo", repo])
    run_gh(args, cwd=repo_root)


def release_tag(version: str) -> str:
    return version if version.startswith("v") else f"v{version}"


def ensure_default_ready(repo_root: Path, default: str) -> None:
    run_git(
        ["fetch", "--quiet", "--prune", "origin", "+refs/heads/*:refs/remotes/origin/*"],
        cwd=repo_root,
    )
    run_git(["switch", "--quiet", default], cwd=repo_root)
    run_git(["pull", "--ff-only", "origin", default], cwd=repo_root)


@dataclass
class BranchState:
    local: bool
    remote: bool
    in_use: bool
    clean: bool
    local_merged: bool
    remote_merged: bool


def collect_branch_states(repo_root: Path, default: str, repo: str | None) -> dict[str, BranchState]:
    local_release = release_branches_local(repo_root)
    remote_release = release_branches_remote(repo_root)
    used_worktrees = worktree_usage(repo_root)
    branches = sorted(local_release | remote_release)
    states: dict[str, BranchState] = {}

    for branch in branches:
        in_use = branch in used_worktrees
        local = branch in local_release
        remote = branch in remote_release
        clean = True
        if local:
            path = used_worktrees.get(branch)
            if path is not None:
                clean = worktree_is_clean(repo_root, path)
        local_merged = is_branch_merged(repo_root, branch, default, repo) if local else False
        remote_merged = is_remote_merged(repo_root, branch, default, repo) if remote else False
        states[branch] = BranchState(local, remote, in_use, clean, local_merged, remote_merged)
    return states


def evaluate_and_cleanup(repo_root: Path, default: str, states: dict[str, BranchState]) -> int:
    failed = 0
    removed_any = False

    for branch in sorted(states):
        state = states[branch]
        reasons: list[str] = []
        if branch == default:
            reasons.append("default")

        merged_ok = (not state.local or state.local_merged) and (not state.remote or state.remote_merged)
        clean_ok = (not state.local) or state.clean
        if not merged_ok:
            reasons.append("unmerged")
        if not clean_ok:
            reasons.append("dirty")
        if state.in_use:
            reasons.append("in-use")

        if reasons:
            print(f"[cleanup] retain {branch}: {', '.join(reasons)}")
            failed += 1
            continue

        if state.local:
            run_git(["branch", "-d", branch], cwd=repo_root)
            removed_any = True
            print(f"[cleanup] deleted local branch {branch}")
        if state.remote:
            run_git(["push", "origin", "--delete", branch], cwd=repo_root)
            removed_any = True
            print(f"[cleanup] deleted remote branch origin/{branch}")

    if removed_any:
        run_git(["worktree", "prune"], cwd=repo_root)
        print("[cleanup] worktree prune completed")

    return 1 if failed else 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True)
    parser.add_argument(
        "--repo",
        default=os.environ.get("GITHUB_REPOSITORY"),
        help="Repository for `gh release view`, default from GITHUB_REPOSITORY",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path.cwd()
    version = release_tag(args.version)

    print(f"[cleanup] release target: {version}")
    verify_release_published(repo_root, version, args.repo)

    default = default_branch(repo_root)
    print(f"[cleanup] default branch detected: {default}")
    ensure_default_ready(repo_root, default)
    states = collect_branch_states(repo_root, default, args.repo)
    print(f"[cleanup] candidates: {', '.join(sorted(states)) or 'none'}")
    return evaluate_and_cleanup(repo_root, default, states)


if __name__ == "__main__":
    raise SystemExit(main())
