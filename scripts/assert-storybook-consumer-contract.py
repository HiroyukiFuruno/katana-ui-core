#!/usr/bin/env python3
from pathlib import Path
import re
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CONTRACT_DOC = ROOT / "docs/storybook-consumer-contract.md"
READINESS_AUDIT_DOC = ROOT / "docs/storybook-consumer-readiness-audit.md"
LEGACY_RECHECK_DOC = ROOT / "docs/legacy-01-24-consumer-recheck.md"
TASKS = ROOT / "openspec/changes/establish-kuc-atoms-molecules-catalog/tasks.md"
JUSTFILE = ROOT / "Justfile"
REQUIREMENT_GATE = ROOT / "scripts/storybook-requirement-gate.sh"
REQUIREMENTS_RS = ROOT / "crates/katana-ui-core-storybook/src/requirements.rs"

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
READY_STATUS_TOKENS = ("ready", "partial", "not-ready")
LEGACY_RECHECK_HEADERS = (
    "legacy id",
    "page",
    "public api",
    "typed props/options",
    "typed state",
    "typed action",
    "typed event",
    "preset",
    "preview",
    "settings",
    "test/guard",
    "visual",
)
LEGACY_RECHECK_REQUIRED_PATHS = (
    "crates/katana-ui-core-storybook/src/requirements.rs",
    "crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs",
    "crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs",
    "crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs",
    "crates/katana-ui-core-storybook/tests/legacy_01_24_detail_contract.rs",
)
LEGACY_RECHECK_DATE_TOKEN = "2026-05-23"
P0_READY_PAGES = (
    "checkbox",
    "radio",
    "select-box",
    "combo-box",
    "search-box",
    "selection-list",
)
REQUIRED_PAGES_BLOCK_PATTERN = re.compile(
    r"const\s+REQUIRED_PAGES:\s*&\[\s*&str\s*\]\s*=\s*&\[(?P<body>.*?)\];",
    flags=re.DOTALL,
)
REQUIRED_COUNT_PATTERN = re.compile(r"required\s+(?P<count>\d+)\s+pages", flags=re.IGNORECASE)
SUMMARY_COUNT_PATTERN = re.compile(
    r"-\s*(?P<status>ready|partial|not-ready):\s*(?P<count>\d+)\s*$",
    flags=re.IGNORECASE | re.MULTILINE,
)


def relative(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def required_pages_from_source(source: str) -> tuple[str, ...]:
    match = REQUIRED_PAGES_BLOCK_PATTERN.search(source)
    if match is None:
        raise ValueError("REQUIRED_PAGES block not found")
    pages = re.findall(r'"([a-z0-9-]+)"', match.group("body"))
    if not pages:
        raise ValueError("REQUIRED_PAGES block is empty")
    return tuple(pages)


def required_pages() -> tuple[str, ...]:
    return required_pages_from_source(read(REQUIREMENTS_RS))


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
        r"- \[(?: |x)\].*全 Storybook page の readiness audit",
        flags=re.IGNORECASE,
    )
    if not readiness_audit_pattern.search(source):
        failures.append(f"{relative(TASKS)}: readiness audit task entry is missing")
    if "ready" in source.lower():
        missing_links = [token for token in READY_LINK_TOKENS if token.lower() not in source.lower()]
        if missing_links:
            failures.append(
                f"{relative(TASKS)}: ready criteria is not linked to consumer contract tokens: {', '.join(missing_links)}"
            )
    return failures


def parse_readiness_rows(source: str) -> list[tuple[str, str, str, str]]:
    rows: list[tuple[str, str, str, str]] = []
    for line in source.splitlines():
        if not line.startswith("|"):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if len(parts) != 4:
            continue
        page, status, evidence, missing = parts
        if page in {"page", "---"}:
            continue
        rows.append((page, status, evidence, missing))
    return rows


def parse_summary_counts(source: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for match in SUMMARY_COUNT_PATTERN.finditer(source):
        counts[match.group("status").lower()] = int(match.group("count"))
    return counts


def readiness_audit_source_failures(source: str, required: tuple[str, ...]) -> list[str]:
    failures: list[str] = []
    if "consumer contract readiness audit" not in source.lower():
        return [f"{relative(READINESS_AUDIT_DOC)}: missing audit title"]
    rows = parse_readiness_rows(source)
    if not rows:
        return [f"{relative(READINESS_AUDIT_DOC)}: readiness table is missing"]

    required_count_match = REQUIRED_COUNT_PATTERN.search(source)
    if required_count_match is None:
        failures.append(
            f"{relative(READINESS_AUDIT_DOC)}: required page count line is missing"
        )
    else:
        documented_required_count = int(required_count_match.group("count"))
        if documented_required_count != len(required):
            failures.append(
                f"{relative(READINESS_AUDIT_DOC)}: required count mismatch doc={documented_required_count} actual={len(required)}"
            )

    summary_counts = parse_summary_counts(source)
    for status in READY_STATUS_TOKENS:
        if status not in summary_counts:
            failures.append(
                f"{relative(READINESS_AUDIT_DOC)}: summary count for `{status}` is missing"
            )

    row_map: dict[str, tuple[str, str, str]] = {}
    actual_counts = {status: 0 for status in READY_STATUS_TOKENS}
    for page, status, evidence, missing in rows:
        if status not in READY_STATUS_TOKENS:
            failures.append(
                f"{relative(READINESS_AUDIT_DOC)}: `{page}` has invalid status `{status}`"
            )
            continue
        if page in row_map:
            failures.append(
                f"{relative(READINESS_AUDIT_DOC)}: duplicate page entry `{page}`"
            )
            continue
        row_map[page] = (status, evidence, missing)
        actual_counts[status] += 1

        if status == "ready":
            required_tokens = ("callback log", "state", "action", "event")
            missing_tokens = [token for token in required_tokens if token not in evidence.lower()]
            has_hit_or_rendering = (
                "hit target" in evidence.lower() or "rendering contract" in evidence.lower()
            )
            if missing_tokens:
                failures.append(
                    f"{relative(READINESS_AUDIT_DOC)}: `{page}` ready evidence missing tokens: {', '.join(missing_tokens)}"
                )
            if not has_hit_or_rendering:
                failures.append(
                    f"{relative(READINESS_AUDIT_DOC)}: `{page}` ready evidence must include hit target or rendering contract"
                )
            if ".rs" not in evidence and "scripts/" not in evidence:
                failures.append(
                    f"{relative(READINESS_AUDIT_DOC)}: `{page}` ready evidence must reference test/guard path"
                )
            if missing != "-":
                failures.append(
                    f"{relative(READINESS_AUDIT_DOC)}: `{page}` ready row must set missing to `-`"
                )

    missing_pages = [page for page in required if page not in row_map]
    if missing_pages:
        failures.append(
            f"{relative(READINESS_AUDIT_DOC)}: missing page rows: {', '.join(missing_pages)}"
        )
    extra_pages = [page for page in row_map if page not in required]
    if extra_pages:
        failures.append(
            f"{relative(READINESS_AUDIT_DOC)}: unknown page rows: {', '.join(extra_pages)}"
        )

    if len(rows) != len(required):
        failures.append(
            f"{relative(READINESS_AUDIT_DOC)}: table row count mismatch rows={len(rows)} required={len(required)}"
        )
    for status in READY_STATUS_TOKENS:
        expected = summary_counts.get(status)
        actual = actual_counts[status]
        if expected is not None and expected != actual:
            failures.append(
                f"{relative(READINESS_AUDIT_DOC)}: summary mismatch `{status}` doc={expected} table={actual}"
            )

    for page in P0_READY_PAGES:
        status = row_map.get(page, ("", "", ""))[0]
        if status != "ready":
            failures.append(
                f"{relative(READINESS_AUDIT_DOC)}: `{page}` must stay `ready` in P0 readiness checkpoint"
            )
    return failures


def readiness_audit_failures() -> list[str]:
    if not READINESS_AUDIT_DOC.exists():
        return [f"{relative(READINESS_AUDIT_DOC)}: readiness audit doc is missing"]
    source = read(READINESS_AUDIT_DOC)
    try:
        required = required_pages()
    except ValueError as error:
        return [f"{relative(REQUIREMENTS_RS)}: {error}"]
    return readiness_audit_source_failures(source, required)


def legacy_recheck_source_failures(source: str) -> list[str]:
    lowered = source.lower()
    failures: list[str] = []
    if LEGACY_RECHECK_DATE_TOKEN not in source:
        failures.append(
            f"{relative(LEGACY_RECHECK_DOC)}: missing recheck date"
        )
    for header in LEGACY_RECHECK_HEADERS:
        if header not in lowered:
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: missing table header `{header}`"
            )
    for path in LEGACY_RECHECK_REQUIRED_PATHS:
        if path not in source:
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: missing evidence path `{path}`"
            )

    ids: set[int] = set()
    for line in source.splitlines():
        if not line.startswith("|"):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if len(parts) != len(LEGACY_RECHECK_HEADERS):
            continue
        if parts[0] in {"legacy id", "---"}:
            continue
        if not re.fullmatch(r"\d{2}", parts[0]):
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: invalid legacy id format `{parts[0]}`"
            )
            continue
        legacy_id = int(parts[0])
        if legacy_id < 1 or legacy_id > 24:
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: legacy id out of range `{parts[0]}`"
            )
            continue
        if legacy_id in ids:
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: duplicate legacy id `{parts[0]}`"
            )
            continue
        ids.add(legacy_id)
        if any(not item for item in parts[1:]):
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: legacy id `{parts[0]}` has an empty column"
            )
            continue
        if any(".rs" not in item for item in parts[2:]):
            failures.append(
                f"{relative(LEGACY_RECHECK_DOC)}: legacy id `{parts[0]}` columns must reference `.rs` evidence"
            )

    missing_ids = [str(it).zfill(2) for it in range(1, 25) if it not in ids]
    if missing_ids:
        failures.append(
            f"{relative(LEGACY_RECHECK_DOC)}: missing legacy ids: {', '.join(missing_ids)}"
        )
    return failures


def legacy_recheck_failures() -> list[str]:
    if not LEGACY_RECHECK_DOC.exists():
        return [f"{relative(LEGACY_RECHECK_DOC)}: legacy recheck doc is missing"]
    return legacy_recheck_source_failures(read(LEGACY_RECHECK_DOC))


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
    required_fixture = 'const REQUIRED_PAGES: &[&str] = &["a", "b"];'
    try:
        parsed = required_pages_from_source(required_fixture)
    except ValueError:
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    if parsed != ("a", "b"):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    try:
        required_pages_from_source("const MIN_SINGLE_NODE: usize = 1;")
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    except ValueError:
        pass
    summary_fixture = "- ready: 1\n- partial: 2\n- not-ready: 3\n"
    counts = parse_summary_counts(summary_fixture)
    if counts.get("ready") != 1 or counts.get("partial") != 2 or counts.get("not-ready") != 3:
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    mismatch_fixture = "- ready: 1\n- partial: 2\n- not-ready: 2\n"
    mismatch_counts = parse_summary_counts(mismatch_fixture)
    if mismatch_counts.get("not-ready") == 3:
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    required_fixture_pages = (
        "checkbox",
        "radio",
        "select-box",
        "combo-box",
        "search-box",
        "selection-list",
    )
    fixture_template = """# Storybook Consumer Contract Readiness Audit
対象: `crates/katana-ui-core-storybook/src/requirements.rs` の required {required_count} pages

## 集計
- ready: {ready}
- partial: {partial}
- not-ready: {not_ready}

## Page 別分類
| page | status | evidence | missing |
| --- | --- | --- | --- |
| checkbox | ready | callback log/state/action/event/hit target: `a.rs` | - |
| radio | ready | callback log/state/action/event/hit target: `a.rs` | - |
| select-box | ready | callback log/state/action/event/hit target: `a.rs` | - |
| combo-box | ready | callback log/state/action/event/hit target: `a.rs` | - |
| search-box | ready | callback log/state/action/event/hit target: `a.rs` | - |
| selection-list | ready | callback log/state/action/event/hit target: `a.rs` | - |
"""
    ok_source = fixture_template.format(required_count=6, ready=6, partial=0, not_ready=0)
    if readiness_audit_source_failures(ok_source, required_fixture_pages):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    summary_mismatch_source = fixture_template.format(
        required_count=6, ready=5, partial=1, not_ready=0
    )
    summary_mismatch = readiness_audit_source_failures(
        summary_mismatch_source, required_fixture_pages
    )
    if not any("summary mismatch `ready`" in failure for failure in summary_mismatch):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    required_count_mismatch_source = fixture_template.format(
        required_count=7, ready=6, partial=0, not_ready=0
    )
    required_count_mismatch = readiness_audit_source_failures(
        required_count_mismatch_source, required_fixture_pages
    )
    if not any("required count mismatch" in failure for failure in required_count_mismatch):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    row_count_mismatch_source = """# Storybook Consumer Contract Readiness Audit
対象: `crates/katana-ui-core-storybook/src/requirements.rs` の required 6 pages

## 集計
- ready: 6
- partial: 0
- not-ready: 0

## Page 別分類
| page | status | evidence | missing |
| --- | --- | --- | --- |
| checkbox | ready | callback log/state/action/event/hit target: `a.rs` | - |
| radio | ready | callback log/state/action/event/hit target: `a.rs` | - |
| select-box | ready | callback log/state/action/event/hit target: `a.rs` | - |
| combo-box | ready | callback log/state/action/event/hit target: `a.rs` | - |
| search-box | ready | callback log/state/action/event/hit target: `a.rs` | - |
"""
    row_count_mismatch = readiness_audit_source_failures(
        row_count_mismatch_source, required_fixture_pages
    )
    if not any("table row count mismatch" in failure for failure in row_count_mismatch):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    legacy_doc_fixture = """# Legacy 01〜24 Consumer Harness 再評価（2026-05-23）
| legacy id | page | public API | typed props/options | typed state | typed action | typed event | preset | preview | settings | test/guard | visual |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
"""
    valid_row = "| {id} | `page` | `crates/katana-ui-core-storybook/src/requirements.rs` | `crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs` | `crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs` | `crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs` | `crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs` | `crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs` | `crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs` | `crates/katana-ui-core-storybook/tests/legacy_01_24_detail_contract.rs` | `crates/katana-ui-core-storybook/tests/legacy_01_24_detail_contract.rs` | `crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs` |"
    valid_rows = "\n".join(valid_row.format(id=str(it).zfill(2)) for it in range(1, 24))
    row_24 = (
        "| 24 | `page` | `crates/katana-ui-core-storybook/src/requirements.rs` | "
        "`crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs` | "
        "`crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs` | "
        "`crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs` | "
        "`crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs` | "
        "`crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs` | "
        "`crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs` | "
        "`crates/katana-ui-core-storybook/tests/legacy_01_24_detail_contract.rs` | "
        "`crates/katana-ui-core-storybook/tests/legacy_01_24_detail_contract.rs` | "
        "`crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs` |"
    )
    valid_source = (
        f"{legacy_doc_fixture}{valid_rows}\n"
        f"{row_24}\n"
    )
    if legacy_recheck_source_failures(valid_source):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    missing_id_source = valid_source.replace(f"{row_24}\n", "")
    missing_id_failures = legacy_recheck_source_failures(missing_id_source)
    if not any("missing legacy ids: 24" in failure for failure in missing_id_failures):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    duplicate_source = valid_source + valid_row.format(id="01") + "\n"
    duplicate_failures = legacy_recheck_source_failures(duplicate_source)
    if not any("duplicate legacy id `01`" in failure for failure in duplicate_failures):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    empty_column_source = valid_source.replace("`page`", "", 1)
    empty_column_failures = legacy_recheck_source_failures(empty_column_source)
    if not any("has an empty column" in failure for failure in empty_column_failures):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    missing_rs_source = valid_source.replace(
        "crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs",
        "docs/storybook-consumer-contract.md",
        1,
    )
    missing_rs_failures = legacy_recheck_source_failures(missing_rs_source)
    if not any("must reference `.rs` evidence" in failure for failure in missing_rs_failures):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    missing_path_source = valid_source.replace(
        "crates/katana-ui-core-storybook/src/requirements.rs",
        "docs/legacy-01-24-consumer-recheck.md",
    )
    missing_path_failures = legacy_recheck_source_failures(missing_path_source)
    if not any("missing evidence path `crates/katana-ui-core-storybook/src/requirements.rs`" in failure for failure in missing_path_failures):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1

    missing_date_source = valid_source.replace(LEGACY_RECHECK_DATE_TOKEN, "")
    missing_date_failures = legacy_recheck_source_failures(missing_date_source)
    if not any("missing recheck date" in failure for failure in missing_date_failures):
        print("storybook consumer contract self-test failed", file=sys.stderr)
        return 1
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()
    failures: list[str] = []
    failures.extend(doc_failures())
    failures.extend(readiness_audit_failures())
    failures.extend(legacy_recheck_failures())
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
