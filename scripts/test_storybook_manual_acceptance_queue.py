#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_queue import format_queue_entry, manual_acceptance_queue


class StorybookManualAcceptanceQueueTest(unittest.TestCase):
    def test_queue_contains_only_manual_acceptance_pending_pages(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            manifest.write_text(
                json.dumps(
                    {
                        "ui": [
                            {
                                "page": "text",
                                "audit_status": "partial",
                                "manual_acceptance_order": 10,
                                "dependency_layer": "foundation-text",
                                "depends_on": [],
                                "required_operations": ["pointer", "drag", "keyboard"],
                                "acceptance_checks": [
                                    "text_drag_selection",
                                    "text_keyboard_copy",
                                    "text_zero_distance_drag_no_selection",
                                ],
                                "acceptance_observations": [
                                    "manifest text selection observation",
                                    "manifest clipboard copy observation",
                                    "manifest zero-distance drag no-op observation",
                                ],
                                "minimum_observation_frames": 1,
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
                            {
                                "page": "progress-bar",
                                "audit_status": "partial",
                                "manual_acceptance_order": 30,
                                "dependency_layer": "feedback-motion",
                                "depends_on": ["text"],
                                "required_operations": ["pointer", "timed_tick"],
                                "acceptance_checks": [
                                    "progress_preview_click",
                                    "progress_timed_tick",
                                    "progress_timed_cycle",
                                    "progress_indeterminate_segment_motion",
                                ],
                                "acceptance_observations": [
                                    "manifest progress preview click observation",
                                    "manifest progress tick observation",
                                    "manifest progress cycle observation",
                                    "manifest progress segment motion observation",
                                ],
                                "minimum_observation_frames": 96,
                                "gaps": [
                                    "manual_acceptance_pending: user confirmation is required"
                                ],
                            },
                        ]
                    }
                ),
                encoding="utf-8",
            )

            queue = manual_acceptance_queue(manifest)

            self.assertEqual(["text", "progress-bar"], [entry["page"] for entry in queue])
            self.assertEqual(10, queue[0]["manual_acceptance_order"])
            self.assertEqual("foundation-text", queue[0]["dependency_layer"])
            self.assertEqual([], queue[0]["depends_on"])
            self.assertEqual(["text"], queue[1]["depends_on"])
            self.assertEqual(
                "do not proceed to the next UI until this page is approved",
                queue[0]["manual_gate"],
            )
            self.assertEqual(
                "rtk cargo run --release -p katana-ui-core-storybook --bin "
                "katana-ui-core-storybook --locked -- --open-window text",
                queue[0]["command"],
            )
            self.assertEqual(
                "rtk cargo run --release -p katana-ui-core-storybook --bin "
                "katana-ui-core-storybook --locked -- --open-window 1 text",
                queue[0]["smoke_command"],
            )
            self.assertEqual(["pointer", "drag", "keyboard"], queue[0]["required_operations"])
            self.assertEqual(["pointer", "timed_tick"], queue[1]["required_operations"])
            self.assertEqual(96, queue[1]["minimum_observation_frames"])
            self.assertIn("progress_preview_click", queue[1]["acceptance_checks"])
            self.assertIn("progress_timed_cycle", queue[1]["acceptance_checks"])
            self.assertIn(
                "progress_indeterminate_segment_motion",
                queue[1]["acceptance_checks"],
            )
            self.assertIn(
                "manifest progress tick observation",
                queue[1]["acceptance_observations"],
            )
            self.assertIn(
                "manifest clipboard copy observation",
                queue[0]["acceptance_observations"],
            )
            self.assertEqual(
                "rtk cargo run --release -p katana-ui-core-storybook --bin "
                "katana-ui-core-storybook --locked -- --open-window 96 progress-bar",
                queue[1]["smoke_command"],
            )
            self.assertEqual(
                "progress-bar\toperations=pointer,timed_tick\t"
                "order=30\tlayer=feedback-motion\tdepends_on=text\t"
                "checks=progress_preview_click,progress_timed_tick,progress_timed_cycle,progress_indeterminate_segment_motion\t"
                "observe=manifest progress preview click observation;manifest progress tick observation;manifest progress cycle observation;manifest progress segment motion observation\t"
                "manual_gate=do not proceed to the next UI until this page is approved\t"
                "command=rtk cargo run --release -p katana-ui-core-storybook --bin "
                "katana-ui-core-storybook --locked -- --open-window progress-bar\t"
                "smoke=rtk cargo run --release -p katana-ui-core-storybook --bin "
                "katana-ui-core-storybook --locked -- --open-window 96 progress-bar",
                format_queue_entry(queue[1]),
            )

    def test_queue_sorts_by_manual_acceptance_order_not_manifest_position(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            manifest.write_text(
                json.dumps(
                    {
                        "ui": [
                            pending_entry("checkbox", 20, "binary-choice", ["text"]),
                            pending_entry("text", 10, "foundation-text", []),
                        ]
                    }
                ),
                encoding="utf-8",
            )

            queue = manual_acceptance_queue(manifest)

            self.assertEqual(["text", "checkbox"], [entry["page"] for entry in queue])

    def test_manifest_pending_entries_require_dependency_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            manifest.write_text(
                json.dumps({"ui": [pending_entry("text", None, "", None)]}),
                encoding="utf-8",
            )

            with self.assertRaisesRegex(ValueError, "manual_acceptance_order"):
                manual_acceptance_queue(manifest)

    def test_manifest_pending_entries_reject_verified_status(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            entry = pending_entry("text", 10, "foundation-text", [])
            entry["audit_status"] = "verified"
            manifest.write_text(json.dumps({"ui": [entry]}), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "audit_status"):
                manual_acceptance_queue(manifest)

    def test_manifest_pending_entries_require_positive_minimum_observation_frames(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            entry = pending_entry("text", 10, "foundation-text", [])
            entry["minimum_observation_frames"] = 0
            manifest.write_text(json.dumps({"ui": [entry]}), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "minimum_observation_frames"):
                manual_acceptance_queue(manifest)

    def test_manifest_pending_entries_require_strict_acceptance_checks(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            entry = pending_entry("text", 10, "foundation-text", [])
            entry["acceptance_checks"] = ["text_drag_selection", "", 42]
            manifest.write_text(json.dumps({"ui": [entry]}), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "acceptance_checks"):
                manual_acceptance_queue(manifest)

    def test_manifest_pending_entries_reject_duplicate_acceptance_checks(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            entry = pending_entry("text", 10, "foundation-text", [])
            entry["acceptance_checks"] = ["text_drag_selection", "text_drag_selection"]
            manifest.write_text(json.dumps({"ui": [entry]}), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "acceptance_checks"):
                manual_acceptance_queue(manifest)

    def test_manifest_pending_entries_require_strict_acceptance_observations(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            entry = pending_entry("text", 10, "foundation-text", [])
            entry["acceptance_observations"] = ["drag text", "", 42]
            manifest.write_text(json.dumps({"ui": [entry]}), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "acceptance_observations"):
                manual_acceptance_queue(manifest)

    def test_manifest_pending_entries_reject_duplicate_required_operations(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            entry = pending_entry("text", 10, "foundation-text", [])
            entry["required_operations"] = ["pointer", "pointer"]
            manifest.write_text(json.dumps({"ui": [entry]}), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "required_operations"):
                manual_acceptance_queue(manifest)

    def test_checked_in_manifest_has_no_release_blocking_manual_acceptance_queue(self) -> None:
        manifest_path = Path("docs/storybook-77ui-interaction-manifest.json")
        queue = manual_acceptance_queue(manifest_path)

        self.assertEqual([], [entry["page"] for entry in queue])

        payload = json.loads(manifest_path.read_text(encoding="utf-8"))
        entries = {
            entry["page"]: entry
            for entry in payload["ui"]
            if entry["page"] in {"text", "checkbox", "progress-bar", "tooltip", "modal", "tree-view"}
        }
        self.assertEqual(
            {
                "text": (10, []),
                "checkbox": (20, ["text"]),
                "progress-bar": (30, ["text"]),
                "tooltip": (40, ["text", "checkbox"]),
                "modal": (50, ["text", "checkbox", "tooltip"]),
                "tree-view": (60, ["text", "checkbox"]),
            },
            {
                page: (entry["manual_acceptance_order"], entry["depends_on"])
                for page, entry in entries.items()
            },
        )


def pending_entry(
    page: str,
    manual_acceptance_order: int | None,
    dependency_layer: str,
    depends_on: list[str] | None,
) -> dict[str, object]:
    entry: dict[str, object] = {
        "page": page,
        "audit_status": "partial",
        "required_operations": ["pointer"],
        "acceptance_checks": [f"{page}_check"],
        "acceptance_observations": [f"{page} observation"],
        "minimum_observation_frames": 1,
        "gaps": ["manual_acceptance_pending: user confirmation is required"],
    }
    if manual_acceptance_order is not None:
        entry["manual_acceptance_order"] = manual_acceptance_order
    if dependency_layer:
        entry["dependency_layer"] = dependency_layer
    if depends_on is not None:
        entry["depends_on"] = depends_on
    return entry


if __name__ == "__main__":
    unittest.main()
