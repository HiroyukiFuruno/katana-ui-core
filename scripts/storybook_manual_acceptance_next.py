#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shlex
import subprocess
from pathlib import Path
from typing import Any

from storybook_manual_acceptance_queue import (
    MANIFEST_PATH,
    manual_acceptance_queue,
    require_no_pending_dependencies,
    string_values,
)
from storybook_manual_acceptance_review import CommandRunner, format_review_entry


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--open", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    result, output = run_next_manual_acceptance(
        args.manifest,
        args.open,
        lambda command: subprocess.run(command, check=False).returncode,
    )
    print(output)
    return result


def next_manual_acceptance_entry(manifest_path: Path) -> dict[str, Any] | None:
    queue = manual_acceptance_queue(manifest_path)
    if not queue:
        return None
    entry = queue[0]
    require_no_pending_dependencies(entry, queue)
    return entry


def run_next_manual_acceptance(
    manifest_path: Path,
    open_page: bool,
    runner: CommandRunner,
) -> tuple[int, str]:
    try:
        entry = next_manual_acceptance_entry(manifest_path)
    except ValueError as error:
        return 1, f"storybook manual acceptance next failed: {error}"
    if entry is None:
        return 0, "storybook manual acceptance next: no pending manual acceptance page"
    output = format_next_entry(entry)
    if not open_page:
        return 0, output
    command = entry.get("command")
    if not isinstance(command, str) or not command.strip():
        return 1, output + f"\n{entry.get('page', '<unknown>')}: command is missing"
    result = runner(shlex.split(command))
    if result != 0:
        return (
            1,
            output + f"\n{entry.get('page', '<unknown>')}: command failed with exit code {result}",
        )
    return 0, output


def format_next_entry(entry: dict[str, Any]) -> str:
    page = str(entry.get("page", ""))
    depends_on = ", ".join(string_values(entry.get("depends_on", [])))
    lines = [
        format_review_entry(1, 1, entry),
        f"order: {entry.get('manual_acceptance_order', '')}",
        f"layer: {entry.get('dependency_layer', '')}",
        f"depends_on: {depends_on}",
    ]
    lines.extend(format_evidence_contract(entry))
    lines.extend(
        [
            "approval template:",
            "  rtk just storybook-manual-acceptance-approval-template",
            "after user OK only:",
            "  rtk just storybook-manual-acceptance-complete-next <approved_by> <approved_at>",
        ]
    )
    return "\n".join(lines)


def format_evidence_contract(entry: dict[str, Any]) -> list[str]:
    contracts = entry.get("acceptance_evidence_contract")
    if not isinstance(contracts, list) or not contracts:
        return []
    lines = ["evidence contract:"]
    for contract in contracts:
        if not isinstance(contract, dict):
            continue
        check = contract.get("check")
        operation_kind = contract.get("operation_kind")
        state = contract.get("state")
        action = contract.get("action")
        event = contract.get("event")
        if not all(isinstance(value, str) and value for value in [check, operation_kind, state, action, event]):
            continue
        lines.append(
            f"  - {check} operation_kind={operation_kind} state={state} action={action} event={event}"
        )
    if len(lines) == 1:
        return []
    return lines


if __name__ == "__main__":
    raise SystemExit(main())
