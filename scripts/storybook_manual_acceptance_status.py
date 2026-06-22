#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from storybook_manual_acceptance_approve import (
    APPROVAL_LOG_PATH,
    LEDGER_PATH,
    manual_acceptance_approval_failures,
)
from storybook_manual_acceptance_final_gate import ledger_pending_failures
from storybook_manual_acceptance_queue import MANIFEST_PATH, manual_acceptance_queue
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
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    print(
        json.dumps(
            manual_acceptance_status(
                args.manifest,
                args.ledger,
                args.evidence,
                args.approval_log,
            ),
            ensure_ascii=False,
            indent=2,
        )
    )
    return 0


def manual_acceptance_status(
    manifest_path: Path,
    ledger_path: Path | None = None,
    evidence_path: Path | None = None,
    approval_log_path: Path | None = None,
) -> dict[str, Any]:
    queue = manual_acceptance_queue(manifest_path)
    first = queue[0] if queue else {}
    ledger_pending = ledger_pending_failures(ledger_path) if ledger_path is not None else []
    next_page = first.get("page", "") if isinstance(first, dict) else ""
    next_pages = {next_page} if isinstance(next_page, str) and next_page else set()
    evidence_failures = (
        manual_acceptance_evidence_report_failures(
            manifest_path,
            evidence_path,
            next_pages,
        )
        if evidence_path is not None and next_pages
        else []
    )
    approval_failures = (
        manual_acceptance_approval_failures(
            manifest_path,
            approval_log_path,
            next_pages,
        )
        if approval_log_path is not None and next_pages
        else []
    )
    complete = not queue and not ledger_pending
    evidence_ready = not evidence_failures
    approval_ready = not approval_failures
    return {
        "complete": complete,
        "pending_count": len(queue),
        "pending_pages": [
            entry["page"]
            for entry in queue
            if isinstance(entry.get("page"), str)
        ],
        "next_page": next_page,
        "manual_gate": first.get("manual_gate", "") if isinstance(first, dict) else "",
        "pending_reason": "manual_acceptance_pending" if queue or ledger_pending else "",
        "ledger_pending_count": len(ledger_pending),
        "next_command": first.get("command", "") if isinstance(first, dict) else "",
        "next_smoke_command": first.get("smoke_command", "") if isinstance(first, dict) else "",
        "evidence_ready": evidence_ready,
        "evidence_failures": evidence_failures,
        "approval_ready": approval_ready,
        "approval_failures": approval_failures,
        "next_action": next_action(complete, evidence_ready, approval_ready),
    }


def next_action(complete: bool, evidence_ready: bool, approval_ready: bool) -> str:
    if complete:
        return "none"
    if not evidence_ready:
        return "refresh_manual_acceptance_evidence"
    if not approval_ready:
        return "await_user_storybook_confirmation"
    return "apply_manual_acceptance_approval"


if __name__ == "__main__":
    raise SystemExit(main())
