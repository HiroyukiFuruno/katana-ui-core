#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from storybook_manual_acceptance_approve import APPROVAL_LOG_PATH
from storybook_manual_acceptance_metadata import validate_approval_metadata
from storybook_manual_acceptance_queue import (
    MANIFEST_PATH,
    manual_acceptance_queue,
    require_no_pending_dependencies,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--approval-log", type=Path, default=APPROVAL_LOG_PATH)
    parser.add_argument("--page", action="append", required=True)
    parser.add_argument("--approved-by", required=True)
    parser.add_argument("--approved-at", required=True)
    parser.add_argument("--dry-run", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        output = mark_manual_acceptance_approved(
            args.manifest,
            args.approval_log,
            set(args.page),
            args.approved_by,
            args.approved_at,
            args.dry_run,
        )
    except ValueError as error:
        print(f"storybook manual acceptance mark approved failed: {error}")
        return 1
    for line in output:
        print(line)
    return 0


def mark_manual_acceptance_approved(
    manifest_path: Path,
    approval_log_path: Path,
    pages: set[str],
    approved_by: str,
    approved_at: str,
    dry_run: bool = False,
) -> list[str]:
    validate_approval_metadata(approved_by, approved_at)
    if len(pages) != 1:
        raise ValueError("manual acceptance must be applied one page at a time")
    queue = manual_acceptance_queue(manifest_path)
    queue_by_page = {
        entry.get("page"): entry
        for entry in queue
        if isinstance(entry.get("page"), str)
    }
    missing = sorted(pages - set(queue_by_page))
    if missing:
        raise ValueError(f"manual acceptance page is not pending: {', '.join(missing)}")
    page = next(iter(pages))
    next_page = next_pending_page(queue)
    if next_page is not None and page != next_page:
        raise ValueError(f"manual acceptance must follow order; next pending page is {next_page}")
    require_no_pending_dependencies(queue_by_page[page], queue)
    approvals = load_existing_approvals(approval_log_path)
    approvals_by_page: dict[str, dict[str, Any]] = {}
    ordered_pages: list[str] = []
    allowed_pages = approval_allowed_pages(manifest_path, queue)
    future_pending_pages = set(queue_by_page) - pages
    for entry in approvals:
        page = entry.get("page")
        if not isinstance(page, str):
            continue
        if page in approvals_by_page:
            raise ValueError(f"{page}: approval log entry is duplicated")
        if page in future_pending_pages:
            raise ValueError(f"{page}: approval log has future pending page")
        if page not in allowed_pages:
            raise ValueError(f"{page}: approval log has unexpected page")
        if page not in approvals_by_page:
            ordered_pages.append(page)
        approvals_by_page[page] = entry
    for page in sorted(pages):
        if page not in approvals_by_page:
            ordered_pages.append(page)
        approvals_by_page[page] = approved_entry_from_queue(
            queue_by_page[page],
            approved_by,
            approved_at,
        )
    next_approvals = [approvals_by_page[page] for page in ordered_pages]
    if not dry_run:
        approval_log_path.parent.mkdir(parents=True, exist_ok=True)
        approval_log_path.write_text(
            json.dumps(next_approvals, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
    output = [f"{page}: marked approved" for page in sorted(pages)]
    if dry_run:
        output.append("dry-run: no files changed")
    return output


def approval_allowed_pages(manifest_path: Path, queue: list[dict[str, Any]]) -> set[str]:
    allowed = {
        page
        for page in (entry.get("page") for entry in queue)
        if isinstance(page, str)
    }
    payload = json.loads(manifest_path.read_text(encoding="utf-8"))
    entries = payload.get("ui", [])
    if not isinstance(entries, list):
        return allowed
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        if not isinstance(page, str):
            continue
        if entry.get("acceptance_checks") or entry.get("acceptance_observations"):
            allowed.add(page)
    return allowed


def next_pending_page(queue: list[dict[str, Any]]) -> str | None:
    for entry in queue:
        page = entry.get("page")
        if isinstance(page, str):
            return page
    return None


def load_existing_approvals(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, list):
        raise ValueError(f"{path}: approval log must be an array")
    return [entry for entry in payload if isinstance(entry, dict)]


def approved_entry_from_queue(
    queue_entry: dict[str, Any],
    approved_by: str,
    approved_at: str,
) -> dict[str, Any]:
    return {
        "page": queue_entry["page"],
        "approved": True,
        "approved_by": approved_by,
        "approved_at": approved_at,
        "command": queue_entry.get("command", ""),
        "smoke_command": queue_entry.get("smoke_command", ""),
        "minimum_observation_frames": queue_entry.get("minimum_observation_frames"),
        "acceptance_checks": queue_entry.get("acceptance_checks", []),
        "acceptance_observations": queue_entry.get("acceptance_observations", []),
        "acceptance_evidence_contract": queue_entry.get(
            "acceptance_evidence_contract",
            [],
        ),
        "manual_gate": queue_entry.get("manual_gate", ""),
        "notes": "Approved only after the user manually confirmed this Storybook page.",
    }


if __name__ == "__main__":
    raise SystemExit(main())
