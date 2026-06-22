#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_status import manual_acceptance_status


class StorybookManualAcceptanceStatusTest(unittest.TestCase):
    def test_status_reports_next_pending_page_and_manual_gate(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "missing-approval.json"

            status = manual_acceptance_status(manifest, ledger, evidence, approval)

            self.assertFalse(status["complete"])
            self.assertEqual(2, status["pending_count"])
            self.assertEqual(["text", "checkbox"], status["pending_pages"])
            self.assertEqual("text", status["next_page"])
            self.assertEqual(
                "do not proceed to the next UI until this page is approved",
                status["manual_gate"],
            )
            self.assertNotIn("blocked_reason", status)
            self.assertEqual("manual_acceptance_pending", status["pending_reason"])
            self.assertEqual(3, status["ledger_pending_count"])
            self.assertIn("--open-window text", status["next_command"])
            self.assertIn("--open-window 1 text", status["next_smoke_command"])
            self.assertTrue(status["evidence_ready"])
            self.assertEqual([], status["evidence_failures"])
            self.assertFalse(status["approval_ready"])
            self.assertIn("approval log is missing", "; ".join(status["approval_failures"]))
            self.assertEqual(
                "await_user_storybook_confirmation",
                status["next_action"],
            )

    def test_status_reports_complete_when_manifest_and_ledger_have_no_pending_rows(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            manifest.write_text(
                json.dumps({"ui": [{"page": "text", "audit_status": "verified", "gaps": []}]}),
                encoding="utf-8",
            )
            ledger = root / "ledger.md"
            ledger.write_text(
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
                "| ---: | --- | --- | --- | --- | --- |\n"
                "| 01 | text | done | ok | ok | 実証済み |\n",
                encoding="utf-8",
            )

            status = manual_acceptance_status(manifest, ledger)

            self.assertTrue(status["complete"])
            self.assertEqual(0, status["pending_count"])
            self.assertEqual([], status["pending_pages"])
            self.assertEqual("", status["next_page"])
            self.assertNotIn("blocked_reason", status)
            self.assertEqual("", status["pending_reason"])
            self.assertEqual(0, status["ledger_pending_count"])
            self.assertTrue(status["evidence_ready"])
            self.assertTrue(status["approval_ready"])
            self.assertEqual("none", status["next_action"])


def write_manifest(root: Path) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    pending_entry("checkbox", 20, ["text"]),
                    pending_entry("text", 10, []),
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


def pending_entry(page: str, order: int, depends_on: list[str]) -> dict[str, object]:
    return {
        "page": page,
        "audit_status": "partial",
        "manual_acceptance_order": order,
        "dependency_layer": f"{page}-layer",
        "depends_on": depends_on,
        "required_operations": ["pointer"],
        "minimum_observation_frames": 1,
        "acceptance_checks": [f"{page}_check"],
        "acceptance_observations": [f"{page} observation"],
        "gaps": ["manual_acceptance_pending: user confirmation is required"],
    }


def write_ledger(root: Path) -> Path:
    ledger = root / "ledger.md"
    ledger.write_text(
        "\n".join(
            [
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |",
                "| ---: | --- | --- | --- | --- | --- |",
                "| 01 | text | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "| 01a | text follow-up | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "| 10 | checkbox | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "",
            ]
        ),
        encoding="utf-8",
    )
    return ledger


def write_evidence(root: Path) -> Path:
    evidence = root / "evidence.json"
    evidence.write_text(
        json.dumps(
            [
                {
                    "page": "text",
                    "command": command_for("text"),
                    "smoke_command": smoke_command_for("text"),
                    "minimum_observation_frames": 1,
                    "acceptance_observations": ["text observation"],
                    "audit_evidence": [
                        {
                            "check": "text_check",
                            "passed": True,
                        }
                    ],
                }
            ]
        ),
        encoding="utf-8",
    )
    return evidence


def command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window {page}"
    )


def smoke_command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window 1 {page}"
    )


if __name__ == "__main__":
    unittest.main()
