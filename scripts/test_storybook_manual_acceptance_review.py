#!/usr/bin/env python3
from __future__ import annotations

import json
import contextlib
import io
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_review import (
    format_review_entry,
    manual_acceptance_review_entries,
    review_manual_acceptance,
)


class StorybookManualAcceptanceReviewTest(unittest.TestCase):
    def test_review_entries_follow_manual_acceptance_queue_and_page_filter(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            entries = manual_acceptance_review_entries(manifest, {"checkbox"})

            self.assertEqual(["checkbox"], [entry["page"] for entry in entries])
            self.assertIn("checkbox_pointer_checks_both_rows", entries[0]["acceptance_checks"])
            self.assertIn("both checkbox rows checked", entries[0]["acceptance_observations"])

    def test_format_review_entry_prints_observations_commands_and_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            entry = manual_acceptance_review_entries(Path(write_manifest(Path(tmp))), {"text"})[0]

            output = format_review_entry(1, 1, entry)

            self.assertIn("[1/1] text", output)
            self.assertIn("operations: pointer, drag, keyboard", output)
            self.assertIn("Drag creates a visible text selection highlight", output)
            self.assertIn("manual gate:", output)
            self.assertIn("do not proceed to the next UI until this page is approved", output)
            self.assertIn("--open-window text", output)
            self.assertIn("--open-window 1 text", output)

    def test_open_mode_runs_only_selected_page_commands(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))
            commands: list[list[str]] = []
            captured = io.StringIO()

            with contextlib.redirect_stdout(captured):
                result = review_manual_acceptance(
                    manifest,
                    {"checkbox"},
                    True,
                    lambda command: commands.append(command) or 0,
                )

            self.assertEqual(0, result)
            self.assertEqual(1, len(commands))
            self.assertEqual("checkbox", commands[0][-1])
            self.assertIn("[1/1] checkbox", captured.getvalue())

    def test_open_mode_reports_failed_command(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))
            captured = io.StringIO()

            with contextlib.redirect_stdout(captured):
                result = review_manual_acceptance(
                    manifest,
                    {"text"},
                    True,
                    lambda _command: 9,
                )

            self.assertEqual(1, result)
            self.assertIn("storybook manual acceptance review failed", captured.getvalue())
            self.assertIn("text: command failed with exit code 9", captured.getvalue())


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
                            "Drag creates a visible text selection highlight",
                            "Copy exports selected text",
                            "Zero-distance drag does not create a selection action, highlight, or copy payload",
                        ],
                        "gaps": [
                            "manual_acceptance_pending: user confirmation is required"
                        ],
                    },
                    {
                        "page": "checkbox",
                        "audit_status": "partial",
                        "manual_acceptance_order": 20,
                        "dependency_layer": "binary-choice-state-display",
                        "depends_on": ["text"],
                        "required_operations": ["pointer", "keyboard", "focus", "hover"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": [
                            "checkbox_pointer_checks_both_rows",
                            "checkbox_keyboard_toggle_off",
                        ],
                        "acceptance_observations": [
                            "both checkbox rows checked",
                            "second keyboard toggle removes the mark",
                        ],
                        "gaps": [
                            "manual_acceptance_pending: user confirmation is required"
                        ],
                    },
                    {
                        "page": "button",
                        "audit_status": "verified",
                        "required_operations": ["pointer"],
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
