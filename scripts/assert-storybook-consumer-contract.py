#!/usr/bin/env python3
from pathlib import Path
import re
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CONTRACT_DOC = ROOT / "docs/storybook-consumer-contract.md"
TASKS = ROOT / "openspec/changes/establish-kuc-atoms-molecules-catalog/tasks.md"
JUSTFILE = ROOT / "Justfile"
REQUIREMENT_GATE = ROOT / "scripts/storybook-requirement-gate.sh"

REQUIRED_DOC_TOKENS = (
    "public API",
    "typed props / options",
    "typed state",
    "typed action",
    "typed event / log",
    "layout bounds",
    "hit target",
    "rendering contract",
    "fallback 禁止",
    "Storybook 専用状態の禁止",
    "目視確認",
    "スクリーンショット",
)

FORBIDDEN_READY_EVIDENCE_PATTERNS = (
    r"Storybook.*(ready|完了).*(目視|スクリーンショット|screenshot)",
    r"(目視|スクリーンショット|screenshot).*(ready|完了根拠|完了判定)",
    r"(fallback|フォールバック).*(ready|完了根拠|完了判定)",
)
ALLOWED_NEGATION_TOKENS = (
    "しない",
    "ない",
    "禁止",
    "ではない",
    "してはならない",
    "ならない",
    "失敗扱い",
    "reject",
)

TASK_REQUIRED_TOKENS = (
    "consumer harness",
    "[/]",
    "[ ]",
    "readiness audit",
)
READY_LINK_TOKENS = (
    "ready",
    "public API",
    "typed props",
    "typed state",
    "typed action",
    "typed event",
    "layout bounds",
    "hit target",
    "rendering contract",
)


def relative(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def doc_failures() -> list[str]:
    if not CONTRACT_DOC.exists():
        return [f"{relative(CONTRACT_DOC)}: Storybook consumer contract doc is missing"]
    source = read(CONTRACT_DOC)
    failures = [
        f"{relative(CONTRACT_DOC)}: missing token `{token}`"
        for token in REQUIRED_DOC_TOKENS
        if token not in source
    ]
    if "ready" not in source and "Ready" not in source:
        failures.append(f"{relative(CONTRACT_DOC)}: ready condition is not defined")
    return failures


def gate_failures() -> list[str]:
    failures: list[str] = []
    if not JUSTFILE.exists():
        return [f"{relative(JUSTFILE)}: Justfile is missing"]
    justfile_source = read(JUSTFILE)
    required_lines = (
        "scripts/assert-storybook-consumer-contract.py --self-test",
        "scripts/assert-storybook-consumer-contract.py",
    )
    for line in required_lines:
        if line not in justfile_source:
            failures.append(f"{relative(JUSTFILE)}: kuc-guardrails missing `{line}`")
    if not REQUIREMENT_GATE.exists():
        failures.append(f"{relative(REQUIREMENT_GATE)}: storybook requirement gate is missing")
    return failures


def tasks_failures() -> list[str]:
    if not TASKS.exists():
        return [f"{relative(TASKS)}: tasks.md is missing"]
    source = read(TASKS)
    failures = [
        f"{relative(TASKS)}: consumer contract task missing `{token}`"
        for token in TASK_REQUIRED_TOKENS
        if token not in source
    ]
    readiness_audit_pattern = re.compile(
        r"- \[ \].*全 Storybook page の readiness audit",
        flags=re.IGNORECASE,
    )
    if not readiness_audit_pattern.search(source):
        failures.append(
            f"{relative(TASKS)}: readiness audit task must remain unchecked (`- [ ]`) until re-evaluated"
        )
    if "ready" in source.lower():
        missing_links = [token for token in READY_LINK_TOKENS if token.lower() not in source.lower()]
        if missing_links:
            failures.append(
                f"{relative(TASKS)}: ready criteria is not linked to consumer contract tokens: {', '.join(missing_links)}"
            )
    return failures


def ready_evidence_failures(paths: tuple[Path, ...]) -> list[str]:
    failures: list[str] = []
    for path in paths:
        if not path.exists():
            continue
        source = read(path)
        for line_number, line in enumerate(source.splitlines(), start=1):
            lowered = line.lower()
            if any(token in line for token in ALLOWED_NEGATION_TOKENS) or "reject" in lowered:
                continue
            for pattern in FORBIDDEN_READY_EVIDENCE_PATTERNS:
                if re.search(pattern, line, flags=re.IGNORECASE):
                    failures.append(
                        f"{relative(path)}:{line_number}: forbidden Storybook ready evidence: {line.strip()}"
                    )
                    break
    return failures


def self_test() -> int:
    good_lines = (
        "Storybook の目視確認を完了根拠にしない。",
        "fallback を ready 判定に使わない。",
    )
    bad_lines = (
        "Storybook ready はスクリーンショットで確認する。",
        "fallback があっても完了判定にする。",
    )
    for line in good_lines:
        if any(
            re.search(pattern, line, flags=re.IGNORECASE)
            and not any(token in line for token in ALLOWED_NEGATION_TOKENS)
            for pattern in FORBIDDEN_READY_EVIDENCE_PATTERNS
        ):
            print("storybook consumer contract self-test failed", file=sys.stderr)
            return 1
    for line in bad_lines:
        if not any(re.search(pattern, line, flags=re.IGNORECASE) for pattern in FORBIDDEN_READY_EVIDENCE_PATTERNS):
            print("storybook consumer contract self-test failed", file=sys.stderr)
            return 1
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        fake = root / "fake.md"
        fake.write_text("Storybook ready は目視で確認する。\n", encoding="utf-8")
        local_failures = ready_evidence_failures((fake,))
        if not local_failures:
            print("storybook consumer contract self-test failed", file=sys.stderr)
            return 1
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()
    failures: list[str] = []
    failures.extend(doc_failures())
    failures.extend(gate_failures())
    failures.extend(tasks_failures())
    failures.extend(ready_evidence_failures((CONTRACT_DOC, TASKS, ROOT / "AGENTS.md")))
    if failures:
        print("storybook consumer contract guard failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
