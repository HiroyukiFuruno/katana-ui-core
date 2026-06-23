#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path

from storybook_manual_acceptance_queue import MANIFEST_PATH, manual_acceptance_queue

ROOT = Path(__file__).resolve().parents[1]
PRIORITY_PATH = (
    "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md"
)
HANDOFF_GLOB = "docs/reviews/*kuc-remaining-work-handoff.md"


@dataclass(frozen=True)
class StorybookPriorityRow:
    priority: str
    page: str
    change: str
    implementation_status: str
    dod_status: str
    next_action: str


class StorybookNextChangeResolver:
    def __init__(self, root: Path = ROOT) -> None:
        self.root = root

    def next_row(self) -> StorybookPriorityRow | None:
        for row in self.priority_rows():
            if self.is_incomplete(row.change):
                return row
        return None

    def completion_payload(self) -> dict[str, object]:
        row = self.next_row()
        if row is not None:
            return {
                "complete": False,
                "leaf_queue_complete": False,
                "completion_scope": "storybook_page_leaf_changes",
                "priority": row.priority,
                "page": row.page,
                "change": row.change,
                "implementation_status": row.implementation_status,
                "dod_status": row.dod_status,
                "next_action": row.next_action,
            }
        remaining_handoff_items = self.remaining_handoff_items()
        kuc_dod_complete = not remaining_handoff_items
        payload: dict[str, object] = {
            "complete": kuc_dod_complete,
            "leaf_queue_complete": True,
            "completion_scope": "storybook_page_leaf_changes",
            "kuc_dod_complete": kuc_dod_complete,
            "remaining_handoff_items": remaining_handoff_items,
            "next_action": (
                "KUC DoD complete"
                if kuc_dod_complete
                else "audit remaining P0/P1 handoff items before claiming KUC DoD complete"
            ),
        }
        payload.update(self.manual_acceptance_payload())
        return payload

    def priority_rows(self) -> list[StorybookPriorityRow]:
        path = self.root / PRIORITY_PATH
        rows: list[StorybookPriorityRow] = []
        for line in path.read_text(encoding="utf-8").splitlines():
            if not line.startswith("| SB-"):
                continue
            cells = [cell.strip() for cell in line.strip("|").split("|")]
            if len(cells) < 6:
                continue
            rows.append(
                StorybookPriorityRow(
                    priority=cells[0],
                    page=cells[1].strip("`"),
                    change=cells[2].strip("`"),
                    implementation_status=cells[3],
                    dod_status=cells[4],
                    next_action=cells[5],
                )
            )
        return rows

    def is_incomplete(self, change: str) -> bool:
        tasks = self.root / "openspec/changes" / change / "tasks.md"
        if not tasks.exists():
            return True
        source = tasks.read_text(encoding="utf-8")
        return "- [ ]" in source

    def remaining_handoff_items(self) -> list[str]:
        handoff = self.latest_handoff()
        if handoff is None:
            return ["handoff file missing"]
        items: list[str] = []
        for line in handoff.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if stripped.startswith("- [/] P") or stripped.startswith("- [ ] P"):
                items.append(stripped)
        return items

    def latest_handoff(self) -> Path | None:
        candidates = sorted(self.root.glob(HANDOFF_GLOB))
        if not candidates:
            return None
        return candidates[-1]

    def manual_acceptance_payload(self) -> dict[str, object]:
        manifest = self.root / MANIFEST_PATH
        if not manifest.exists():
            return {}
        queue = manual_acceptance_queue(manifest)
        if not queue:
            return {}
        first = queue[0]
        pending_pages = [
            entry["page"]
            for entry in queue
            if isinstance(entry.get("page"), str)
        ]
        next_page = first.get("page", "") if isinstance(first, dict) else ""
        return {
            "pending_reason": "manual_acceptance_pending",
            "pending_manual_acceptance_pages": pending_pages,
            "next_manual_acceptance_page": next_page,
            "next_command": first.get("command", "") if isinstance(first, dict) else "",
            "next_smoke_command": first.get("smoke_command", "") if isinstance(first, dict) else "",
            "manual_gate": first.get("manual_gate", "") if isinstance(first, dict) else "",
            "next_action": "await_user_storybook_confirmation",
        }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true", dest="as_json")
    parser.add_argument("--root", type=Path, default=ROOT)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    payload = StorybookNextChangeResolver(args.root).completion_payload()
    if payload["leaf_queue_complete"]:
        if args.as_json:
            print(json.dumps(payload, ensure_ascii=False))
        else:
            print(format_completion_payload(payload))
        return 0
    if args.as_json:
        print(json.dumps(payload, ensure_ascii=False))
        return 0
    print(f"{payload['priority']} {payload['change']} page={payload['page']}")
    print(f"implementation_status={payload['implementation_status']}")
    print(f"dod_status={payload['dod_status']}")
    print(f"next_action={payload['next_action']}")
    return 0


def format_completion_payload(payload: dict[str, object]) -> str:
    lines = [
        "all storybook page leaf changes are complete",
        f"completion_scope={payload.get('completion_scope', '')}",
        f"kuc_dod_complete={str(payload.get('kuc_dod_complete', False)).lower()}",
    ]
    pending_reason = string_value(payload.get("pending_reason"))
    if pending_reason:
        lines.append(f"pending_reason={pending_reason}")
    next_page = string_value(payload.get("next_manual_acceptance_page"))
    if next_page:
        lines.append(f"next_manual_acceptance_page={next_page}")
    next_command = string_value(payload.get("next_command"))
    if next_command:
        lines.append(f"next_command={next_command}")
    next_smoke_command = string_value(payload.get("next_smoke_command"))
    if next_smoke_command:
        lines.append(f"next_smoke_command={next_smoke_command}")
    manual_gate = string_value(payload.get("manual_gate"))
    if manual_gate:
        lines.append(f"manual_gate={manual_gate}")
    lines.append(f"next_action={payload.get('next_action', '')}")
    return "\n".join(lines)


def string_value(value: object) -> str:
    return value if isinstance(value, str) else ""


if __name__ == "__main__":
    raise SystemExit(main())
