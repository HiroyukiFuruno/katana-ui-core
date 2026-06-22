#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path

from storybook_manual_acceptance_approve import (
    APPROVAL_LOG_PATH,
    LEDGER_PATH,
    approve_ledger_pages,
    approve_manual_acceptance,
    manual_acceptance_approval_failures,
)
from storybook_manual_acceptance_metadata import validate_approval_metadata
from storybook_manual_acceptance_next import next_manual_acceptance_entry
from storybook_manual_acceptance_queue import MANIFEST_PATH
from storybook_manual_acceptance_smoke import (
    EVIDENCE_PATH,
    manual_acceptance_evidence_report_failures,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--ledger", type=Path, default=LEDGER_PATH)
    parser.add_argument("--evidence", type=Path, default=EVIDENCE_PATH)
    parser.add_argument("--approval-log", type=Path, default=APPROVAL_LOG_PATH)
    parser.add_argument("--approved-by", required=True)
    parser.add_argument("--approved-at", required=True)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        output = complete_next_manual_acceptance(
            args.manifest,
            args.ledger,
            args.evidence,
            args.approval_log,
            args.approved_by,
            args.approved_at,
        )
    except ValueError as error:
        print(f"storybook manual acceptance complete next failed: {error}")
        return 1
    for line in output:
        print(line)
    return 0


def complete_next_manual_acceptance(
    manifest_path: Path,
    ledger_path: Path,
    evidence_path: Path,
    approval_log_path: Path,
    approved_by: str,
    approved_at: str,
) -> list[str]:
    entry = next_manual_acceptance_entry(manifest_path)
    if entry is None:
        return ["no pending manual acceptance page"]
    page = entry.get("page")
    if not isinstance(page, str):
        raise ValueError("next manual acceptance page is invalid")
    pages = {page}
    preflight_next_manual_acceptance(
        manifest_path,
        ledger_path,
        evidence_path,
        approval_log_path,
        pages,
        approved_by,
        approved_at,
    )
    output = approve_manual_acceptance(
        manifest_path,
        ledger_path,
        evidence_path,
        approval_log_path,
        pages,
    )
    return output


def preflight_next_manual_acceptance(
    manifest_path: Path,
    ledger_path: Path,
    evidence_path: Path,
    approval_log_path: Path,
    pages: set[str],
    approved_by: str,
    approved_at: str,
) -> None:
    validate_approval_metadata(approved_by, approved_at)
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
    ledger_source = ledger_path.read_text(encoding="utf-8")
    _ledger_next, ledger_changes = approve_ledger_pages(ledger_source, pages)
    ledger_pages = {page for page, _count, main_count in ledger_changes if main_count > 0}
    missing_ledger = sorted(pages - ledger_pages)
    if missing_ledger:
        raise ValueError(
            "manual acceptance ledger row is missing or not pending: "
            + ", ".join(missing_ledger)
        )


if __name__ == "__main__":
    raise SystemExit(main())
