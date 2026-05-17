#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
FILES = {
    "root": ROOT / "docs/architecture/ui-separation/root-plan-source.md",
    "plan": ROOT / "docs/ui-separation-plan.md",
    "tasks": ROOT / "openspec/changes/ui-core-root-plan/tasks.md",
}

TASK_ID = re.compile(r"\bP(?:0-[BC]|1-[A-Z]|4-0|8-A)-\d{3}\b")
TASK_PREFIX = re.compile(r"^(P(?:0-[BC]|1-[A-Z]|4-0|8-A))-\d{3}$")


def ids(path: Path) -> set[str]:
    return set(TASK_ID.findall(path.read_text(encoding="utf-8")))


def prefix(task_id: str) -> str:
    match = TASK_PREFIX.match(task_id)
    if match:
        return match.group(1)
    return task_id


def main() -> int:
    values = {name: ids(path) for name, path in FILES.items()}
    root_text = FILES["root"].read_text(encoding="utf-8")
    root_ids = values["root"]
    failures: list[str] = []
    for name in ("plan", "tasks"):
        extra = sorted(
            task_id
            for task_id in values[name] - root_ids
            if prefix(task_id) not in root_text
        )
        if extra:
            failures.append(f"{name}: no root evidence for {', '.join(extra)}")

    if failures:
        print("root plan task drift detected", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
