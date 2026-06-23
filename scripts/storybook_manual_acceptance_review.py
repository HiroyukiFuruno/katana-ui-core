#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shlex
import subprocess
from pathlib import Path
from typing import Any, Protocol

from storybook_manual_acceptance_queue import (
    MANIFEST_PATH,
    manual_acceptance_queue,
    string_values,
)


class CommandRunner(Protocol):
    def __call__(self, command: list[str]) -> int: ...


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--page", action="append", default=[])
    parser.add_argument(
        "--open",
        action="store_true",
        help="open each selected Storybook page after printing its checklist",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    return review_manual_acceptance(
        args.manifest,
        set(args.page),
        args.open,
        lambda command: subprocess.run(command, check=False).returncode,
    )


def review_manual_acceptance(
    manifest_path: Path,
    pages: set[str],
    open_pages: bool,
    runner: CommandRunner,
) -> int:
    selected = manual_acceptance_review_entries(manifest_path, pages)
    if not selected:
        print("storybook manual acceptance review: no matching pending pages")
        return 0
    failures: list[str] = []
    for index, entry in enumerate(selected, start=1):
        print(format_review_entry(index, len(selected), entry))
        if not open_pages:
            continue
        command = entry.get("command")
        if not isinstance(command, str) or not command.strip():
            failures.append(f"{entry.get('page', '<unknown>')}: command is missing")
            continue
        result = runner(shlex.split(command))
        if result != 0:
            failures.append(f"{entry.get('page', '<unknown>')}: command failed with exit code {result}")
    if failures:
        print("storybook manual acceptance review failed")
        for failure in failures:
            print(f"- {failure}")
        return 1
    return 0


def manual_acceptance_review_entries(
    manifest_path: Path,
    pages: set[str],
) -> list[dict[str, Any]]:
    entries = manual_acceptance_queue(manifest_path)
    if not pages:
        return entries
    return [entry for entry in entries if entry.get("page") in pages]


def format_review_entry(index: int, total: int, entry: dict[str, Any]) -> str:
    page = entry.get("page", "<unknown>")
    operations = ", ".join(string_values(entry.get("required_operations", [])))
    checks = ", ".join(string_values(entry.get("acceptance_checks", [])))
    observations = "\n".join(
        f"  - {observation}"
        for observation in string_values(entry.get("acceptance_observations", []))
    )
    manual_gate = entry.get("manual_gate", "")
    command = entry.get("command", "")
    smoke_command = entry.get("smoke_command", "")
    return "\n".join(
        [
            f"[{index}/{total}] {page}",
            f"operations: {operations}",
            f"checks: {checks}",
            "observe:",
            observations,
            "manual gate:",
            f"  {manual_gate}",
            f"open: {command}",
            f"smoke: {smoke_command}",
        ]
    )


if __name__ == "__main__":
    raise SystemExit(main())
