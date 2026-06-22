#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from storybook_manifest_interaction_smoke import (
    AUDIT_PATH,
    MANIFEST_PATH,
    load_json,
    manifest_smoke_failures,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--audit", type=Path, default=AUDIT_PATH)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    failures = pending_only_failures(
        args.root / args.manifest,
        args.root / args.audit,
    )
    if failures:
        print("storybook interaction smoke has blocking failure drift")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("storybook interaction smoke has no blocking failures; manual pending is non-blocking")
    return 0


def pending_only_failures(manifest_path: Path, audit_path: Path) -> list[str]:
    actual = set(manifest_smoke_failures(manifest_path, audit_path))
    expected = set()
    return pending_only_difference_failures(actual, expected)


def pending_only_difference_failures(actual: set[str], expected: set[str]) -> list[str]:
    failures: list[str] = []
    unexpected = sorted(actual - expected)
    missing = sorted(expected - actual)
    failures.extend(f"unexpected failure: {failure}" for failure in unexpected)
    failures.extend(f"missing manual pending failure: {failure}" for failure in missing)
    return failures


def has_manual_acceptance_pending(gaps: Any) -> bool:
    if not isinstance(gaps, list):
        return False
    return any(
        isinstance(gap, str) and "manual_acceptance_pending" in gap for gap in gaps
    )


if __name__ == "__main__":
    raise SystemExit(main())
