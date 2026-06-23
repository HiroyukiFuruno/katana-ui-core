#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_smoke import (
    manual_acceptance_evidence_report,
    manual_acceptance_evidence_report_failures,
    manual_acceptance_smoke_failures,
)


class StorybookManualAcceptanceSmokeTest(unittest.TestCase):
    def test_runs_manual_acceptance_smoke_commands_from_queue(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))
            commands: list[list[str]] = []

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda command: commands.append(command) or 0,
            )

            self.assertEqual([], failures)
            self.assertEqual(
                [
                    [
                        "rtk",
                        "cargo",
                        "run",
                        "--release",
                        "-p",
                        "katana-ui-core-storybook",
                        "--bin",
                        "katana-ui-core-storybook",
                        "--locked",
                        "--",
                        "--open-window",
                        "48",
                        "progress-bar",
                    ]
                ],
                commands,
            )

    def test_reports_failed_manual_acceptance_smoke_command(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(Path(tmp))

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 17,
            )

            self.assertEqual(
                ["progress-bar: smoke_command failed with exit code 17"],
                failures,
            )

    def test_can_run_smoke_for_one_manual_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = write_manifest(
                Path(tmp),
                pages=("text", "progress-bar"),
            )
            commands: list[list[str]] = []

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda command: commands.append(command) or 0,
                pages={"progress-bar"},
            )

            self.assertEqual([], failures)
            self.assertEqual(1, len(commands))
            self.assertEqual("progress-bar", commands[0][-1])
            self.assertEqual("48", commands[0][-2])

    def test_reports_missing_acceptance_audit_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            audit = write_audit(root, [])

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertEqual(
                [
                    "progress-bar: progress_preview_click is missing from live interaction audit",
                    "progress-bar: progress_timed_tick is missing from live interaction audit",
                    "progress-bar: progress_timed_cycle is missing from live interaction audit",
                    "progress-bar: progress_indeterminate_segment_motion is missing from live interaction audit",
                ],
                failures,
            )

    def test_reports_missing_manifest_acceptance_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            payload["ui"][0].pop("acceptance_checks")
            payload["ui"][0].pop("acceptance_observations")
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            audit = write_audit(
                root,
                [
                    preview_click_scenario("percent=82", 120),
                    progress_scenario("progress_timed_tick", "percent=82", 120),
                    progress_scenario("progress_timed_cycle", "percent=0", 120),
                    progress_scenario(
                        "progress_indeterminate_segment_motion",
                        "percent=82",
                        120,
                    ),
                ],
            )

            with self.assertRaisesRegex(ValueError, "acceptance_checks"):
                manual_acceptance_smoke_failures(
                    manifest,
                    lambda _command: 0,
                    audit_path=audit,
                )

    def test_reports_missing_manifest_observation_frame_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            payload["ui"][0].pop("minimum_observation_frames")
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            audit = write_audit(
                root,
                [
                    preview_click_scenario("percent=82", 120),
                    progress_scenario("progress_timed_tick", "percent=82", 120),
                    progress_scenario("progress_timed_cycle", "percent=0", 120),
                    progress_scenario(
                        "progress_indeterminate_segment_motion",
                        "percent=82",
                        120,
                    ),
                ],
            )

            with self.assertRaisesRegex(ValueError, "minimum_observation_frames"):
                manual_acceptance_smoke_failures(
                    manifest,
                    lambda _command: 0,
                    audit_path=audit,
                )

    def test_reports_progress_tick_audit_without_runtime_state(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            audit = write_audit(
                root,
                [
                    preview_click_scenario("percent=82", 120),
                    progress_scenario("progress_timed_tick", "percent=65", 120),
                    progress_scenario("progress_timed_cycle", "percent=0", 120),
                    progress_scenario(
                        "progress_indeterminate_segment_motion",
                        "percent=82",
                        120,
                    ),
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertEqual(
                ["progress-bar: progress_timed_tick must reach state percent=82"],
                failures,
            )

    def test_accepts_progress_bar_when_audit_proves_runtime_motion(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            audit = write_audit(
                root,
                [
                    preview_click_scenario("percent=82", 120),
                    progress_scenario("progress_timed_tick", "percent=82", 120),
                    progress_scenario("progress_timed_cycle", "percent=0", 120),
                    progress_scenario(
                        "progress_indeterminate_segment_motion",
                        "percent=82",
                        120,
                    ),
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertEqual([], failures)

    def test_reports_text_selection_copy_wrong_operation_kind(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            drag = text_scenario("text_drag_selection", 4, 0)
            drag["operation_kind"] = "keyboard"
            copy = text_scenario("text_keyboard_copy", 0, 12)
            copy["operation_kind"] = "drag"
            zero = text_scenario("text_zero_distance_drag_no_selection", 0, 0)
            audit = write_audit(
                root,
                [
                    drag,
                    copy,
                    text_scenario("text_keyboard_paste", 0, 0),
                    zero,
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertEqual(
                [
                    "text: text_drag_selection must be a drag operation",
                    "text: text_keyboard_copy must be a keyboard operation",
                ],
                failures,
            )

    def test_reports_text_drag_selection_without_selection_action_or_event(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            drag = text_scenario("text_drag_selection", 4, 0)
            drag["action"] = "none"
            drag["event"] = "none"
            audit = write_audit(
                root,
                [
                    drag,
                    text_scenario("text_keyboard_copy", 0, 12),
                    text_scenario("text_zero_distance_drag_no_selection", 0, 0),
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertIn(
                "text: text_drag_selection must emit action select_text",
                failures,
            )
            self.assertIn(
                "text: text_drag_selection must emit event text_selection_changed",
                failures,
            )

    def test_reports_text_keyboard_copy_without_copy_action_or_event(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            copy = text_scenario("text_keyboard_copy", 0, 12)
            copy["action"] = "none"
            copy["event"] = "none"
            audit = write_audit(
                root,
                [
                    text_scenario("text_drag_selection", 4, 0),
                    copy,
                    text_scenario("text_zero_distance_drag_no_selection", 0, 0),
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertIn(
                "text: text_keyboard_copy must emit action copy_selection",
                failures,
            )
            self.assertIn(
                "text: text_keyboard_copy must emit event clipboard_copy",
                failures,
            )

    def test_builds_observation_evidence_report_from_manifest_and_audit(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            audit = write_audit(
                root,
                [
                    preview_click_scenario("percent=82", 120),
                    progress_scenario("progress_timed_tick", "percent=82", 120),
                    progress_scenario("progress_timed_cycle", "percent=0", 120),
                    progress_scenario(
                        "progress_indeterminate_segment_motion",
                        "percent=82",
                        120,
                    ),
                ],
            )

            report = manual_acceptance_evidence_report(manifest, audit)

            self.assertEqual("progress-bar", report[0]["page"])
            self.assertEqual(
                "rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window progress-bar",
                report[0]["command"],
            )
            self.assertEqual(
                "rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 48 progress-bar",
                report[0]["smoke_command"],
            )
            self.assertEqual(48, report[0]["minimum_observation_frames"])
            self.assertEqual(
                [
                    "preview click advances meter from 65% to 82%",
                    "meter advances from 65% to 82%",
                    "meter cycles back to 0% after max",
                    "indeterminate segment visibly moves on timed tick",
                ],
                report[0]["acceptance_observations"],
            )
            self.assertEqual(
                [
                    {
                        "check": "progress_preview_click",
                        "passed": True,
                        "operation_kind": "pointer",
                        "state": "percent=82",
                        "action": "progress_change",
                        "event": "progress_changed",
                        "body_pixel_diff": 120,
                    },
                    {
                        "check": "progress_timed_tick",
                        "passed": True,
                        "operation_kind": "timed_tick",
                        "state": "percent=82",
                        "action": "progress_tick",
                        "event": "progress_changed",
                        "body_pixel_diff": 120,
                    },
                    {
                        "check": "progress_timed_cycle",
                        "passed": True,
                        "operation_kind": "timed_tick",
                        "state": "percent=0",
                        "action": "progress_tick",
                        "event": "progress_changed",
                        "body_pixel_diff": 120,
                    },
                    {
                        "check": "progress_indeterminate_segment_motion",
                        "passed": True,
                        "operation_kind": "timed_tick",
                        "state": "percent=82",
                        "action": "progress_tick",
                        "event": "progress_changed",
                        "body_pixel_diff": 120,
                    },
                ],
                report[0]["audit_evidence"],
            )

    def test_evidence_report_can_be_filtered_to_smoked_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text", "progress-bar"))
            audit = write_audit(
                root,
                [
                    text_scenario("text_drag_selection", 4, 0),
                    text_scenario("text_keyboard_copy", 0, 12),
                    preview_click_scenario("percent=82", 120),
                    progress_scenario("progress_timed_tick", "percent=82", 120),
                    progress_scenario("progress_timed_cycle", "percent=0", 120),
                    progress_scenario(
                        "progress_indeterminate_segment_motion",
                        "percent=82",
                        120,
                    ),
                ],
            )

            report = manual_acceptance_evidence_report(
                manifest,
                audit,
                pages={"progress-bar"},
            )

            self.assertEqual(["progress-bar"], [entry["page"] for entry in report])

    def test_evidence_report_copies_text_zero_distance_action_event_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            audit = write_audit(
                root,
                [
                    text_scenario("text_drag_selection", 4, 0),
                    text_scenario("text_keyboard_copy", 0, 12),
                    text_scenario("text_zero_distance_drag_no_selection", 0, 0),
                ],
            )

            report = manual_acceptance_evidence_report(manifest, audit)

            zero = next(
                item
                for item in report[0]["audit_evidence"]
                if item["check"] == "text_zero_distance_drag_no_selection"
            )
            self.assertEqual("drag", zero["operation_kind"])
            self.assertEqual("none", zero["action"])
            self.assertEqual("none", zero["event"])
            self.assertEqual(0, zero["body_pixel_diff"])
            self.assertEqual(0, zero["clipboard_text_len"])

    def test_reports_evidence_file_missing_manual_queue_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text", "progress-bar"))
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "progress-bar",
                            "command": "wrong",
                            "smoke_command": "wrong",
                            "minimum_observation_frames": 1,
                            "acceptance_observations": acceptance_observations_for(
                                "progress-bar"
                            ),
                            "audit_evidence": [
                                {
                                    "check": "progress_preview_click",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_tick",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_cycle",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_indeterminate_segment_motion",
                                    "passed": True,
                                },
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "text: manual acceptance evidence report entry is missing",
                failures,
            )
            self.assertIn(
                "progress-bar: manual acceptance evidence command does not match queue",
                failures,
            )
            self.assertIn(
                "progress-bar: manual acceptance evidence smoke_command does not match queue",
                failures,
            )
            self.assertIn(
                "progress-bar: manual acceptance evidence minimum_observation_frames does not match queue",
                failures,
            )

    def test_reports_evidence_file_missing_expected_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "progress-bar",
                            "command": command_for("progress-bar"),
                            "smoke_command": smoke_command_for("progress-bar"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "progress-bar"
                            ),
                            "acceptance_observations": acceptance_observations_for(
                                "progress-bar"
                            ),
                            "audit_evidence": [
                                {
                                    "check": "progress_preview_click",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_tick",
                                    "passed": True,
                                }
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "progress-bar: manual acceptance evidence missing check progress_timed_cycle",
                failures,
            )

    def test_reports_evidence_file_with_unexpected_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "text",
                            "command": command_for("text"),
                            "smoke_command": smoke_command_for("text"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "text"
                            ),
                            "acceptance_observations": acceptance_observations_for("text"),
                            "audit_evidence": [
                                text_scenario("text_drag_selection", 12, 0)
                                | {"check": "text_drag_selection"},
                                text_scenario("text_keyboard_copy", 0, 8)
                                | {"check": "text_keyboard_copy"},
                                text_scenario(
                                    "text_zero_distance_drag_no_selection",
                                    0,
                                    0,
                                )
                                | {"check": "text_zero_distance_drag_no_selection"},
                                {
                                    "check": "text_stale_extra_check",
                                    "passed": True,
                                },
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "text: manual acceptance evidence has unexpected check text_stale_extra_check",
                failures,
            )

    def test_reports_duplicate_evidence_pages_before_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            evidence = root / "evidence.json"
            valid_entry = {
                "page": "progress-bar",
                "command": command_for("progress-bar"),
                "smoke_command": smoke_command_for("progress-bar"),
                "minimum_observation_frames": minimum_observation_frames_for(
                    "progress-bar"
                ),
                "acceptance_observations": acceptance_observations_for("progress-bar"),
                "audit_evidence": [
                    {
                        "check": "progress_preview_click",
                        "passed": True,
                    },
                    {
                        "check": "progress_timed_tick",
                        "passed": True,
                    },
                    {
                        "check": "progress_timed_cycle",
                        "passed": True,
                    },
                    {
                        "check": "progress_indeterminate_segment_motion",
                        "passed": True,
                    },
                ],
            }
            evidence.write_text(json.dumps([valid_entry, valid_entry]), encoding="utf-8")

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "progress-bar: manual acceptance evidence report entry is duplicated",
                failures,
            )

    def test_reports_unexpected_evidence_page_before_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "button",
                            "command": command_for("button"),
                            "smoke_command": smoke_command_for("button"),
                            "minimum_observation_frames": 1,
                            "acceptance_observations": ["button click"],
                            "audit_evidence": [
                                {
                                    "check": "button_click",
                                    "passed": True,
                                }
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "button: manual acceptance evidence has unexpected page",
                failures,
            )
            self.assertIn(
                "progress-bar: manual acceptance evidence report entry is missing",
                failures,
            )

    def test_reports_future_pending_evidence_page_before_current_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text", "progress-bar"))
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "text",
                            "command": command_for("text"),
                            "smoke_command": smoke_command_for("text"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "text"
                            ),
                            "acceptance_observations": acceptance_observations_for("text"),
                            "audit_evidence": [
                                {
                                    "check": "text_drag_selection",
                                    "passed": True,
                                },
                                {
                                    "check": "text_keyboard_copy",
                                    "passed": True,
                                },
                            ],
                        },
                        {
                            "page": "progress-bar",
                            "command": command_for("progress-bar"),
                            "smoke_command": smoke_command_for("progress-bar"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "progress-bar"
                            ),
                            "acceptance_observations": acceptance_observations_for(
                                "progress-bar"
                            ),
                            "audit_evidence": [
                                {
                                    "check": "progress_preview_click",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_tick",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_cycle",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_indeterminate_segment_motion",
                                    "passed": True,
                                },
                            ],
                        },
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(
                manifest,
                evidence,
                pages={"text"},
            )

            self.assertIn(
                "progress-bar: manual acceptance evidence has future pending page",
                failures,
            )

    def test_reports_duplicate_evidence_checks_before_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "progress-bar",
                            "command": command_for("progress-bar"),
                            "smoke_command": smoke_command_for("progress-bar"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "progress-bar"
                            ),
                            "acceptance_observations": acceptance_observations_for(
                                "progress-bar"
                            ),
                            "audit_evidence": [
                                {
                                    "check": "progress_preview_click",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_preview_click",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_tick",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_timed_cycle",
                                    "passed": True,
                                },
                                {
                                    "check": "progress_indeterminate_segment_motion",
                                    "passed": True,
                                },
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "progress-bar: manual acceptance evidence check progress_preview_click is duplicated",
                failures,
            )

    def test_reports_text_evidence_missing_operation_kind_and_payload_before_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "text",
                            "command": command_for("text"),
                            "smoke_command": smoke_command_for("text"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "text"
                            ),
                            "acceptance_observations": acceptance_observations_for("text"),
                            "audit_evidence": [
                                {
                                    "check": "text_drag_selection",
                                    "passed": True,
                                },
                                {
                                    "check": "text_keyboard_copy",
                                    "passed": True,
                                },
                                {
                                    "check": "text_zero_distance_drag_no_selection",
                                    "passed": True,
                                },
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "text: manual acceptance evidence text_drag_selection must include operation_kind drag",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_drag_selection must include positive body_pixel_diff",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_drag_selection must include action select_text",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_drag_selection must include event text_selection_changed",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_drag_selection must include state selection=active",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_keyboard_copy must include operation_kind keyboard",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_keyboard_copy must include positive clipboard_text_len",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_keyboard_copy must include action copy_selection",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_keyboard_copy must include event clipboard_copy",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_keyboard_copy must include state clipboard=selected_text",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_zero_distance_drag_no_selection must include operation_kind drag",
                failures,
            )

    def test_reports_text_zero_distance_evidence_with_selection_action_before_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "text",
                            "command": command_for("text"),
                            "smoke_command": smoke_command_for("text"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "text"
                            ),
                            "acceptance_observations": acceptance_observations_for("text"),
                            "audit_evidence": [
                                text_scenario("text_drag_selection", 12, 0)
                                | {"check": "text_drag_selection"},
                                text_scenario("text_keyboard_copy", 0, 8)
                                | {"check": "text_keyboard_copy"},
                                text_scenario(
                                    "text_zero_distance_drag_no_selection",
                                    0,
                                    0,
                                )
                                | {
                                    "check": "text_zero_distance_drag_no_selection",
                                    "state": "selecting",
                                    "action": "select_text",
                                    "event": "text_selection_changed",
                                },
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "text: manual acceptance evidence text_zero_distance_drag_no_selection must include action none",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_zero_distance_drag_no_selection must include event none",
                failures,
            )
            self.assertIn(
                "text: manual acceptance evidence text_zero_distance_drag_no_selection must include state idle",
                failures,
            )

    def test_reports_checkbox_live_audit_missing_pointer_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("checkbox",))
            audit = write_audit(
                root,
                [
                    {
                        "page": "checkbox",
                        "operation": "checkbox_pointer_checks_both_rows",
                        "passed": True,
                    }
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertIn(
                "checkbox: checkbox_pointer_checks_both_rows must be a pointer operation",
                failures,
            )
            self.assertIn(
                "checkbox: checkbox_pointer_checks_both_rows must use action checkbox_toggle",
                failures,
            )
            self.assertIn(
                "checkbox: checkbox_pointer_checks_both_rows must emit event checked_changed",
                failures,
            )
            self.assertIn(
                "checkbox: checkbox_pointer_checks_both_rows must change rendered checkbox pixels",
                failures,
            )

    def test_reports_text_live_audit_zero_distance_drag_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("text",))
            audit = write_audit(
                root,
                [
                    text_scenario("text_drag_selection", 12, 0),
                    text_scenario("text_keyboard_copy", 0, 8),
                    {
                        "page": "text",
                        "operation": "text_zero_distance_drag_no_selection",
                        "operation_kind": "drag",
                        "passed": True,
                        "state": "selecting",
                        "action": "select_text",
                        "event": "text_selection_changed",
                        "body_pixel_diff": 2,
                        "clipboard_text_len": 1,
                    },
                ],
            )

            failures = manual_acceptance_smoke_failures(
                manifest,
                lambda _command: 0,
                audit_path=audit,
            )

            self.assertIn(
                "text: text_zero_distance_drag_no_selection must not change rendered selection pixels",
                failures,
            )
            self.assertIn(
                "text: text_zero_distance_drag_no_selection must not copy text to clipboard",
                failures,
            )
            self.assertIn(
                "text: text_zero_distance_drag_no_selection must not emit a selection action",
                failures,
            )
            self.assertIn(
                "text: text_zero_distance_drag_no_selection must not emit a selection event",
                failures,
            )
            self.assertIn(
                "text: text_zero_distance_drag_no_selection must remain state idle",
                failures,
            )

    def test_reports_checkbox_evidence_missing_pointer_payload_before_approval(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root, pages=("checkbox",))
            evidence = root / "evidence.json"
            evidence.write_text(
                json.dumps(
                    [
                        {
                            "page": "checkbox",
                            "command": command_for("checkbox"),
                            "smoke_command": smoke_command_for("checkbox"),
                            "minimum_observation_frames": minimum_observation_frames_for(
                                "checkbox"
                            ),
                            "acceptance_observations": acceptance_observations_for(
                                "checkbox"
                            ),
                            "audit_evidence": [
                                {
                                    "check": "checkbox_pointer_checks_both_rows",
                                    "passed": True,
                                }
                            ],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_evidence_report_failures(manifest, evidence)

            self.assertIn(
                "checkbox: manual acceptance evidence checkbox_pointer_checks_both_rows must include operation_kind pointer",
                failures,
            )
            self.assertIn(
                "checkbox: manual acceptance evidence checkbox_pointer_checks_both_rows must include action checkbox_toggle",
                failures,
            )
            self.assertIn(
                "checkbox: manual acceptance evidence checkbox_pointer_checks_both_rows must include event checked_changed",
                failures,
            )
            self.assertIn(
                "checkbox: manual acceptance evidence checkbox_pointer_checks_both_rows must include positive body_pixel_diff",
                failures,
            )


def write_manifest(root: Path, pages: tuple[str, ...] = ("progress-bar",)) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    {
                        "page": page,
                        "audit_status": "partial",
                        "manual_acceptance_order": manual_acceptance_order_for(page),
                        "dependency_layer": dependency_layer_for(page),
                        "depends_on": depends_on_for(page),
                        "required_operations": required_operations_for(page),
                        "minimum_observation_frames": minimum_observation_frames_for(page),
                        "acceptance_checks": acceptance_checks_for(page),
                        "acceptance_observations": acceptance_observations_for(page),
                        "gaps": [
                            "manual_acceptance_pending: user confirmation is required"
                        ],
                    }
                    for page in pages
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


def required_operations_for(page: str) -> list[str]:
    if page == "progress-bar":
        return ["pointer", "timed_tick"]
    return ["pointer", "keyboard"]


def manual_acceptance_order_for(page: str) -> int:
    if page == "progress-bar":
        return 30
    if page == "checkbox":
        return 20
    return 10


def dependency_layer_for(page: str) -> str:
    if page == "progress-bar":
        return "feedback-motion-meter"
    if page == "checkbox":
        return "binary-choice-state-display"
    return "foundation-text-selection"


def depends_on_for(page: str) -> list[str]:
    if page == "progress-bar":
        return ["text"]
    if page == "checkbox":
        return ["text"]
    return []


def minimum_observation_frames_for(page: str) -> int:
    if page == "progress-bar":
        return 48
    return 1


def command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window {page}"
    )


def smoke_command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        "katana-ui-core-storybook --locked -- --open-window "
        f"{minimum_observation_frames_for(page)} {page}"
    )


def acceptance_checks_for(page: str) -> list[str]:
    if page == "progress-bar":
        return [
            "progress_preview_click",
            "progress_timed_tick",
            "progress_timed_cycle",
            "progress_indeterminate_segment_motion",
        ]
    if page == "checkbox":
        return ["checkbox_pointer_checks_both_rows"]
    return [
        "text_drag_selection",
        "text_keyboard_copy",
        "text_keyboard_paste",
        "text_zero_distance_drag_no_selection",
    ]


def acceptance_observations_for(page: str) -> list[str]:
    if page == "progress-bar":
        return [
            "preview click advances meter from 65% to 82%",
            "meter advances from 65% to 82%",
            "meter cycles back to 0% after max",
            "indeterminate segment visibly moves on timed tick",
        ]
    if page == "checkbox":
        return ["Both rows toggle through core checkbox pointer events"]
    return [
        "Drag creates a visible text selection highlight",
        "Copy exports selected text",
        "Paste is ignored for display Text through the same keyboard clipboard contract",
        "Zero-distance drag does not create a selection action, highlight, or copy payload",
    ]


def write_audit(root: Path, scenarios: list[dict[str, object]]) -> Path:
    audit = root / "audit.json"
    audit.write_text(json.dumps({"scenarios": scenarios}), encoding="utf-8")
    return audit


def progress_scenario(
    operation: str,
    state: str,
    body_pixel_diff: int,
) -> dict[str, object]:
    return {
        "page": "progress-bar",
        "operation": operation,
        "operation_kind": "timed_tick",
        "passed": True,
        "state": state,
        "action": "progress_tick",
        "event": "progress_changed",
        "body_pixel_diff": body_pixel_diff,
    }


def preview_click_scenario(state: str, body_pixel_diff: int) -> dict[str, object]:
    return {
        "page": "progress-bar",
        "operation": "preview_click",
        "operation_kind": "pointer",
        "passed": True,
        "state": state,
        "action": "progress_change",
        "event": "progress_changed",
        "body_pixel_diff": body_pixel_diff,
    }


def text_scenario(
    operation: str,
    body_pixel_diff: int,
    clipboard_text_len: int,
) -> dict[str, object]:
    action = "none"
    event = "none"
    state = "idle"
    if operation == "text_drag_selection":
        state = "selection=active"
        action = "select_text"
        event = "text_selection_changed"
    if operation == "text_keyboard_copy":
        state = "clipboard=selected_text"
        action = "copy_selection"
        event = "clipboard_copy"
    operation_kind = (
        "keyboard"
        if operation in ("text_keyboard_copy", "text_keyboard_paste")
        else "drag"
    )
    return {
        "page": "text",
        "operation": operation,
        "operation_kind": operation_kind,
        "passed": True,
        "state": state,
        "action": action,
        "event": event,
        "body_pixel_diff": body_pixel_diff,
        "clipboard_text_len": clipboard_text_len,
    }


if __name__ == "__main__":
    unittest.main()
