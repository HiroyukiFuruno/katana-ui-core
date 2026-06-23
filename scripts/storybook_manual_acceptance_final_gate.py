#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from storybook_manual_acceptance_queue import MANIFEST_PATH, MANUAL_GATE, manual_acceptance_queue
from storybook_manual_acceptance_queue import OPEN_WINDOW_PREFIX
from storybook_manual_acceptance_approve import APPROVAL_LOG_PATH
from storybook_manual_acceptance_metadata import validate_approval_metadata
from storybook_manual_acceptance_smoke import EVIDENCE_PATH

LEDGER_PATH = Path("docs/storybook-77ui-deep-audit-ledger.md")
PRIORITY_ORDER_PATH = Path(
    "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md"
)
REQUIRED_MANUAL_ACCEPTANCE_PAGES = (
    "text",
    "checkbox",
    "progress-bar",
    "tooltip",
    "modal",
    "tree-view",
)
REQUIRED_MANUAL_ACCEPTANCE_ORDER = {
    "text": 10,
    "checkbox": 20,
    "progress-bar": 30,
    "tooltip": 40,
    "modal": 50,
    "tree-view": 60,
}
REQUIRED_MANUAL_ACCEPTANCE_DEPENDENCIES = {
    "text": [],
    "checkbox": ["text"],
    "progress-bar": ["text"],
    "tooltip": ["text", "checkbox"],
    "modal": ["text", "checkbox", "tooltip"],
    "tree-view": ["text", "checkbox"],
}
REQUIRED_MANUAL_ACCEPTANCE_LAYERS = {
    "text": "foundation-text-selection",
    "checkbox": "binary-choice-state-display",
    "progress-bar": "feedback-motion-meter",
    "tooltip": "overlay-anchor-hover-focus",
    "modal": "overlay-modal-focus-dismiss",
    "tree-view": "selection-tree-scroll-context",
}
REQUIRED_MANUAL_ACCEPTANCE_OPERATIONS = {
    "text": ["pointer", "drag", "keyboard"],
    "checkbox": ["pointer", "keyboard", "focus", "hover"],
    "progress-bar": ["pointer", "timed_tick"],
    "tooltip": ["pointer", "hover", "focus"],
    "modal": ["pointer", "keyboard", "focus"],
    "tree-view": ["pointer", "keyboard", "focus", "hover", "scroll", "context_menu"],
}
REQUIRED_MANUAL_ACCEPTANCE_FRAMES = {
    "text": 1,
    "checkbox": 1,
    "progress-bar": 48,
    "tooltip": 1,
    "modal": 1,
    "tree-view": 1,
}
REQUIRED_MANUAL_ACCEPTANCE_CHECKS = {
    "text": [
        "text_drag_selection",
        "text_keyboard_copy",
        "text_keyboard_paste",
        "text_zero_distance_drag_no_selection",
    ],
    "checkbox": [
        "row_click",
        "checkbox_pointer_checks_both_rows",
        "checkbox_keyboard_toggle",
        "checkbox_keyboard_toggle_off",
        "checkbox_keyboard_focused_secondary_row",
        "checkbox_control_toggle_reset",
        "checkbox_focus",
        "preview_hover",
        "checkbox_hover_no_click_event",
        "checkbox_hover_secondary_row",
        "disabled_focus_keyboard_block",
        "checkbox_disabled_pointer_block",
        "checkbox_no_runtime_overlay_over_controls",
        "checkbox_controls_bottom_padding",
        "checkbox_disabled_snapshot_click_block",
        "checkbox_disabled_controls_are_muted",
        "checkbox_disabled_hover_is_muted",
        "checkbox_checked_preset_state_consistency",
        "checkbox_disabled_preset_state_consistency",
        "checkbox_focus_preset_state_consistency",
        "checkbox_checked_state_read_preserves_checked_state_metadata",
        "checkbox_disabled_state_read_control_is_blocked",
        "checkbox_focus_state_read_preserves_focus_state_metadata",
        "checkbox_initial_snapshot_state_consistency",
        "checkbox_focus_labels_visible",
        "checkbox_focus_single_active_row",
        "checkbox_inspector_options_are_labeled",
        "checkbox_modern_spacing",
        "checkbox_snapshot_state_consistency",
    ],
    "progress-bar": [
        "progress_preview_click",
        "progress_timed_tick",
        "progress_timed_cycle",
        "progress_indeterminate_segment_motion",
    ],
    "tooltip": [
        "preview_click",
        "tooltip_anchor_hover_open",
        "tooltip_hover_idempotent",
        "tooltip_hover_leave_close",
        "tooltip_idle_bubble_hidden_until_hover",
        "tooltip_focus_open",
        "tooltip_window_hover_clear_close",
        "tooltip_hover_bubble_geometry",
    ],
    "modal": [
        "preview_click",
        "modal_keyboard_escape",
        "modal_escape_removes_surface",
        "modal_escape_after_close_idempotent",
        "modal_focus_trap",
    ],
    "tree-view": [
        "preview_click",
        "tree_keyboard_select",
        "tree_focus_item",
        "tree_hover_item",
        "tree_view_context_menu",
        "tree_scroll_retained",
    ],
}
REQUIRED_MANUAL_ACCEPTANCE_EVIDENCE_CONTRACT = {
    "text": [
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
            "check": "text_keyboard_paste",
            "operation_kind": "keyboard",
            "state": "idle",
            "action": "none",
            "event": "none",
        },
        {
            "check": "text_zero_distance_drag_no_selection",
            "operation_kind": "drag",
            "state": "idle",
            "action": "none",
            "event": "none",
        },
    ],
}
REQUIRED_MANUAL_ACCEPTANCE_OBSERVATIONS = {
    "text": [
        "Drag creates a visible text selection highlight",
        "Copy exports selected text",
        "Zero-distance drag does not create a selection action, highlight, or copy payload",
    ],
    "checkbox": [
        "row click toggles the checked mark and state together",
        "row 0 and row 1 pointer clicks can leave both checkbox rows checked at the same time",
        "keyboard activation toggles the checked mark and state together",
        "second keyboard activation toggles the checked mark and state back off together",
        "keyboard activation mutates the focused secondary row without changing the primary row",
        "toggle and reset controls mutate checked state and rendered mark through the same public action path",
        "focus renders a visible focus state",
        "hover renders without repeatedly firing click events",
        "hover does not increment action_count, emit checkbox_toggle, or mutate checked state",
        "hover feedback follows the actual checkbox row under the pointer",
        "disabled preset blocks focus and keyboard checked mutation while preserving the mark",
        "disabled preset blocks pointer checked mutation while preserving the mark",
        "Storybook runtime overlay does not draw clicked labels over core checkbox controls",
        "checkbox control row keeps bottom padding inside the component frame",
        "disabled clicked snapshot path does not bypass window interaction disabled blocking",
        "disabled preset mutes checkbox control button labels instead of presenting enabled controls",
        "disabled preset does not show enabled hover feedback",
        "checked preset reports current checked state in preview and Inspector instead of idle/false state",
        "disabled preset reports current disabled state in preview and Inspector instead of idle state",
        "disabled focus and keyboard block preserve disabled=true state metadata",
        "focus preset reports current focus state in preview and Inspector instead of idle/false state",
        "state read preserves checked=true, disabled=true, and focused=true current public state metadata instead of replacing it with before/after history labels",
        "initial snapshot keeps idle state visible and does not render a no-op before/after transition as operation history",
        "focus preset keeps checkbox row labels visibly rendered",
        "focus preset renders a focus ring on the active row only",
        "Inspector settings rows label binary-choice mutations as option values instead of current state values",
        "checkbox mark, row, and status spacing meet the modern binary-choice layout contract",
        "checked glyph uses the core accent-foreground theme token through VisualPalette instead of a Storybook-only fixed literal",
        "clicked snapshot keeps preview status and Inspector state/action/event consistent",
    ],
    "progress-bar": [
        "preview click advances meter from 65% to 82%",
        "meter advances from 65% to 82%",
        "meter cycles back to 0% after max",
        "indeterminate segment visibly moves on timed tick",
    ],
    "tooltip": [
        "preview trigger opens the tooltip surface",
        "hover opens the tooltip surface without repeated event spam",
        "hover leave closes the tooltip surface without a click-like replacement event",
        "focus opens the tooltip surface through the core focus path",
        "window-level hover clear closes an open tooltip when the pointer leaves the window",
        "hover bubble remains inside the preview component and visually covers the anchor center",
    ],
    "modal": [
        "preview action changes the modal open/closed surface",
        "Escape closes the modal through the core modal action",
        "closed modal state removes backdrop/dialog/native/close surfaces from the preview",
        "Escape after a closed modal is ignored without emitting another close event",
        "focus operation enters the modal focus trap",
    ],
    "tree-view": [
        "row click toggles or selects a tree item",
        "keyboard selection updates the selected item",
        "focus targets the tree row",
        "hover targets the tree row without repeated event spam",
        "context menu opens on a tree row",
        "clicking after scroll keeps the visible tree offset instead of jumping to the top",
    ],
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--ledger", type=Path, default=LEDGER_PATH)
    parser.add_argument("--approval-log", type=Path, default=APPROVAL_LOG_PATH)
    parser.add_argument("--evidence", type=Path, default=EVIDENCE_PATH)
    parser.add_argument("--priority-order", type=Path, default=PRIORITY_ORDER_PATH)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    failures = manual_acceptance_final_gate_failures(
        args.manifest,
        args.ledger,
        args.approval_log,
        args.evidence,
        args.priority_order,
    )
    if failures:
        print("storybook manual acceptance final gate failed")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("storybook manual acceptance final gate passed")
    return 0


def manual_acceptance_final_gate_failures(
    manifest_path: Path,
    ledger_path: Path | None = None,
    approval_log_path: Path | None = None,
    evidence_path: Path | None = None,
    priority_order_path: Path | None = None,
) -> list[str]:
    manifest_failures = manifest_shape_failures(manifest_path)
    if manifest_failures:
        return manifest_failures

    failures: list[str] = []
    try:
        queue = manual_acceptance_queue(manifest_path)
    except ValueError as error:
        return [f"manifest manual acceptance queue invalid: {error}"]
    pending_pages: list[str] = []
    for entry in queue:
        page = entry.get("page")
        if isinstance(page, str):
            pending_pages.append(page)
            failures.append(f"{page}: manual acceptance is still pending")
        else:
            failures.append("unknown page: manual acceptance is still pending")
    if priority_order_path is not None and pending_pages:
        failures.extend(priority_order_pending_failures(priority_order_path, pending_pages))
    if ledger_path is not None:
        failures.extend(ledger_pending_failures(ledger_path))
    if (
        not failures
        and not pending_pages
        and approval_log_path == APPROVAL_LOG_PATH
        and evidence_path == EVIDENCE_PATH
    ):
        return failures
    if not failures and approval_log_path is not None and evidence_path is not None:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        if ledger_path is not None:
            failures.extend(ledger_verified_failures(ledger_path))
        failures.extend(manual_acceptance_target_sync_failures(manifest))
        failures.extend(
            final_approval_evidence_failures(
                approval_log_path,
                evidence_path,
                acceptance_checks_by_page(manifest),
                acceptance_observations_by_page(manifest),
                minimum_observation_frames_by_page(manifest),
                string_field_by_page(manifest, "command"),
                string_field_by_page(manifest, "smoke_command"),
                evidence_contract_by_page(manifest),
            )
        )
    return failures


def manual_acceptance_target_sync_failures(manifest: dict[str, Any]) -> list[str]:
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return []
    required_pages = set(REQUIRED_MANUAL_ACCEPTANCE_PAGES)
    entries_by_page = {}
    target_entries_by_page = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        if not isinstance(page, str):
            continue
        entries_by_page[page] = entry
        if page in required_pages or is_manual_acceptance_target(entry):
            target_entries_by_page[page] = entry
    actual = set(target_entries_by_page)
    expected = required_pages
    failures: list[str] = []
    for page in sorted(expected - actual):
        failures.append(f"{page}: required manual acceptance target is missing from manifest")
    for page in sorted(actual - expected):
        failures.append(f"{page}: manifest has unexpected manual acceptance target")
    for page in REQUIRED_MANUAL_ACCEPTANCE_PAGES:
        entry = entries_by_page.get(page)
        if not isinstance(entry, dict):
            continue
        if entry.get("audit_status") != "verified":
            failures.append(
                f"{page}: manifest audit_status must be verified after manual acceptance"
            )
        expected_order = REQUIRED_MANUAL_ACCEPTANCE_ORDER[page]
        if entry.get("manual_acceptance_order") != expected_order:
            failures.append(
                f"{page}: manifest manual_acceptance_order must be {expected_order}"
            )
        expected_depends_on = REQUIRED_MANUAL_ACCEPTANCE_DEPENDENCIES[page]
        if string_values(entry.get("depends_on", [])) != expected_depends_on:
            failures.append(
                f"{page}: manifest depends_on must match manual acceptance dependency order: {', '.join(expected_depends_on)}"
            )
        expected_layer = REQUIRED_MANUAL_ACCEPTANCE_LAYERS[page]
        if entry.get("dependency_layer") != expected_layer:
            failures.append(f"{page}: manifest dependency_layer must be {expected_layer}")
        expected_operations = REQUIRED_MANUAL_ACCEPTANCE_OPERATIONS[page]
        if string_values(entry.get("required_operations", [])) != expected_operations:
            failures.append(
                f"{page}: manifest required_operations must match manual acceptance contract: {', '.join(expected_operations)}"
            )
        expected_frames = REQUIRED_MANUAL_ACCEPTANCE_FRAMES[page]
        if entry.get("minimum_observation_frames") != expected_frames:
            failures.append(
                f"{page}: manifest minimum_observation_frames must be {expected_frames}"
            )
        expected_command = f"{OPEN_WINDOW_PREFIX} {page}"
        if entry.get("command") != expected_command:
            failures.append(
                f"{page}: manifest command must match manual acceptance contract"
            )
        expected_smoke_command = f"{OPEN_WINDOW_PREFIX} {expected_frames} {page}"
        if entry.get("smoke_command") != expected_smoke_command:
            failures.append(
                f"{page}: manifest smoke_command must match manual acceptance contract"
            )
        expected_checks = REQUIRED_MANUAL_ACCEPTANCE_CHECKS.get(page)
        if expected_checks is not None and string_values(entry.get("acceptance_checks", [])) != expected_checks:
            failures.append(
                f"{page}: manifest acceptance_checks must match manual acceptance contract: {', '.join(expected_checks)}"
            )
        expected_observations = REQUIRED_MANUAL_ACCEPTANCE_OBSERVATIONS.get(page)
        if expected_observations is not None and string_values(entry.get("acceptance_observations", [])) != expected_observations:
            failures.append(
                f"{page}: manifest acceptance_observations must match manual acceptance contract: {'; '.join(expected_observations)}"
            )
        expected_evidence_contract = REQUIRED_MANUAL_ACCEPTANCE_EVIDENCE_CONTRACT.get(page)
        if expected_evidence_contract is not None and entry.get("acceptance_evidence_contract", []) != expected_evidence_contract:
            failures.append(
                f"{page}: manifest acceptance_evidence_contract must match manual acceptance contract"
            )
        if not string_values(entry.get("acceptance_checks", [])):
            failures.append(f"{page}: manifest acceptance_checks must not be empty")
        if not string_values(entry.get("acceptance_observations", [])):
            failures.append(f"{page}: manifest acceptance_observations must not be empty")
    return failures


def is_manual_acceptance_target(entry: dict[str, Any]) -> bool:
    return bool(
        string_values(entry.get("acceptance_checks", []))
        or string_values(entry.get("acceptance_observations", []))
    )


def final_approval_evidence_failures(
    approval_log_path: Path,
    evidence_path: Path,
    expected_checks_by_page: dict[str, list[str]] | None = None,
    expected_observations_by_page: dict[str, list[str]] | None = None,
    expected_frames_by_page: dict[str, int] | None = None,
    expected_commands_by_page: dict[str, str] | None = None,
    expected_smoke_commands_by_page: dict[str, str] | None = None,
    expected_evidence_contract_by_page: dict[str, list[dict[str, str]]] | None = None,
) -> list[str]:
    approvals = load_json_array(approval_log_path, "approval log")
    evidence = load_json_array(evidence_path, "manual acceptance evidence")
    failures: list[str] = []
    if isinstance(approvals, list):
        failures.extend(required_page_order_failures(approvals, "final approval"))
        approvals_by_page, approval_page_failures = unique_entries_by_required_page(
            approvals,
            "final approval",
        )
        failures.extend(approval_page_failures)
    else:
        failures.append(approvals)
        approvals_by_page = {}
    if isinstance(evidence, list):
        failures.extend(required_page_order_failures(evidence, "final evidence"))
        evidence_by_page, evidence_page_failures = unique_entries_by_required_page(
            evidence,
            "final evidence",
        )
        failures.extend(evidence_page_failures)
    else:
        failures.append(evidence)
        evidence_by_page = {}
    for page in REQUIRED_MANUAL_ACCEPTANCE_PAGES:
        approval = approvals_by_page.get(page)
        evidence_entry = evidence_by_page.get(page)
        if not isinstance(approval, dict):
            failures.append(f"{page}: final approval entry is missing")
            continue
        if approval.get("approved") is not True:
            failures.append(f"{page}: final approval must set approved=true")
        approved_by = approval.get("approved_by")
        approved_at = approval.get("approved_at")
        try:
            validate_approval_metadata(
                approved_by if isinstance(approved_by, str) else "",
                approved_at if isinstance(approved_at, str) else "",
            )
        except ValueError as error:
            failures.append(f"{page}: final approval {error}")
        if not isinstance(evidence_entry, dict):
            failures.append(f"{page}: final evidence entry is missing")
            continue
        failures.extend(
            final_approval_matches_evidence_failures(
                page,
                approval,
                evidence_entry,
                expected_checks_by_page.get(page, [])
                if expected_checks_by_page is not None
                else string_values(approval.get("acceptance_checks", [])),
                expected_observations_by_page.get(page, [])
                if expected_observations_by_page is not None
                else string_values(approval.get("acceptance_observations", [])),
                expected_frames_by_page.get(page)
                if expected_frames_by_page is not None
                else None,
                expected_commands_by_page.get(page)
                if expected_commands_by_page is not None
                else None,
                expected_smoke_commands_by_page.get(page)
                if expected_smoke_commands_by_page is not None
                else None,
                expected_evidence_contract_by_page.get(page)
                if expected_evidence_contract_by_page is not None
                else None,
            )
        )
    return failures


def required_page_order_failures(entries: list[Any], label: str) -> list[str]:
    pages = [
        entry.get("page")
        for entry in entries
        if isinstance(entry, dict) and isinstance(entry.get("page"), str)
    ]
    required = list(REQUIRED_MANUAL_ACCEPTANCE_PAGES)
    if pages == required:
        return []
    return [
        f"{label} order must match manual acceptance dependency order: {', '.join(required)}"
    ]


def acceptance_checks_by_page(manifest: dict[str, Any]) -> dict[str, list[str]]:
    return manifest_string_list_by_page(manifest, "acceptance_checks")


def acceptance_observations_by_page(manifest: dict[str, Any]) -> dict[str, list[str]]:
    return manifest_string_list_by_page(manifest, "acceptance_observations")


def evidence_contract_by_page(manifest: dict[str, Any]) -> dict[str, list[dict[str, str]]]:
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return {}
    values_by_page: dict[str, list[dict[str, str]]] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        value = entry.get("acceptance_evidence_contract", [])
        if isinstance(page, str) and isinstance(value, list):
            values_by_page[page] = [
                item
                for item in value
                if isinstance(item, dict)
                and all(isinstance(key, str) and isinstance(item_value, str) for key, item_value in item.items())
            ]
    return values_by_page


def minimum_observation_frames_by_page(manifest: dict[str, Any]) -> dict[str, int]:
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return {}
    values_by_page: dict[str, int] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        frames = entry.get("minimum_observation_frames")
        if isinstance(page, str) and isinstance(frames, int):
            values_by_page[page] = frames
    return values_by_page


def string_field_by_page(manifest: dict[str, Any], key: str) -> dict[str, str]:
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return {}
    values_by_page: dict[str, str] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        value = entry.get(key)
        if isinstance(page, str) and isinstance(value, str):
            values_by_page[page] = value
    return values_by_page


def manifest_string_list_by_page(manifest: dict[str, Any], key: str) -> dict[str, list[str]]:
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return {}
    values_by_page: dict[str, list[str]] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        if isinstance(page, str):
            values_by_page[page] = string_values(entry.get(key, []))
    return values_by_page


def unique_entries_by_required_page(
    entries: list[Any],
    label: str,
) -> tuple[dict[str, dict[str, Any]], list[str]]:
    expected = set(REQUIRED_MANUAL_ACCEPTANCE_PAGES)
    by_page: dict[str, dict[str, Any]] = {}
    failures: list[str] = []
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            failures.append(f"{label}[{index}]: entry must be an object")
            continue
        page = entry.get("page")
        if not isinstance(page, str) or not page:
            failures.append(f"{label}[{index}]: page must be a non-empty string")
            continue
        if page not in expected:
            failures.append(f"{page}: {label} has unexpected page")
            continue
        if page in by_page:
            failures.append(f"{page}: {label} entry is duplicated")
            continue
        by_page[page] = entry
    return by_page, failures


def final_approval_matches_evidence_failures(
    page: str,
    approval: dict[str, Any],
    evidence_entry: dict[str, Any],
    expected_checks: list[str],
    expected_observations: list[str],
    expected_frames: int | None = None,
    expected_command: str | None = None,
    expected_smoke_command: str | None = None,
    expected_evidence_contract: list[dict[str, str]] | None = None,
) -> list[str]:
    failures: list[str] = []
    for key in ("command", "smoke_command", "minimum_observation_frames"):
        if approval.get(key) != evidence_entry.get(key):
            failures.append(f"{page}: final approval {key} does not match evidence")
    if expected_command is not None:
        if approval.get("command") != expected_command:
            failures.append(f"{page}: final approval command must match manifest")
        if evidence_entry.get("command") != expected_command:
            failures.append(f"{page}: final evidence command must match manifest")
    if expected_smoke_command is not None:
        if approval.get("smoke_command") != expected_smoke_command:
            failures.append(f"{page}: final approval smoke_command must match manifest")
        if evidence_entry.get("smoke_command") != expected_smoke_command:
            failures.append(f"{page}: final evidence smoke_command must match manifest")
    if expected_frames is not None:
        if approval.get("minimum_observation_frames") != expected_frames:
            failures.append(
                f"{page}: final approval minimum_observation_frames must match manifest"
            )
        if evidence_entry.get("minimum_observation_frames") != expected_frames:
            failures.append(
                f"{page}: final evidence minimum_observation_frames must match manifest"
            )
    approval_observations = string_values(approval.get("acceptance_observations", []))
    evidence_observations = string_values(evidence_entry.get("acceptance_observations", []))
    if approval_observations != expected_observations:
        failures.append(f"{page}: final approval observations do not match manifest")
    if evidence_observations != expected_observations:
        failures.append(f"{page}: final evidence observations do not match manifest")
    approval_checks = string_values(approval.get("acceptance_checks", []))
    if approval_checks != expected_checks:
        failures.append(f"{page}: final approval acceptance_checks do not match manifest")
    if expected_evidence_contract is not None and approval.get("acceptance_evidence_contract", []) != expected_evidence_contract:
        failures.append(f"{page}: final approval acceptance_evidence_contract does not match manifest")
    if approval.get("manual_gate") != MANUAL_GATE:
        failures.append(
            f"{page}: final approval manual_gate must match manual acceptance contract"
        )
    evidence_items = evidence_entry.get("audit_evidence", [])
    if not isinstance(evidence_items, list):
        failures.append(f"{page}: final evidence audit_evidence must be an array")
        return failures
    expected_check_set = set(expected_checks)
    evidence_by_check: dict[str, dict[str, Any]] = {}
    for index, item in enumerate(evidence_items):
        if not isinstance(item, dict):
            failures.append(f"{page}: final evidence audit_evidence[{index}] must be an object")
            continue
        check = item.get("check")
        if not isinstance(check, str) or not check:
            failures.append(
                f"{page}: final evidence audit_evidence[{index}] check must be a non-empty string"
            )
            continue
        if check not in expected_check_set:
            failures.append(f"{page}: final evidence has unexpected check {check}")
            continue
        if check in evidence_by_check:
            failures.append(f"{page}: final evidence check {check} is duplicated")
            continue
        evidence_by_check[check] = item
    for check in expected_checks:
        item = evidence_by_check.get(check)
        if not isinstance(item, dict):
            failures.append(f"{page}: final evidence missing check {check}")
        elif item.get("passed") is not True:
            failures.append(f"{page}: final evidence check {check} did not pass")
        else:
            failures.extend(final_evidence_item_contract_failures(page, check, item))
    return failures


def final_evidence_item_contract_failures(
    page: str,
    check: str,
    item: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    expected_kind = {
        "text_drag_selection": "drag",
        "text_keyboard_copy": "keyboard",
        "text_keyboard_paste": "keyboard",
        "text_zero_distance_drag_no_selection": "drag",
    }.get(check)
    if expected_kind is not None and item.get("operation_kind") != expected_kind:
        failures.append(
            f"{page}: final evidence {check} must include operation_kind {expected_kind}"
        )
    if check == "text_drag_selection" and int_value(item.get("body_pixel_diff")) <= 0:
        failures.append(
            f"{page}: final evidence {check} must include positive body_pixel_diff"
        )
    if check == "text_drag_selection":
        if item.get("state") != "selection=active":
            failures.append(
                f"{page}: final evidence {check} must include state selection=active"
            )
        if item.get("action") != "select_text":
            failures.append(
                f"{page}: final evidence {check} must include action select_text"
            )
        if item.get("event") != "text_selection_changed":
            failures.append(
                f"{page}: final evidence {check} must include event text_selection_changed"
            )
    if check == "text_keyboard_copy" and int_value(item.get("clipboard_text_len")) <= 0:
        failures.append(
            f"{page}: final evidence {check} must include positive clipboard_text_len"
        )
    if check == "text_keyboard_copy":
        if item.get("state") != "clipboard=selected_text":
            failures.append(
                f"{page}: final evidence {check} must include state clipboard=selected_text"
            )
        if item.get("action") != "copy_selection":
            failures.append(
                f"{page}: final evidence {check} must include action copy_selection"
            )
        if item.get("event") != "clipboard_copy":
            failures.append(
                f"{page}: final evidence {check} must include event clipboard_copy"
            )
    if check == "text_keyboard_paste":
        if item.get("state") != "idle":
            failures.append(
                f"{page}: final evidence {check} must include state idle"
            )
        if item.get("action") != "none":
            failures.append(
                f"{page}: final evidence {check} must include action none"
            )
        if item.get("event") != "none":
            failures.append(
                f"{page}: final evidence {check} must include event none"
            )
    if check == "text_zero_distance_drag_no_selection":
        if item.get("state") != "idle":
            failures.append(
                f"{page}: final evidence {check} must include state idle"
            )
        if int_value(item.get("body_pixel_diff")) != 0:
            failures.append(
                f"{page}: final evidence {check} must include zero body_pixel_diff"
            )
        if int_value(item.get("clipboard_text_len")) != 0:
            failures.append(
                f"{page}: final evidence {check} must include zero clipboard_text_len"
            )
        if item.get("action") != "none":
            failures.append(
                f"{page}: final evidence {check} must include action none"
            )
        if item.get("event") != "none":
            failures.append(
                f"{page}: final evidence {check} must include event none"
            )
    if check in ("row_click", "checkbox_pointer_checks_both_rows"):
        failures.extend(checkbox_pointer_final_evidence_failures(page, check, item))
    if check.startswith("progress_"):
        failures.extend(progress_final_evidence_failures(page, check, item))
    return failures


def checkbox_pointer_final_evidence_failures(
    page: str,
    check: str,
    item: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    if item.get("operation_kind") != "pointer":
        failures.append(
            f"{page}: final evidence {check} must include operation_kind pointer"
        )
    if item.get("action") != "checkbox_toggle":
        failures.append(
            f"{page}: final evidence {check} must include action checkbox_toggle"
        )
    if item.get("event") != "checked_changed":
        failures.append(
            f"{page}: final evidence {check} must include event checked_changed"
        )
    if int_value(item.get("body_pixel_diff")) <= 0:
        failures.append(
            f"{page}: final evidence {check} must include positive body_pixel_diff"
        )
    return failures


def progress_final_evidence_failures(
    page: str,
    check: str,
    item: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    expected_state = {
        "progress_preview_click": "percent=82",
        "progress_timed_tick": "percent=82",
        "progress_timed_cycle": "percent=0",
        "progress_indeterminate_segment_motion": "percent=82",
    }.get(check)
    expected_action = (
        "progress_change" if check == "progress_preview_click" else "progress_tick"
    )
    expected_kind = "pointer" if check == "progress_preview_click" else "timed_tick"
    if item.get("operation_kind") != expected_kind:
        failures.append(
            f"{page}: final evidence {check} must include operation_kind {expected_kind}"
        )
    if item.get("action") != expected_action:
        failures.append(
            f"{page}: final evidence {check} must include action {expected_action}"
        )
    if item.get("event") != "progress_changed":
        failures.append(
            f"{page}: final evidence {check} must include event progress_changed"
        )
    if expected_state is not None and item.get("state") != expected_state:
        failures.append(
            f"{page}: final evidence {check} must include state {expected_state}"
        )
    if int_value(item.get("body_pixel_diff")) <= 0:
        failures.append(
            f"{page}: final evidence {check} must include positive body_pixel_diff"
        )
    return failures


def load_json_array(path: Path, label: str) -> list[Any] | str:
    if not path.exists():
        return f"{path}: {label} is missing"
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, list):
        return f"{path}: {label} must be an array"
    return payload


def string_values(values: Any) -> list[str]:
    if not isinstance(values, list):
        return []
    return [value for value in values if isinstance(value, str)]


def int_value(value: Any) -> int:
    return value if isinstance(value, int) else 0


def ledger_pending_failures(ledger_path: Path) -> list[str]:
    if not ledger_path.exists():
        return [f"{ledger_path}: ledger is missing"]
    failures: list[str] = []
    for line_number, line in enumerate(ledger_path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line.startswith("|") or "manual_acceptance_pending" not in line:
            continue
        cells = split_markdown_row(line)
        page = cells[1].strip() if len(cells) > 1 else "<unknown>"
        failures.append(f"{page}: ledger manual acceptance is still pending at line {line_number}")
    return failures


def priority_order_pending_failures(
    priority_order_path: Path,
    pending_pages: list[str],
) -> list[str]:
    if not priority_order_path.exists():
        return [f"{priority_order_path}: priority order is missing"]
    rows_by_page: dict[str, list[str]] = {}
    for line in priority_order_path.read_text(encoding="utf-8").splitlines():
        if not line.startswith("| SB-"):
            continue
        cells = [cell.strip() for cell in split_markdown_row(line)]
        if len(cells) < 6:
            continue
        page = cells[1].strip("`")
        rows_by_page[page] = cells
    failures: list[str] = []
    for page in pending_pages:
        cells = rows_by_page.get(page)
        if cells is None:
            failures.append(f"{page}: priority order row is missing")
            continue
        dod_status = cells[4].strip()
        next_action = cells[5].strip()
        if dod_status == "完了":
            failures.append(
                f"{page}: priority order DoD status must not be 完了 while manual acceptance is pending"
            )
        if next_action == "完了":
            failures.append(
                f"{page}: priority order next action must not be 完了 while manual acceptance is pending"
            )
    return failures


def ledger_verified_failures(ledger_path: Path) -> list[str]:
    if not ledger_path.exists():
        return [f"{ledger_path}: ledger is missing"]
    required_pages = set(REQUIRED_MANUAL_ACCEPTANCE_PAGES)
    main_seen: set[str] = set()
    failures: list[str] = []
    for line_number, line in enumerate(ledger_path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line.startswith("|"):
            continue
        cells = split_markdown_row(line)
        if len(cells) < 6:
            continue
        ui = cells[1].strip()
        page = ledger_required_page_for_ui(ui, required_pages)
        if page is None:
            continue
        if ui == page:
            main_seen.add(page)
        status = cells[-1].strip()
        if status != "実証済み":
            failures.append(
                f"{ui}: ledger manual acceptance must be 実証済み at line {line_number}"
            )
    for page in sorted(required_pages - main_seen):
        failures.append(f"{page}: ledger manual acceptance row is missing")
    return failures


def ledger_required_page_for_ui(ui: str, required_pages: set[str]) -> str | None:
    for page in required_pages:
        if ui == page or ui.startswith(f"{page} follow-up"):
            return page
    return None


def split_markdown_row(line: str) -> list[str]:
    body = line.strip()
    if body.startswith("|"):
        body = body[1:]
    if body.endswith("|"):
        body = body[:-1]
    return body.split("|")


def manifest_shape_failures(manifest_path: Path) -> list[str]:
    if not manifest_path.exists():
        return [f"{manifest_path}: manifest is missing"]
    payload = json.loads(manifest_path.read_text(encoding="utf-8"))
    entries: Any = payload.get("ui")
    if not isinstance(entries, list):
        return [f"{manifest_path}: ui must be a list"]
    failures: list[str] = []
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            failures.append(f"{manifest_path}: ui[{index}] must be an object")
            continue
        page = entry.get("page")
        if not isinstance(page, str) or not page:
            failures.append(f"{manifest_path}: ui[{index}].page must be a non-empty string")
        gaps = entry.get("gaps", [])
        if not isinstance(gaps, list):
            label = page if isinstance(page, str) and page else f"ui[{index}]"
            failures.append(f"{manifest_path}: {label}.gaps must be a list")
    return failures


if __name__ == "__main__":
    raise SystemExit(main())
