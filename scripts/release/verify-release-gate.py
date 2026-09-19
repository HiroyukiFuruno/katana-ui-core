#!/usr/bin/env python3
"""Validate and record the SHA-bound release-gate preconditions."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from tempfile import NamedTemporaryFile
from typing import Any


SCHEMA_VERSION = 1
SHA_RE = re.compile(r"^[0-9a-f]{40,64}$")


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def elapsed_seconds(started_at: object, finished_at: str) -> int | None:
    if not isinstance(started_at, str):
        return None
    try:
        started = datetime.fromisoformat(started_at.replace("Z", "+00:00"))
        finished = datetime.fromisoformat(finished_at.replace("Z", "+00:00"))
    except ValueError:
        return None
    return max(0, int((finished - started).total_seconds()))


def read_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValueError(f"could not read JSON manifest {path}: {error}") from error


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with NamedTemporaryFile("w", encoding="utf-8", dir=path.parent, delete=False) as temporary:
        json.dump(payload, temporary, indent=2, sort_keys=True)
        temporary.write("\n")
        temporary_path = Path(temporary.name)
    os.replace(temporary_path, path)


def manifest_failures(payload: object, expected_sha: str) -> list[str]:
    if not isinstance(payload, dict):
        return ["release gate manifest root must be an object"]
    failures: list[str] = []
    if payload.get("schema_version") != SCHEMA_VERSION:
        failures.append(f"release gate manifest schema_version must be {SCHEMA_VERSION}")
    actual_sha = payload.get("sha")
    if not isinstance(actual_sha, str) or not SHA_RE.fullmatch(actual_sha):
        failures.append("release gate manifest sha is missing or malformed")
    elif actual_sha != expected_sha:
        failures.append(f"release gate manifest sha {actual_sha} does not match expected {expected_sha}")

    threads = payload.get("review_threads")
    if not isinstance(threads, dict):
        failures.append("release gate manifest review_threads is missing")
    else:
        unresolved = threads.get("unresolved")
        if type(unresolved) is not int or unresolved < 0:
            failures.append("release gate manifest review_threads.unresolved must be a non-negative integer")
        elif unresolved != 0:
            failures.append(f"release gate has {unresolved} unresolved review thread(s)")

    preflight = payload.get("preflight")
    if not isinstance(preflight, dict):
        failures.append("release gate manifest preflight is missing")
    else:
        if preflight.get("status") != "passed":
            failures.append("release gate preflight must be passed")
        if preflight.get("sha") != expected_sha:
            failures.append("release gate preflight sha does not match expected sha")
    return failures


def state_failures(payload: object, expected_sha: str) -> list[str]:
    if not isinstance(payload, dict):
        return ["release gate state root must be an object"]
    failures: list[str] = []
    if payload.get("schema_version") != SCHEMA_VERSION:
        failures.append(f"release gate state schema_version must be {SCHEMA_VERSION}")
    if payload.get("sha") != expected_sha:
        failures.append("release gate state sha does not match expected sha")
    if payload.get("phase") != "start":
        failures.append("release gate state must originate from a start phase")
    if payload.get("status") != "started":
        failures.append("release gate state must be started before completion")
    if not isinstance(payload.get("started_at"), str) or not payload["started_at"]:
        failures.append("release gate state started_at is missing")
    return failures


def start_gate(manifest: Path, expected_sha: str, state_output: Path) -> int:
    manifest_payload = read_json(manifest)
    failures = manifest_failures(manifest_payload, expected_sha)
    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1
    assert isinstance(manifest_payload, dict)
    state = {
        "schema_version": SCHEMA_VERSION,
        "phase": "start",
        "status": "started",
        "sha": expected_sha,
        "stage": "release-check",
        "review_threads_unresolved": manifest_payload["review_threads"]["unresolved"],
        "preflight_status": manifest_payload["preflight"]["status"],
        "started_at": utc_now(),
    }
    write_json(state_output, state)
    print(
        f"release gate started: sha={expected_sha} stage=release-check "
        f"started_at={state['started_at']} next=run release-check"
    )
    return 0


def complete_gate(state_input: Path, expected_sha: str, result: str, state_output: Path) -> int:
    state = read_json(state_input)
    failures = state_failures(state, expected_sha)
    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1
    assert isinstance(state, dict)
    completed = dict(state)
    finished_at = utc_now()
    completed.update({"phase": "complete", "status": result, "finished_at": finished_at})
    duration = elapsed_seconds(completed.get("started_at"), finished_at)
    if duration is None:
        print("release gate state started_at is invalid", file=sys.stderr)
        return 1
    completed["duration_seconds"] = duration
    write_json(state_output, completed)
    next_step = "release may continue" if result == "passed" else "fix failures and rerun for this SHA"
    print(
        f"release gate {result}: sha={expected_sha} stage=release-check "
        f"finished_at={finished_at} duration_seconds={duration} next={next_step}"
    )
    return 0 if result == "passed" else 1


def create_manifest(output: Path, sha: str, unresolved: int, preflight_status: str) -> int:
    if not SHA_RE.fullmatch(sha):
        print("manifest sha is malformed", file=sys.stderr)
        return 1
    if unresolved < 0:
        print("unresolved review thread count must be non-negative", file=sys.stderr)
        return 1
    write_json(
        output,
        {
            "schema_version": SCHEMA_VERSION,
            "sha": sha,
            "review_threads": {"unresolved": unresolved},
            "preflight": {"status": preflight_status, "sha": sha},
        },
    )
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, help="input release gate manifest")
    parser.add_argument("--expected-sha", default=os.environ.get("GITHUB_SHA"))
    parser.add_argument("--phase", choices=("start", "complete"))
    parser.add_argument("--state-input", type=Path)
    parser.add_argument("--state-output", type=Path)
    parser.add_argument("--result", choices=("passed", "failed"), default="passed")
    parser.add_argument("--write-manifest", type=Path)
    parser.add_argument("--sha")
    parser.add_argument("--unresolved-review-threads", type=int, default=0)
    parser.add_argument("--preflight-status", default="passed")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.write_manifest:
        if not args.sha:
            print("--sha is required with --write-manifest", file=sys.stderr)
            return 2
        return create_manifest(args.write_manifest, args.sha, args.unresolved_review_threads, args.preflight_status)
    if not args.manifest or not args.expected_sha or not args.phase or not args.state_output:
        print("--manifest, --expected-sha, --phase, and --state-output are required", file=sys.stderr)
        return 2
    if args.phase == "start":
        return start_gate(args.manifest, args.expected_sha, args.state_output)
    if not args.state_input:
        print("--state-input is required for complete phase", file=sys.stderr)
        return 2
    failures = manifest_failures(read_json(args.manifest), args.expected_sha)
    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1
    return complete_gate(args.state_input, args.expected_sha, args.result, args.state_output)


if __name__ == "__main__":
    raise SystemExit(main())
