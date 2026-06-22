#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from storybook_manual_acceptance_approve import APPROVAL_LOG_PATH
from storybook_manual_acceptance_queue import MANIFEST_PATH, manual_acceptance_queue


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--output", type=Path, default=APPROVAL_LOG_PATH)
    parser.add_argument("--page", action="append", default=[])
    parser.add_argument("--approved-by", default="")
    parser.add_argument("--approved-at", default="")
    parser.add_argument("--force", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        entries = approval_template_entries(
            args.manifest,
            set(args.page),
            args.approved_by,
            args.approved_at,
        )
        write_approval_template(args.output, entries, args.force)
    except ValueError as error:
        print(f"storybook manual acceptance approval template failed: {error}")
        return 1
    print(f"wrote {args.output} ({len(entries)} page(s))")
    return 0


def approval_template_entries(
    manifest_path: Path,
    pages: set[str],
    approved_by: str = "",
    approved_at: str = "",
) -> list[dict[str, Any]]:
    queue = manual_acceptance_queue(manifest_path)
    if not queue:
        if pages:
            raise ValueError(f"manual acceptance page is not pending: {', '.join(sorted(pages))}")
        return []
    next_entry = queue[0]
    next_page = next_entry.get("page")
    if not isinstance(next_page, str):
        raise ValueError("next manual acceptance page is invalid")
    if pages and pages != {next_page}:
        requested = ", ".join(sorted(pages))
        raise ValueError(
            f"{requested}: manual acceptance must follow order; next pending page is {next_page}"
        )
    queue = [next_entry]
    entries: list[dict[str, Any]] = []
    for entry in queue:
        page = entry.get("page")
        if not isinstance(page, str):
            continue
        entries.append(
            {
                "page": page,
                "approved": False,
                "approved_by": approved_by,
                "approved_at": approved_at,
                "command": entry.get("command", ""),
                "smoke_command": entry.get("smoke_command", ""),
                "minimum_observation_frames": entry.get("minimum_observation_frames"),
                "acceptance_checks": entry.get("acceptance_checks", []),
                "acceptance_observations": entry.get("acceptance_observations", []),
                "acceptance_evidence_contract": entry.get(
                    "acceptance_evidence_contract",
                    [],
                ),
                "manual_gate": entry.get("manual_gate", ""),
                "notes": "Set approved=true only after user manually confirms this Storybook page.",
            }
        )
    missing = sorted(pages - {entry["page"] for entry in entries})
    if missing:
        raise ValueError(f"manual acceptance page is not pending: {', '.join(missing)}")
    return entries


def write_approval_template(
    output_path: Path,
    entries: list[dict[str, Any]],
    force: bool = False,
) -> None:
    if output_path.exists() and not force:
        raise ValueError(f"{output_path} already exists; pass --force to overwrite")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(entries, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    raise SystemExit(main())
