#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_mark_approved import mark_manual_acceptance_approved


class StorybookManualAcceptanceMarkApprovedTest(unittest.TestCase):
    def test_creates_approved_entry_from_current_pending_queue(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            approval_log = root / "approval.json"

            output = mark_manual_acceptance_approved(
                manifest,
                approval_log,
                {"text"},
                "hiroyuki",
                "2026-06-15T12:00:00+09:00",
            )

            entries = json.loads(approval_log.read_text(encoding="utf-8"))
            self.assertEqual(["text"], [entry["page"] for entry in entries])
            text = entries[0]
            self.assertIs(text["approved"], True)
            self.assertEqual("hiroyuki", text["approved_by"])
            self.assertEqual("2026-06-15T12:00:00+09:00", text["approved_at"])
            self.assertIn("--open-window text", text["command"])
            self.assertIn("--open-window 1 text", text["smoke_command"])
            self.assertEqual(1, text["minimum_observation_frames"])
            self.assertEqual(
                [
                    "text_drag_selection",
                    "text_keyboard_copy",
                    "text_zero_distance_drag_no_selection",
                ],
                text["acceptance_checks"],
            )
            self.assertEqual(
                ["drag text", "copy text", "zero distance drag no-op"],
                text["acceptance_observations"],
            )
            self.assertEqual(text_evidence_contract(), text["acceptance_evidence_contract"])
            self.assertEqual(
                "do not proceed to the next UI until this page is approved",
                text["manual_gate"],
            )
            self.assertEqual(["text: marked approved"], output)

    def test_rejects_existing_future_pending_approval_before_writing_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            approval_log = root / "approval.json"
            approval_log.write_text(
                json.dumps(
                    [
                        {
                            "page": "checkbox",
                            "approved": True,
                            "approved_by": "user",
                            "approved_at": "2026-06-15T11:00:00+09:00",
                        },
                        {
                            "page": "text",
                            "approved": False,
                            "command": "stale",
                        },
                    ]
                ),
                encoding="utf-8",
            )

            before = approval_log.read_text(encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "checkbox: approval log has future pending page"):
                mark_manual_acceptance_approved(
                    manifest,
                    approval_log,
                    {"text"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )
            self.assertEqual(before, approval_log.read_text(encoding="utf-8"))

    def test_rejects_existing_approval_log_with_duplicate_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            approval_log = root / "approval.json"
            approval_log.write_text(
                json.dumps(
                    [
                        {"page": "text", "approved": True},
                        {"page": "text", "approved": True},
                    ]
                ),
                encoding="utf-8",
            )

            with self.assertRaisesRegex(ValueError, "text: approval log entry is duplicated"):
                mark_manual_acceptance_approved(
                    manifest,
                    approval_log,
                    {"text"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

    def test_rejects_existing_approval_log_with_unexpected_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            approval_log = root / "approval.json"
            approval_log.write_text(
                json.dumps([{"page": "button", "approved": True}]),
                encoding="utf-8",
            )

            with self.assertRaisesRegex(ValueError, "button: approval log has unexpected page"):
                mark_manual_acceptance_approved(
                    manifest,
                    approval_log,
                    {"text"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

    def test_rejects_non_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)

            with self.assertRaisesRegex(ValueError, "button"):
                mark_manual_acceptance_approved(
                    manifest,
                    root / "approval.json",
                    {"button"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

    def test_rejects_out_of_order_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)

            with self.assertRaisesRegex(ValueError, "next pending page is text"):
                mark_manual_acceptance_approved(
                    manifest,
                    root / "approval.json",
                    {"checkbox"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

    def test_rejects_next_page_with_pending_dependency_before_writing_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_blocked_manifest(root)
            approval_log = root / "approval.json"

            with self.assertRaisesRegex(ValueError, "checkbox depends on pending pages: text"):
                mark_manual_acceptance_approved(
                    manifest,
                    approval_log,
                    {"checkbox"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

            self.assertFalse(approval_log.exists())

    def test_rejects_multiple_pages_at_once(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)

            with self.assertRaisesRegex(ValueError, "one page at a time"):
                mark_manual_acceptance_approved(
                    manifest,
                    root / "approval.json",
                    {"text", "checkbox"},
                    "hiroyuki",
                    "2026-06-15T12:00:00+09:00",
                )

    def test_rejects_missing_approval_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)

            with self.assertRaisesRegex(ValueError, "approved_by"):
                mark_manual_acceptance_approved(
                    manifest,
                    root / "approval.json",
                    {"checkbox"},
                    "",
                    "2026-06-15T12:00:00+09:00",
                )
            with self.assertRaisesRegex(ValueError, "approved_at"):
                mark_manual_acceptance_approved(
                    manifest,
                    root / "approval.json",
                    {"checkbox"},
                    "hiroyuki",
                    "",
                )

    def test_rejects_non_iso_or_timezone_less_approved_at(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)

            for approved_at in ["today", "2026-06-15T12:00:00"]:
                with self.subTest(approved_at=approved_at):
                    with self.assertRaisesRegex(ValueError, "approved_at"):
                        mark_manual_acceptance_approved(
                            manifest,
                            root / "approval.json",
                            {"text"},
                            "hiroyuki",
                            approved_at,
                        )

    def test_dry_run_does_not_write_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            approval_log = root / "approval.json"

            output = mark_manual_acceptance_approved(
                manifest,
                approval_log,
                {"text"},
                "hiroyuki",
                "2026-06-15T12:00:00+09:00",
                dry_run=True,
            )

            self.assertFalse(approval_log.exists())
            self.assertIn("dry-run: no files changed", output)


def write_manifest(root: Path) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
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
                    {
                        "page": "checkbox",
                        "audit_status": "partial",
                        "manual_acceptance_order": 20,
                        "dependency_layer": "binary-choice-state-display",
                        "depends_on": ["text"],
                        "required_operations": ["pointer"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": ["checkbox_pointer_checks_both_rows"],
                        "acceptance_observations": ["both checked"],
                        "gaps": ["manual_acceptance_pending: checkbox pending"],
                    },
                    {
                        "page": "button",
                        "audit_status": "verified",
                        "gaps": [],
                    },
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


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
                        "required_operations": ["pointer"],
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


if __name__ == "__main__":
    unittest.main()
