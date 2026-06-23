#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_complete_next import complete_next_manual_acceptance
from storybook_manual_acceptance_queue import manual_acceptance_queue


class StorybookManualAcceptanceCompleteNextTest(unittest.TestCase):
    def test_completes_only_next_pending_page_after_user_ok(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approved_log(root)

            output = complete_next_manual_acceptance(
                manifest,
                ledger,
                evidence,
                approval,
                "hiroyuki",
                "2026-06-15T12:00:00+09:00",
            )

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in manifest_payload["ui"] if entry["page"] == "text")
            checkbox = next(entry for entry in manifest_payload["ui"] if entry["page"] == "checkbox")
            self.assertEqual("verified", text["audit_status"])
            self.assertEqual([], text["gaps"])
            self.assertEqual("partial", checkbox["audit_status"])
            approval_payload = json.loads(approval.read_text(encoding="utf-8"))
            self.assertEqual(["text"], [entry["page"] for entry in approval_payload])
            self.assertIs(approval_payload[0]["approved"], True)
            self.assertEqual(
                [
                    "text_drag_selection",
                    "text_keyboard_copy",
                    "text_zero_distance_drag_no_selection",
                ],
                approval_payload[0]["acceptance_checks"],
            )
            self.assertEqual(
                [
                    "drag text",
                    "copy text",
                    "zero distance drag no-op",
                ],
                approval_payload[0]["acceptance_observations"],
            )
            self.assertEqual(
                text_evidence_contract(),
                approval_payload[0]["acceptance_evidence_contract"],
            )
            self.assertNotIn(
                "checkbox_pointer_checks_both_rows",
                approval_payload[0]["acceptance_checks"],
            )
            self.assertIn("text: manifest partial->verified", output)
            self.assertIn("text: ledger 2 row(s)", output)
            next_queue = manual_acceptance_queue(manifest)
            self.assertEqual(["checkbox"], [entry["page"] for entry in next_queue])

    def test_rejects_without_existing_user_approval_log_before_writing(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "approval.json"

            with self.assertRaisesRegex(ValueError, "approval"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in manifest_payload["ui"] if entry["page"] == "text")
            self.assertEqual("partial", text["audit_status"])

    def test_rejects_unapproved_template_before_writing(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_unapproved_log(root)

            with self.assertRaisesRegex(ValueError, "approved=true"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in manifest_payload["ui"] if entry["page"] == "text")
            self.assertEqual("partial", text["audit_status"])

    def test_rejects_without_evidence_before_writing_approval_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = root / "missing-evidence.json"
            approval = write_approved_log(root)

            with self.assertRaisesRegex(ValueError, "evidence"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in manifest_payload["ui"] if entry["page"] == "text")
            self.assertEqual("partial", text["audit_status"])

    def test_rejects_without_ledger_pending_before_writing_approval_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = root / "ledger.md"
            ledger.write_text(
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
                "| ---: | --- | --- | --- | --- | --- |\n"
                "| 10 | checkbox | pending | ok | ok | manual_acceptance_pending |\n",
                encoding="utf-8",
            )
            evidence = write_evidence(root)
            approval = write_approved_log(root)

            with self.assertRaisesRegex(ValueError, "ledger row"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in manifest_payload["ui"] if entry["page"] == "text")
            self.assertEqual("partial", text["audit_status"])

    def test_rejects_when_only_follow_up_ledger_row_is_pending_before_writing_approval_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = root / "ledger.md"
            ledger.write_text(
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
                "| ---: | --- | --- | --- | --- | --- |\n"
                "| 01a | text follow-up | pending | ok | ok | manual_acceptance_pending |\n",
                encoding="utf-8",
            )
            evidence = write_evidence(root)
            approval = write_approved_log(root)

            with self.assertRaisesRegex(ValueError, "ledger row"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in manifest_payload["ui"] if entry["page"] == "text")
            self.assertEqual("partial", text["audit_status"])

    def test_rejects_pending_dependency_before_writing_approval_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_blocked_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "approval.json"

            with self.assertRaisesRegex(ValueError, "checkbox depends on pending pages: text"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            self.assertFalse(approval.exists())

    def test_rejects_missing_user_approval_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)

            with self.assertRaisesRegex(ValueError, "approved_by"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    root / "approval.json",
                    "",
                    "2026-06-15T12:00:00+09:00",
                )

    def test_rejects_non_iso_approved_at_before_writing_approval_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "approval.json"

            with self.assertRaisesRegex(ValueError, "approved_at"):
                complete_next_manual_acceptance(
                    manifest,
                    ledger,
                    evidence,
                    approval,
                    "hiroyuki",
                    "2026-06-15T12:00:00",
                )

            self.assertFalse(approval.exists())

    def test_reports_no_pending_page_without_writing(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            manifest.write_text(
                json.dumps({"ui": [{"page": "text", "audit_status": "verified", "gaps": []}]}),
                encoding="utf-8",
            )
            output = complete_next_manual_acceptance(
                manifest,
                root / "ledger.md",
                root / "evidence.json",
                root / "approval.json",
                "hiroyuki",
                "2026-06-15T12:00:00+09:00",
            )

            self.assertEqual(["no pending manual acceptance page"], output)


def write_manifest(root: Path) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    {
                        "page": "checkbox",
                        "audit_status": "partial",
                        "manual_acceptance_order": 20,
                        "dependency_layer": "binary-choice-state-display",
                        "depends_on": ["text"],
                        "required_operations": ["pointer", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": ["checkbox_pointer_checks_both_rows"],
                        "acceptance_observations": ["both checked"],
                        "gaps": ["manual_acceptance_pending: checkbox pending"],
                    },
                    {
                        "page": "text",
                        "audit_status": "partial",
                        "manual_acceptance_order": 10,
                        "dependency_layer": "foundation-text-selection",
                        "depends_on": [],
                        "required_operations": ["pointer", "drag", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": [
                            "text_drag_selection",
                            "text_keyboard_copy",
                            "text_zero_distance_drag_no_selection",
                        ],
                        "acceptance_observations": [
                            "drag text",
                            "copy text",
                            "zero distance drag no-op",
                        ],
                        "acceptance_evidence_contract": text_evidence_contract(),
                        "gaps": ["manual_acceptance_pending: text pending"],
                    },
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


def write_blocked_manifest(root: Path) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    {
                        "page": "checkbox",
                        "audit_status": "partial",
                        "manual_acceptance_order": 10,
                        "dependency_layer": "binary-choice-state-display",
                        "depends_on": ["text"],
                        "required_operations": ["pointer", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": ["checkbox_pointer_checks_both_rows"],
                        "acceptance_observations": ["both checked"],
                        "gaps": ["manual_acceptance_pending: checkbox pending"],
                    },
                    {
                        "page": "text",
                        "audit_status": "partial",
                        "manual_acceptance_order": 20,
                        "dependency_layer": "foundation-text-selection",
                        "depends_on": [],
                        "required_operations": ["pointer", "drag", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": [
                            "text_drag_selection",
                            "text_keyboard_copy",
                            "text_zero_distance_drag_no_selection",
                        ],
                        "acceptance_observations": [
                            "drag text",
                            "copy text",
                            "zero distance drag no-op",
                        ],
                        "gaps": ["manual_acceptance_pending: text pending"],
                    },
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


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
                    "acceptance_observations": [
                        "drag text",
                        "copy text",
                        "zero distance drag no-op",
                    ],
                    "audit_evidence": [
                        {
                            "check": "text_drag_selection",
                            "passed": True,
                            "operation_kind": "drag",
                            "state": "selection=active",
                            "action": "select_text",
                            "event": "text_selection_changed",
                            "body_pixel_diff": 12,
                        },
                        {
                            "check": "text_keyboard_copy",
                            "passed": True,
                            "operation_kind": "keyboard",
                            "state": "clipboard=selected_text",
                            "action": "copy_selection",
                            "event": "clipboard_copy",
                            "clipboard_text_len": 8,
                        },
                        {
                            "check": "text_zero_distance_drag_no_selection",
                            "passed": True,
                            "operation_kind": "drag",
                            "state": "idle",
                            "body_pixel_diff": 0,
                            "clipboard_text_len": 0,
                            "action": "none",
                            "event": "none",
                        }
                    ],
                }
            ]
        ),
        encoding="utf-8",
    )
    return evidence


def write_approved_log(root: Path) -> Path:
    approval = root / "approval.json"
    approval.write_text(
        json.dumps(
            [
                {
                    "page": "text",
                    "approved": True,
                    "approved_by": "hiroyuki",
                    "approved_at": "2026-06-15T12:00:00+09:00",
                    "command": command_for("text"),
                    "smoke_command": smoke_command_for("text"),
                    "minimum_observation_frames": 1,
                    "acceptance_checks": [
                        "text_drag_selection",
                        "text_keyboard_copy",
                        "text_zero_distance_drag_no_selection",
                    ],
                    "acceptance_observations": [
                        "drag text",
                        "copy text",
                        "zero distance drag no-op",
                    ],
                    "acceptance_evidence_contract": text_evidence_contract(),
                    "manual_gate": "do not proceed to the next UI until this page is approved",
                }
            ]
        ),
        encoding="utf-8",
    )
    return approval


def write_unapproved_log(root: Path) -> Path:
    approval = write_approved_log(root)
    payload = json.loads(approval.read_text(encoding="utf-8"))
    payload[0]["approved"] = False
    payload[0]["approved_by"] = ""
    approval.write_text(json.dumps(payload), encoding="utf-8")
    return approval


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


def text_evidence_contract() -> list[dict[str, str]]:
    return [
        {
            "check": "text_drag_selection",
            "operation_kind": "drag",
            "state": "selection=active",
            "action": "select_text",
            "event": "text_selection_changed",
        },
        {
            "check": "text_keyboard_copy",
            "operation_kind": "keyboard",
            "state": "clipboard=selected_text",
            "action": "copy_selection",
            "event": "clipboard_copy",
        },
        {
            "check": "text_zero_distance_drag_no_selection",
            "operation_kind": "drag",
            "state": "idle",
            "action": "none",
            "event": "none",
        },
    ]


if __name__ == "__main__":
    unittest.main()
