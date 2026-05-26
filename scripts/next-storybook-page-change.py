#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PRIORITY_PATH = (
    "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md"
)


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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true", dest="as_json")
    parser.add_argument("--root", type=Path, default=ROOT)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    row = StorybookNextChangeResolver(args.root).next_row()
    if row is None:
        if args.as_json:
            print(json.dumps({"complete": True}, ensure_ascii=False))
        else:
            print("all storybook page leaf changes are complete")
        return 0
    payload = {
        "complete": False,
        "priority": row.priority,
        "page": row.page,
        "change": row.change,
        "implementation_status": row.implementation_status,
        "dod_status": row.dod_status,
        "next_action": row.next_action,
    }
    if args.as_json:
        print(json.dumps(payload, ensure_ascii=False))
        return 0
    print(f"{row.priority} {row.change} page={row.page}")
    print(f"implementation_status={row.implementation_status}")
    print(f"dod_status={row.dod_status}")
    print(f"next_action={row.next_action}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
