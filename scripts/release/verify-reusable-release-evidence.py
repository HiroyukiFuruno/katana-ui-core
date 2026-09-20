#!/usr/bin/env python3
"""Create and fail-closed verify CI evidence for an identical release-check input."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any


SCHEMA_VERSION = 1
TRUSTED_ISSUER = "github-actions"
TRUSTED_WORKFLOW = ".github/workflows/release-preflight.yml"
EXCLUDED_PREFIXES = ("ci/release-evidence/",)


def utc_now() -> datetime:
    return datetime.now(timezone.utc).replace(microsecond=0)


def iso(value: datetime) -> str:
    return value.isoformat().replace("+00:00", "Z")


def command(*args: str, cwd: Path) -> str:
    return subprocess.check_output(args, cwd=cwd, text=True).strip()


def content_digest(repo: Path) -> str:
    """Hash every tracked input except the evidence-only directory.

    Hashing the repository rather than a hand-maintained source list makes source,
    tests, fixtures, lockfiles, scripts, workflows and toolchain changes invalidate
    evidence by default.
    """
    dirty = command("git", "status", "--porcelain", "--untracked-files=all", cwd=repo)
    if dirty:
        raise ValueError("release evidence cannot be created or reused from a dirty tree")
    files = command("git", "ls-files", "-z", cwd=repo).split("\0")
    digest = hashlib.sha256()
    for relative in sorted(item for item in files if item and not item.startswith(EXCLUDED_PREFIXES)):
        path = repo / relative
        digest.update(relative.encode("utf-8"))
        digest.update(b"\0")
        digest.update(path.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def environment_identity() -> dict[str, str]:
    return {
        "os": command("uname", "-s", cwd=Path.cwd()),
        "arch": command("uname", "-m", cwd=Path.cwd()),
        "rustc": command("rustc", "-Vv", cwd=Path.cwd()),
        "emoji_font_sha256": os.environ.get("KUC_PINNED_LINUX_EMOJI_SHA256", ""),
        "mono_font_sha256": os.environ.get("KUC_PINNED_LINUX_MONO_SHA256", ""),
    }


def evidence(repo: Path, source_sha: str, run_id: str, repository: str, ttl_hours: int) -> dict[str, Any]:
    now = utc_now()
    return {
        "schema_version": SCHEMA_VERSION,
        "issuer": TRUSTED_ISSUER,
        "workflow": TRUSTED_WORKFLOW,
        "repository": repository,
        "run_id": run_id,
        "source_sha": source_sha,
        "content_digest": content_digest(repo),
        "environment": environment_identity(),
        "result": "passed",
        "issued_at": iso(now),
        "expires_at": iso(now + timedelta(hours=ttl_hours)),
    }


def parse_time(value: object) -> datetime | None:
    if not isinstance(value, str):
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def reuse_failures(candidate: object, source_run: object, digest: str, environment: dict[str, str], repository: str, now: datetime) -> list[str]:
    if not isinstance(candidate, dict):
        return ["release evidence root must be an object"]
    failures: list[str] = []
    for key, expected in (("schema_version", SCHEMA_VERSION), ("issuer", TRUSTED_ISSUER), ("workflow", TRUSTED_WORKFLOW), ("repository", repository), ("result", "passed"), ("content_digest", digest), ("environment", environment)):
        if candidate.get(key) != expected:
            failures.append(f"release evidence {key} does not match trusted current input")
    expires = parse_time(candidate.get("expires_at"))
    if expires is None or expires <= now:
        failures.append("release evidence is missing or expired")
    if not isinstance(source_run, dict):
        return failures + ["release evidence source run is missing"]
    if str(source_run.get("id")) != str(candidate.get("run_id")):
        failures.append("release evidence run id does not match GitHub source run")
    if source_run.get("event") != "pull_request" or source_run.get("status") != "completed" or source_run.get("conclusion") != "success":
        failures.append("release evidence source run is not a successful pull_request workflow")
    if source_run.get("repository", {}).get("full_name") != repository:
        failures.append("release evidence source run repository is untrusted")
    if source_run.get("head_sha") != candidate.get("source_sha"):
        failures.append("release evidence source SHA does not match GitHub source run")
    # Actions Runs REST はrefなしのrepository内workflow pathを返す。source SHAは
    # 上で独立検証済みなので、ここでは信頼済みworkflow pathとの完全一致だけを許可する。
    path = source_run.get("path")
    if path != TRUSTED_WORKFLOW:
        failures.append("release evidence source workflow is untrusted")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--write", type=Path)
    parser.add_argument("--candidate", type=Path)
    parser.add_argument("--source-run", type=Path)
    parser.add_argument("--source-sha")
    parser.add_argument("--run-id")
    parser.add_argument("--repository", required=True)
    parser.add_argument("--ttl-hours", type=int, default=24)
    args = parser.parse_args()
    try:
        if args.write:
            if not args.source_sha or not args.run_id or args.ttl_hours <= 0:
                raise ValueError("--write requires --source-sha, --run-id, and positive --ttl-hours")
            payload = evidence(args.repo.resolve(), args.source_sha, args.run_id, args.repository, args.ttl_hours)
            args.write.parent.mkdir(parents=True, exist_ok=True)
            args.write.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            print(f"release evidence recorded: run={args.run_id} digest={payload['content_digest']}")
            return 0
        if not args.candidate or not args.source_run:
            raise ValueError("--candidate and --source-run are required to verify reuse")
        candidate = json.loads(args.candidate.read_text(encoding="utf-8"))
        source_run = json.loads(args.source_run.read_text(encoding="utf-8"))
        failures = reuse_failures(candidate, source_run, content_digest(args.repo.resolve()), environment_identity(), args.repository, utc_now())
        if failures:
            print("release evidence reuse rejected: " + "; ".join(failures), file=sys.stderr)
            return 1
        assert isinstance(candidate, dict)
        print(f"release evidence reuse verified: reason=identical-trusted-ci-input run={candidate['run_id']} digest={candidate['content_digest']}")
        return 0
    except (OSError, ValueError, subprocess.CalledProcessError, json.JSONDecodeError) as error:
        print(f"release evidence reuse rejected: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
