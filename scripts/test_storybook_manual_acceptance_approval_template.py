#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_approval_template import (
    approval_template_entries,
    write_approval_template,
)
from storybook_manual_acceptance_next import next_manual_acceptance_entry


class StorybookManualAcceptanceApprovalTemplateTest(unittest.TestCase):
    def test_template_entries_follow_next_pending_queue_and_start_unapproved(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            entries = approval_template_entries(
                manifest,
                {"text"},
                approved_by="user",
                approved_at="2026-06-15T00:00:00+09:00",
            )

            self.assertEqual(["text"], [entry["page"] for entry in entries])
            self.assertFalse(entries[0]["approved"])
            self.assertEqual("user", entries[0]["approved_by"])
            self.assertEqual("2026-06-15T00:00:00+09:00", entries[0]["approved_at"])
            self.assertIn("--open-window text", entries[0]["command"])
            self.assertIn("--open-window 1 text", entries[0]["smoke_command"])
            self.assertEqual(1, entries[0]["minimum_observation_frames"])
            self.assertEqual(
                [
                    "text_drag_selection",
                    "text_keyboard_copy",
                    "text_zero_distance_drag_no_selection",
                ],
                entries[0]["acceptance_checks"],
            )
            self.assertEqual(
                [
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
                ],
                entries[0]["acceptance_evidence_contract"],
            )
            self.assertEqual(
                "do not proceed to the next UI until this page is approved",
                entries[0]["manual_gate"],
            )
            self.assertIn("Set approved=true only after user manually confirms", entries[0]["notes"])

    def test_template_rejects_out_of_order_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            with self.assertRaisesRegex(ValueError, "next pending page is text"):
                approval_template_entries(manifest, {"checkbox"})

    def test_template_defaults_to_next_pending_page_only(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            entries = approval_template_entries(manifest, set())

            self.assertEqual(["text"], [entry["page"] for entry in entries])

    def test_template_rejects_non_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            with self.assertRaisesRegex(ValueError, "button"):
                approval_template_entries(manifest, {"button"})

    def test_write_template_does_not_overwrite_without_force(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / "approval.json"
            output.write_text("[]\n", encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "already exists"):
                write_approval_template(output, [{"page": "text"}])

    def test_write_template_allows_force_overwrite(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / "approval.json"
            output.write_text("[]\n", encoding="utf-8")

            write_approval_template(output, [{"page": "text", "approved": False}], force=True)

            self.assertEqual(
                [{"page": "text", "approved": False}],
                json.loads(output.read_text(encoding="utf-8")),
            )

    def test_checked_in_example_starts_unapproved(self) -> None:
        example = Path("docs/storybook-manual-acceptance-approvals.example.json")
        entries = json.loads(example.read_text(encoding="utf-8"))
        next_entry = next_manual_acceptance_entry(
            Path("docs/storybook-77ui-interaction-manifest.json")
        )

        if next_entry is None:
            self.assertEqual([], entries)
            return

        self.assertEqual(1, len(entries))
        for entry in entries:
            self.assertIs(entry.get("approved"), False)
            self.assertEqual("", entry.get("approved_by"))
            self.assertEqual("", entry.get("approved_at"))
            self.assertIn("command", entry)
            self.assertIn("smoke_command", entry)
            self.assertIn("minimum_observation_frames", entry)
            self.assertEqual(
                "do not proceed to the next UI until this page is approved",
                entry.get("manual_gate"),
            )
            self.assertIn("Set approved=true only after user manually confirms", entry["notes"])

    def test_checked_in_example_matches_current_next_pending_page(self) -> None:
        example = Path("docs/storybook-manual-acceptance-approvals.example.json")
        entries = json.loads(example.read_text(encoding="utf-8"))
        next_entry = next_manual_acceptance_entry(
            Path("docs/storybook-77ui-interaction-manifest.json")
        )
        if next_entry is None:
            self.assertEqual([], entries)
            return

        self.assertEqual(1, len(entries))
        self.assertEqual(next_entry["page"], entries[0]["page"])
        self.assertEqual(next_entry["command"], entries[0]["command"])
        self.assertEqual(next_entry["smoke_command"], entries[0]["smoke_command"])
        self.assertEqual(
            next_entry["acceptance_checks"],
            entries[0]["acceptance_checks"],
        )
        self.assertEqual(
            next_entry["acceptance_observations"],
            entries[0]["acceptance_observations"],
        )
        self.assertEqual(
            next_entry["acceptance_evidence_contract"],
            entries[0]["acceptance_evidence_contract"],
        )
        self.assertEqual(next_entry["manual_gate"], entries[0]["manual_gate"])


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
                        "required_operations": ["pointer", "drag"],
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
                        "acceptance_evidence_contract": [
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
                        ],
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


if __name__ == "__main__":
    unittest.main()
