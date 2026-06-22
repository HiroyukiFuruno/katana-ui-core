#!/usr/bin/env python3
from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from storybook_manifest_interaction_smoke import manifest_smoke_failures


class StorybookManifestInteractionSmokeTest(unittest.TestCase):
    def test_rejects_unverified_manifest_entry(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(root, status="unverified")

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "button: audit_status must be verified before interaction smoke passes",
                failures,
            )

    def test_accepts_verified_manifest_entry_with_live_audit(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(root, status="verified")

            self.assertEqual([], manifest_smoke_failures(manifest, audit))

    def test_accepts_manual_acceptance_pending_when_live_contract_is_covered(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="partial",
                gaps=("manual_acceptance_pending: user confirmation is required",),
            )

            self.assertEqual([], manifest_smoke_failures(manifest, audit))

    def test_rejects_missing_required_operation_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                required_operations=("pointer", "keyboard"),
                audit_operations=("pointer",),
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "button: live interaction audit missing required operation(s): keyboard",
                failures,
            )

    def test_rejects_manifest_acceptance_check_missing_from_live_audit(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                acceptance_checks=("button_keyboard_activation",),
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "button: live interaction audit missing acceptance check(s): button_keyboard_activation",
                failures,
            )

    def test_rejects_required_operation_missing_from_declared_operation_kinds(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                declared_operations=("pointer",),
                required_operations=("pointer", "timed_tick"),
                audit_operations=("pointer", "timed_tick"),
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "button: required operation is not declared in operation_kinds: timed_tick",
                failures,
            )

    def test_rejects_text_keyboard_copy_without_clipboard_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="text",
                declared_operations=("pointer", "keyboard", "drag", "timed_tick"),
                required_operations=("drag", "keyboard"),
                audit_operations=("drag", "keyboard"),
                keyboard_operation="text_keyboard_copy",
                keyboard_clipboard_text_len=0,
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "text: keyboard copy must produce clipboard payload through copy_selection/clipboard_copy",
                failures,
            )

    def test_rejects_text_keyboard_copy_without_action_event_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="text",
                declared_operations=("pointer", "keyboard", "drag", "timed_tick"),
                required_operations=("drag", "keyboard"),
                audit_operations=("drag", "keyboard"),
                keyboard_operation="text_keyboard_copy",
                keyboard_action="none",
                keyboard_event="none",
                keyboard_clipboard_text_len=12,
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "text: keyboard copy must produce clipboard payload through copy_selection/clipboard_copy",
                failures,
            )

    def test_rejects_text_zero_distance_drag_with_selection_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="text",
                declared_operations=("pointer", "keyboard", "drag", "timed_tick"),
                required_operations=("drag", "keyboard"),
                audit_operations=("drag", "keyboard"),
                keyboard_operation="text_keyboard_copy",
                zero_distance_action="select_text",
                zero_distance_event="text_selection_changed",
                zero_distance_body_pixel_diff=2,
                zero_distance_clipboard_text_len=1,
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "text: zero-distance drag must not select, repaint selection, or copy payload",
                failures,
            )

    def test_rejects_progress_bar_timed_tick_without_progress_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="progress-bar",
                required_operations=("pointer", "timed_tick"),
                audit_operations=("pointer", "timed_tick"),
                timed_tick_action="none",
                timed_tick_event="none",
                timed_tick_state="idle",
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "progress-bar: timed_tick must pass progress_tick/progress_changed with percent=82 and repaint",
                failures,
            )

    def test_rejects_progress_bar_pointer_without_progress_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="progress-bar",
                required_operations=("pointer", "timed_tick"),
                audit_operations=("pointer", "timed_tick"),
                pointer_action="none",
                pointer_event="none",
                pointer_state="idle",
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "progress-bar: pointer preview click must pass progress_change/progress_changed with percent=82 and repaint",
                failures,
            )

    def test_rejects_progress_bar_timed_tick_that_does_not_advance_percent(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="progress-bar",
                required_operations=("pointer", "timed_tick"),
                audit_operations=("pointer", "timed_tick"),
                timed_tick_state="percent=65",
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "progress-bar: timed_tick must pass progress_tick/progress_changed with percent=82 and repaint",
                failures,
            )

    def test_rejects_progress_bar_without_timed_cycle_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="progress-bar",
                required_operations=("pointer", "timed_tick"),
                audit_operations=("pointer", "timed_tick"),
                include_timed_cycle=False,
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "progress-bar: timed_tick must include progress_timed_cycle to percent=0 with repaint",
                failures,
            )

    def test_rejects_progress_bar_without_indeterminate_motion_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                page="progress-bar",
                required_operations=("pointer", "timed_tick"),
                audit_operations=("pointer", "timed_tick"),
                include_indeterminate_motion=False,
            )

            failures = manifest_smoke_failures(manifest, audit)

            self.assertIn(
                "progress-bar: timed_tick must include indeterminate segment motion with repaint",
                failures,
            )

def write_fixture(
    root: Path,
    status: str,
    page: str = "button",
    declared_operations: tuple[str, ...] = ("pointer", "keyboard", "timed_tick"),
    required_operations: tuple[str, ...] = ("pointer",),
    audit_operations: tuple[str, ...] = ("pointer",),
    gaps: tuple[str, ...] = (),
    timed_tick_action: str = "progress_tick",
    timed_tick_event: str = "progress_changed",
    timed_tick_state: str = "percent=82",
    pointer_action: str = "progress_change",
    pointer_event: str = "progress_changed",
    pointer_state: str = "percent=82",
    include_timed_cycle: bool = True,
    include_indeterminate_motion: bool = True,
    keyboard_operation: str = "keyboard",
    keyboard_action: str = "copy_selection",
    keyboard_event: str = "clipboard_copy",
    keyboard_clipboard_text_len: int = 12,
    zero_distance_action: str = "none",
    zero_distance_event: str = "none",
    zero_distance_body_pixel_diff: int = 0,
    zero_distance_clipboard_text_len: int = 0,
    acceptance_checks: tuple[str, ...] = (),
) -> tuple[Path, Path]:
    manifest = root / "manifest.json"
    audit = root / "audit.json"
    declared_operations_json = ",".join(f'"{operation}"' for operation in declared_operations)
    operations_json = ",".join(f'"{operation}"' for operation in required_operations)
    acceptance_checks_json = ",".join(f'"{check}"' for check in acceptance_checks)
    acceptance_checks_field = (
        ',"acceptance_checks":[' + acceptance_checks_json + "]"
        if acceptance_checks
        else ""
    )
    scenarios_json = ",".join(
        audit_scenarios(
            page,
            audit_operations,
            timed_tick_action,
            timed_tick_event,
            timed_tick_state,
            pointer_action,
            pointer_event,
            pointer_state,
            include_timed_cycle,
            include_indeterminate_motion,
            keyboard_operation,
            keyboard_action,
            keyboard_event,
            keyboard_clipboard_text_len,
            zero_distance_action,
            zero_distance_event,
            zero_distance_body_pixel_diff,
            zero_distance_clipboard_text_len,
        )
    )
    gaps_json = ",".join(f'"{gap}"' for gap in gaps)
    manifest.write_text(
        "{"
        '"operation_kinds":['
        + declared_operations_json
        + "],"
        '"defaults_by_engine":{"clickable":{"required_operations":['
        + operations_json
        + "]}},"
        '"ui":[{"page":"'
        + page
        + '","engine":"clickable","audit_status":"'
        + status
        + '","gaps":['
        + gaps_json
        + "]"
        + acceptance_checks_field
        + "}]}",
        encoding="utf-8",
    )
    audit.write_text(
        '{"scenarios":[' + scenarios_json + "]}",
        encoding="utf-8",
    )
    return manifest, audit


def audit_scenarios(
    page: str,
    audit_operations: tuple[str, ...],
    timed_tick_action: str,
    timed_tick_event: str,
    timed_tick_state: str,
    pointer_action: str,
    pointer_event: str,
    pointer_state: str,
    include_timed_cycle: bool,
    include_indeterminate_motion: bool,
    keyboard_operation: str,
    keyboard_action: str,
    keyboard_event: str,
    keyboard_clipboard_text_len: int,
    zero_distance_action: str,
    zero_distance_event: str,
    zero_distance_body_pixel_diff: int,
    zero_distance_clipboard_text_len: int,
) -> list[str]:
    scenarios: list[str] = []
    for operation in audit_operations:
        scenarios.append(
            "{"
            '"page":"'
            + page
            + '",'
            '"operation_kind":"'
            + operation
            + '",'
            '"operation":"'
            + operation_name(operation, keyboard_operation)
            + '",'
            '"passed":true,'
            '"action":"'
            + action_name(operation, timed_tick_action, pointer_action, keyboard_action)
            + '",'
            '"event":"'
            + event_name(operation, timed_tick_event, pointer_event, keyboard_event)
            + '",'
            '"state":"'
            + state_name(operation, timed_tick_state, pointer_state)
            + '",'
            '"body_pixel_diff":1,'
            '"clipboard_text_len":'
            + str(keyboard_clipboard_text_len if operation == "keyboard" else 0)
            + "}"
        )
    if page == "progress-bar" and include_timed_cycle:
        scenarios.append(
            "{"
            '"page":"progress-bar",'
            '"operation_kind":"timed_tick",'
            '"operation":"progress_timed_cycle",'
            '"passed":true,'
            '"action":"progress_tick",'
            '"event":"progress_changed",'
            '"state":"percent=0",'
            '"body_pixel_diff":1,'
            '"clipboard_text_len":0'
            "}"
        )
    if page == "progress-bar" and include_indeterminate_motion:
        scenarios.append(
            "{"
            '"page":"progress-bar",'
            '"operation_kind":"timed_tick",'
            '"operation":"progress_indeterminate_segment_motion",'
            '"passed":true,'
            '"action":"progress_tick",'
            '"event":"progress_changed",'
            '"state":"percent=82",'
            '"body_pixel_diff":1,'
            '"clipboard_text_len":0'
            "}"
        )
    if page == "text":
        scenarios.append(
            "{"
            '"page":"text",'
            '"operation_kind":"drag",'
            '"operation":"text_zero_distance_drag_no_selection",'
            '"passed":true,'
            '"action":"'
            + zero_distance_action
            + '",'
            '"event":"'
            + zero_distance_event
            + '",'
            '"state":"idle",'
            '"body_pixel_diff":'
            + str(zero_distance_body_pixel_diff)
            + ','
            '"clipboard_text_len":'
            + str(zero_distance_clipboard_text_len)
            + "}"
        )
    return scenarios


def operation_name(operation: str, keyboard_operation: str) -> str:
    if operation == "timed_tick":
        return "progress_timed_tick"
    if operation == "pointer":
        return "preview_click"
    if operation == "keyboard":
        return keyboard_operation
    if operation == "drag":
        return "text_drag_selection"
    return operation


def action_name(
    operation: str,
    timed_tick_action: str,
    pointer_action: str,
    keyboard_action: str,
) -> str:
    if operation == "timed_tick":
        return timed_tick_action
    if operation == "pointer":
        return pointer_action
    if operation == "keyboard":
        return keyboard_action
    return operation


def event_name(
    operation: str,
    timed_tick_event: str,
    pointer_event: str,
    keyboard_event: str,
) -> str:
    if operation == "timed_tick":
        return timed_tick_event
    if operation == "pointer":
        return pointer_event
    if operation == "keyboard":
        return keyboard_event
    return operation


def state_name(operation: str, timed_tick_state: str, pointer_state: str) -> str:
    if operation == "timed_tick":
        return timed_tick_state
    if operation == "pointer":
        return pointer_state
    return operation


if __name__ == "__main__":
    unittest.main()
