#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from storybook_manual_acceptance_queue import (
    MANIFEST_PATH,
    manual_acceptance_queue,
    require_no_pending_dependencies,
    string_values,
)
from storybook_manual_acceptance_metadata import validate_approval_metadata
from storybook_manual_acceptance_smoke import (
    EVIDENCE_PATH,
    manual_acceptance_evidence_report_failures,
)

LEDGER_PATH = Path("docs/storybook-77ui-deep-audit-ledger.md")
APPROVAL_LOG_PATH = Path("docs/storybook-manual-acceptance-approvals.json")
PENDING_MARKER = "manual_acceptance_pending"
VERIFIED_STATUS = "実証済み"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--ledger", type=Path, default=LEDGER_PATH)
    parser.add_argument("--evidence", type=Path, default=EVIDENCE_PATH)
    parser.add_argument("--approval-log", type=Path, default=APPROVAL_LOG_PATH)
    parser.add_argument("--page", action="append", required=True)
    parser.add_argument("--dry-run", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    pages = set(args.page)
    try:
        result = approve_manual_acceptance(
            args.manifest,
            args.ledger,
            args.evidence,
            args.approval_log,
            pages,
            args.dry_run,
        )
    except ValueError as error:
        print(f"storybook manual acceptance approve failed: {error}")
        return 1
    for line in result:
        print(line)
    return 0


def approve_manual_acceptance(
    manifest_path: Path,
    ledger_path: Path,
    evidence_path: Path,
    approval_log_path: Path,
    pages: set[str],
    dry_run: bool = False,
) -> list[str]:
    enforce_single_pending_page(manifest_path, pages)
    enforce_next_pending_page(manifest_path, pages)
    approval_failures = manual_acceptance_approval_failures(
        manifest_path,
        approval_log_path,
        pages,
    )
    if approval_failures:
        raise ValueError(
            "manual acceptance approval is not ready: "
            + "; ".join(approval_failures)
        )
    evidence_failures = manual_acceptance_evidence_report_failures(
        manifest_path,
        evidence_path,
        pages,
    )
    if evidence_failures:
        raise ValueError(
            "manual acceptance evidence is not ready: "
            + "; ".join(evidence_failures)
        )
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest_changes = approve_manifest_pages(manifest, pages)
    ledger_source = ledger_path.read_text(encoding="utf-8")
    ledger_next, ledger_changes = approve_ledger_pages(ledger_source, pages)

    missing = sorted(pages - {change.page for change in manifest_changes})
    if missing:
        raise ValueError(f"manual acceptance page is not pending in manifest: {', '.join(missing)}")
    ledger_pages = {page for page, _count, main_count in ledger_changes if main_count > 0}
    missing_ledger = sorted(pages - ledger_pages)
    if missing_ledger:
        raise ValueError(
            "manual acceptance ledger row is missing or not pending: "
            + ", ".join(missing_ledger)
        )
    if not dry_run:
        manifest_path.write_text(
            json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
        ledger_path.write_text(ledger_next, encoding="utf-8")
    output = [
        f"{change.page}: manifest {change.before_status}->{change.after_status}"
        for change in manifest_changes
    ]
    output.extend(f"{page}: ledger {count} row(s)" for page, count, _main_count in ledger_changes)
    if dry_run:
        output.append("dry-run: no files changed")
    return output


def enforce_single_pending_page(manifest_path: Path, pages: set[str]) -> None:
    if len(pages) != 1:
        raise ValueError("manual acceptance must be applied one page at a time")


def enforce_next_pending_page(manifest_path: Path, pages: set[str]) -> None:
    page = next(iter(pages))
    queue = manual_acceptance_queue(manifest_path)
    queue_pages = {entry.get("page") for entry in queue if isinstance(entry.get("page"), str)}
    if page not in queue_pages:
        raise ValueError(f"manual acceptance page is not pending in manifest: {page}")
    for entry in queue:
        next_page = entry.get("page")
        if isinstance(next_page, str):
            if page != next_page:
                raise ValueError(
                    f"manual acceptance must follow order; next pending page is {next_page}"
                )
            require_no_pending_dependencies(entry, queue)
            return


def manual_acceptance_approval_failures(
    manifest_path: Path,
    approval_log_path: Path,
    pages: set[str],
) -> list[str]:
    if not approval_log_path.exists():
        return [f"{approval_log_path}: approval log is missing"]
    approvals = json.loads(approval_log_path.read_text(encoding="utf-8"))
    if not isinstance(approvals, list):
        return [f"{approval_log_path}: approval log must be an array"]
    queue_by_page = {
        entry.get("page"): entry
        for entry in manual_acceptance_queue(manifest_path)
        if isinstance(entry.get("page"), str)
    }
    allowed_pages = approval_allowed_pages(manifest_path, queue_by_page)
    future_pending_pages = set(queue_by_page) - pages
    failures: list[str] = []
    approvals_by_page: dict[str, dict[str, Any]] = {}
    for index, approval in enumerate(approvals):
        if not isinstance(approval, dict):
            failures.append(f"user approval[{index}]: entry must be an object")
            continue
        page = approval.get("page")
        if not isinstance(page, str) or not page:
            failures.append(f"user approval[{index}]: page must be a non-empty string")
            continue
        if page in approvals_by_page:
            failures.append(f"{page}: user approval entry is duplicated")
            continue
        if page in future_pending_pages:
            failures.append(f"{page}: user approval has future pending page")
            continue
        if page not in allowed_pages:
            failures.append(f"{page}: user approval has unexpected page")
            continue
        approvals_by_page[page] = approval
    for page in sorted(pages):
        entry = approvals_by_page.get(page)
        if not isinstance(entry, dict):
            failures.append(f"{page}: user approval entry is missing")
            continue
        if entry.get("approved") is not True:
            failures.append(f"{page}: user approval must set approved=true")
        approved_by = entry.get("approved_by")
        approved_at = entry.get("approved_at")
        try:
            validate_approval_metadata(
                approved_by if isinstance(approved_by, str) else "",
                approved_at if isinstance(approved_at, str) else "",
            )
        except ValueError as error:
            failures.append(f"{page}: user approval {error}")
        queue_entry = queue_by_page.get(page)
        if not isinstance(queue_entry, dict):
            failures.append(f"{page}: user approval page is not pending in manifest")
            continue
        if entry.get("command") != queue_entry.get("command"):
            failures.append(f"{page}: user approval command does not match queue")
        if entry.get("smoke_command") != queue_entry.get("smoke_command"):
            failures.append(f"{page}: user approval smoke_command does not match queue")
        if entry.get("minimum_observation_frames") != queue_entry.get(
            "minimum_observation_frames"
        ):
            failures.append(
                f"{page}: user approval minimum_observation_frames does not match queue"
            )
        expected_checks = string_values(queue_entry.get("acceptance_checks", []))
        actual_checks = string_values(entry.get("acceptance_checks", []))
        if actual_checks != expected_checks:
            failures.append(f"{page}: user approval acceptance_checks do not match queue")
        expected_observations = string_values(
            queue_entry.get("acceptance_observations", [])
        )
        actual_observations = string_values(entry.get("acceptance_observations", []))
        if actual_observations != expected_observations:
            failures.append(
                f"{page}: user approval acceptance_observations do not match queue"
            )
        expected_evidence_contract = queue_entry.get("acceptance_evidence_contract", [])
        actual_evidence_contract = entry.get("acceptance_evidence_contract", [])
        if actual_evidence_contract != expected_evidence_contract:
            failures.append(
                f"{page}: user approval acceptance_evidence_contract does not match queue"
            )
        if entry.get("manual_gate") != queue_entry.get("manual_gate"):
            failures.append(f"{page}: user approval manual_gate does not match queue")
    return failures


def approval_allowed_pages(
    manifest_path: Path,
    queue_by_page: dict[Any, dict[str, Any]],
) -> set[str]:
    allowed = {page for page in queue_by_page if isinstance(page, str)}
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


class ManifestChange:
    def __init__(self, page: str, before_status: str, after_status: str) -> None:
        self.page = page
        self.before_status = before_status
        self.after_status = after_status


def approve_manifest_pages(manifest: dict[str, Any], pages: set[str]) -> list[ManifestChange]:
    entries = manifest.get("ui")
    if not isinstance(entries, list):
        raise ValueError("manifest.ui must be an array")
    changes: list[ManifestChange] = []
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        if page not in pages:
            continue
        gaps = entry.get("gaps", [])
        if not isinstance(gaps, list) or not any(
            isinstance(gap, str) and PENDING_MARKER in gap for gap in gaps
        ):
            continue
        before = str(entry.get("audit_status", ""))
        entry["audit_status"] = "verified"
        entry["gaps"] = [
            gap
            for gap in gaps
            if not (isinstance(gap, str) and PENDING_MARKER in gap)
        ]
        changes.append(ManifestChange(page, before, "verified"))
    return changes


def approve_ledger_pages(source: str, pages: set[str]) -> tuple[str, list[tuple[str, int, int]]]:
    counts = {page: 0 for page in pages}
    main_counts = {page: 0 for page in pages}
    output: list[str] = []
    for line in source.splitlines(keepends=True):
        approved_line = line
        if line.startswith("|"):
            cells = split_markdown_row(line)
            if len(cells) >= 6:
                ui = cells[1].strip()
                page = ledger_page_for_ui(ui, pages)
                if page and cells[-1].strip() == PENDING_MARKER:
                    cells[2] = mark_user_confirmation_done(cells[2])
                    cells[-1] = f" {VERIFIED_STATUS} "
                    approved_line = join_markdown_row(cells, line.endswith("\n"))
                    counts[page] += 1
                    if ui == page:
                        main_counts[page] += 1
        output.append(approved_line)
    return "".join(output), [
        (page, count, main_counts[page]) for page, count in sorted(counts.items()) if count
    ]


def ledger_page_for_ui(ui: str, pages: set[str]) -> str | None:
    for page in pages:
        if ui == page or ui.startswith(f"{page} follow-up"):
            return page
    return None


def mark_user_confirmation_done(text: str) -> str:
    replacements = {
        "Storybook ユーザー確認は未完了": "Storybook ユーザー確認済み",
        "実画面確認は未完了": "実画面確認済み",
        "手動再確認は未完了": "手動再確認済み",
    }
    for before, after in replacements.items():
        text = text.replace(before, after)
    return text


def split_markdown_row(line: str) -> list[str]:
    body = line.strip()
    if body.startswith("|"):
        body = body[1:]
    if body.endswith("|"):
        body = body[:-1]
    return body.split("|")


def join_markdown_row(cells: list[str], newline: bool) -> str:
    line = "|" + "|".join(cells) + "|"
    if newline:
        line += "\n"
    return line


if __name__ == "__main__":
    raise SystemExit(main())
