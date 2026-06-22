#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_next import (
    next_manual_acceptance_entry,
    run_next_manual_acceptance,
)


class StorybookManualAcceptanceNextTest(unittest.TestCase):
    def test_next_entry_is_first_pending_page_by_manual_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            entry = next_manual_acceptance_entry(manifest)

            self.assertIsNotNone(entry)
            assert entry is not None
            self.assertEqual("text", entry["page"])
            self.assertEqual(10, entry["manual_acceptance_order"])
            self.assertEqual("foundation-text-selection", entry["dependency_layer"])
            self.assertEqual([], entry["depends_on"])

    def test_run_prints_only_next_entry_without_opening(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))
            commands: list[list[str]] = []

            result, output = run_next_manual_acceptance(
                manifest,
                open_page=False,
                runner=lambda command: commands.append(command) or 0,
            )

            self.assertEqual(0, result)
            self.assertEqual([], commands)
            self.assertIn("[1/1] text", output)
            self.assertIn("layer: foundation-text-selection", output)
            self.assertIn("depends_on: ", output)
            self.assertIn("--open-window text", output)
            self.assertIn("after user OK only:", output)
            self.assertIn("approval template:", output)
            self.assertIn("rtk just storybook-manual-acceptance-approval-template", output)
            self.assertIn("manual gate:", output)
            self.assertEqual(1, output.count("manual gate:"))
            self.assertIn("do not proceed to the next UI until this page is approved", output)
            self.assertIn(
                "rtk just storybook-manual-acceptance-complete-next <approved_by> <approved_at>",
                output,
            )
            self.assertIn("evidence contract:", output)
            self.assertIn(
                "text_drag_selection operation_kind=drag state=selection=active action=select_text event=text_selection_changed",
                output,
            )
            self.assertIn(
                "text_keyboard_copy operation_kind=keyboard state=clipboard=selected_text action=copy_selection event=clipboard_copy",
                output,
            )
            self.assertIn(
                "text_zero_distance_drag_no_selection operation_kind=drag state=idle action=none event=none",
                output,
            )
            self.assertNotIn("storybook-manual-acceptance-mark-approved", output)
            self.assertNotIn("storybook-manual-acceptance-approve text", output)
            self.assertNotIn("[1/1] checkbox", output)

    def test_run_open_executes_only_next_entry_command(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))
            commands: list[list[str]] = []

            result, output = run_next_manual_acceptance(
                manifest,
                open_page=True,
                runner=lambda command: commands.append(command) or 0,
            )

            self.assertEqual(0, result)
            self.assertIn("[1/1] text", output)
            self.assertEqual(1, len(commands))
            self.assertEqual("text", commands[0][-1])

    def test_run_open_reports_command_failure(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            result, output = run_next_manual_acceptance(
                manifest,
                open_page=True,
                runner=lambda _command: 7,
            )

            self.assertEqual(1, result)
            self.assertIn("text: command failed with exit code 7", output)

    def test_run_reports_no_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "manifest.json"
            manifest.write_text(
                json.dumps({"ui": [{"page": "text", "audit_status": "verified", "gaps": []}]}),
                encoding="utf-8",
            )

            result, output = run_next_manual_acceptance(
                manifest,
                open_page=True,
                runner=lambda _command: 0,
            )

            self.assertEqual(0, result)
            self.assertIn("no pending manual acceptance page", output)

    def test_next_entry_rejects_pending_dependency_before_opening_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest_with_blocked_first_entry(Path(tmp))

            with self.assertRaisesRegex(ValueError, "checkbox depends on pending pages: text"):
                next_manual_acceptance_entry(manifest)

            result, output = run_next_manual_acceptance(
                manifest,
                open_page=True,
                runner=lambda _command: 0,
            )

            self.assertEqual(1, result)
            self.assertIn("checkbox depends on pending pages: text", output)


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
                        "required_operations": ["pointer"],
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


def write_manifest_with_blocked_first_entry(root: Path) -> Path:
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
                        "acceptance_evidence_contract": text_evidence_contract(),
                        "gaps": ["manual_acceptance_pending: text pending"],
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


if __name__ == "__main__":
    unittest.main()
