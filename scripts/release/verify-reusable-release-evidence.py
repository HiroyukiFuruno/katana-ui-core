#!/usr/bin/env python3
"""Create and fail-closed verify CI evidence for an identical release-check input."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any


SCHEMA_VERSION = 2
TRUSTED_ISSUER = "github-actions"
TRUSTED_WORKFLOW = ".github/workflows/release-preflight.yml"
TRUSTED_BRANCH = "master"
EXCLUDED_PREFIXES = ("ci/release-evidence/",)
NATIVE_PACKAGES = (
    "ffmpeg",
    "fonts-noto-cjk",
    "fonts-noto-color-emoji",
    "fonts-noto-mono",
    "xauth",
    "xvfb",
)
EMOJI_FONT = Path("/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf")
MONO_FONT = Path("/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf")


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


def content_digest_at_revision(repo: Path, revision: str) -> str:
    """Hash an immutable Git tree without running its checked-in scripts."""
    files = subprocess.check_output(
        ("git", "ls-tree", "-r", "-z", "--name-only", revision), cwd=repo
    ).split(b"\0")
    digest = hashlib.sha256()
    for raw_relative in sorted(item for item in files if item):
        relative = raw_relative.decode("utf-8")
        if relative.startswith(EXCLUDED_PREFIXES):
            continue
        digest.update(raw_relative)
        digest.update(b"\0")
        digest.update(
            subprocess.check_output(("git", "show", f"{revision}:{relative}"), cwd=repo)
        )
        digest.update(b"\0")
    return digest.hexdigest()


def required_command(*args: str) -> str:
    try:
        value = command(*args, cwd=Path.cwd())
    except (OSError, subprocess.CalledProcessError) as error:
        raise ValueError(f"release evidence environment identity is unavailable: {' '.join(args)}") from error
    if not value:
        raise ValueError(f"release evidence environment identity is empty: {' '.join(args)}")
    return value


def required_environment(name: str) -> str:
    value = os.environ.get(name, "")
    if not value:
        raise ValueError(f"release evidence environment identity is missing {name}")
    return value


def required_file_digest(path: Path) -> str:
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except OSError as error:
        raise ValueError(f"release evidence environment identity cannot read {path}") from error


def os_release_identity() -> str:
    try:
        values = dict(
            line.split("=", 1)
            for line in Path("/etc/os-release").read_text(encoding="utf-8").splitlines()
            if "=" in line
        )
        return f"{values['ID']}:{values['VERSION_ID']}"
    except (OSError, KeyError) as error:
        raise ValueError("release evidence environment identity cannot read /etc/os-release") from error


def package_identity(package: str) -> str:
    value = required_command("dpkg-query", "-W", "-f=${db:Status-Abbrev} ${Version}", package)
    status, *version = value.split(maxsplit=1)
    if status != "ii" or len(version) != 1:
        raise ValueError(f"release evidence environment identity package is not installed: {package}")
    return f"{package}={version[0]}"


def environment_identity() -> dict[str, str]:
    xvfb_path = shutil.which("Xvfb")
    if xvfb_path is None:
        raise ValueError("release evidence environment identity is missing Xvfb")
    return {
        "os": command("uname", "-s", cwd=Path.cwd()),
        "arch": command("uname", "-m", cwd=Path.cwd()),
        "rustc": command("rustc", "-Vv", cwd=Path.cwd()),
        "runner_image_os": required_environment("ImageOS"),
        "runner_image_version": required_environment("ImageVersion"),
        "os_release": os_release_identity(),
        "xvfb_path": xvfb_path,
        "ffmpeg_version": required_command("ffmpeg", "-version").splitlines()[0],
        "native_packages": ";".join(package_identity(package) for package in NATIVE_PACKAGES),
        "emoji_font_sha256": required_file_digest(EMOJI_FONT),
        "mono_font_sha256": required_file_digest(MONO_FONT),
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


def reuse_failures(candidate: object, source_run: object, digest: str, source_digest: str, environment: dict[str, str], repository: str, now: datetime) -> list[str]:
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
    if source_run.get("event") != "workflow_dispatch" or source_run.get("head_branch") != TRUSTED_BRANCH or source_run.get("status") != "completed" or source_run.get("conclusion") != "success":
        failures.append("release evidence source run is not a successful protected workflow dispatch")
    if source_run.get("repository", {}).get("full_name") != repository:
        failures.append("release evidence source run repository is untrusted")
    if source_run.get("head_sha") != candidate.get("source_sha"):
        failures.append("release evidence source SHA does not match GitHub source run")
    if candidate.get("content_digest") != source_digest:
        failures.append("release evidence content digest does not match the GitHub source tree")
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
        source_sha = source_run.get("head_sha")
        if not isinstance(source_sha, str) or not source_sha:
            raise ValueError("release evidence source run is missing a head SHA")
        failures = reuse_failures(
            candidate,
            source_run,
            content_digest(args.repo.resolve()),
            content_digest_at_revision(args.repo.resolve(), source_sha),
            environment_identity(),
            args.repository,
            utc_now(),
        )
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
