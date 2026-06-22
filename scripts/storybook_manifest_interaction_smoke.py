#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

MANIFEST_PATH = Path("docs/storybook-77ui-interaction-manifest.json")
AUDIT_PATH = Path("target/storybook-live-interaction-audit.json")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--audit", type=Path, default=AUDIT_PATH)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    failures = manifest_smoke_failures(
        args.root / args.manifest,
        args.root / args.audit,
    )
    if failures:
        print("storybook manifest interaction smoke failed")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("storybook manifest interaction smoke passed")
    return 0


def manifest_smoke_failures(manifest_path: Path, audit_path: Path) -> list[str]:
    manifest = load_json(manifest_path)
    audit = load_json(audit_path)
    failures: list[str] = []
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return ["manifest `ui` must be an array"]
    scenarios = scenario_pages(audit)
    for entry in entries:
        if not isinstance(entry, dict):
            failures.append("manifest ui entry must be an object")
            continue
        failures.extend(entry_failures(entry, manifest, scenarios))
    return failures


def entry_failures(
    entry: dict[str, Any],
    manifest: dict[str, Any],
    scenarios: dict[str, list[dict[str, Any]]],
) -> list[str]:
    page = entry.get("page", "<missing>")
    status = entry.get("audit_status")
    failures: list[str] = []
    gaps = entry.get("gaps", [])
    manual_pending = has_manual_acceptance_pending(gaps)
    if status != "verified" and not manual_pending:
        failures.append(f"{page}: audit_status must be verified before interaction smoke passes")
    defaults = defaults_for(entry, manifest)
    operations = entry.get("required_operations", defaults.get("required_operations", []))
    if not operations:
        failures.append(f"{page}: required_operations is empty")
    declared_operations = operation_kinds(manifest)
    undeclared_operations = sorted(
        operation
        for operation in operations
        if isinstance(operation, str)
        and declared_operations
        and operation not in declared_operations
    )
    for operation in undeclared_operations:
        failures.append(
            f"{page}: required operation is not declared in operation_kinds: {operation}"
        )
    page_scenarios = scenarios.get(page)
    if not page_scenarios:
        failures.append(f"{page}: live interaction audit scenario is missing")
        return failures
    if not any(scenario.get("passed") is True for scenario in page_scenarios):
        failures.append(f"{page}: live interaction audit did not pass")
    if not any(scenario.get("body_pixel_diff", 0) > 0 for scenario in page_scenarios):
        failures.append(f"{page}: live interaction audit did not repaint component body")
    covered_operations = {
        scenario.get("operation_kind")
        for scenario in page_scenarios
        if scenario.get("passed") is True
    }
    missing_operations = sorted(
        operation
        for operation in operations
        if isinstance(operation, str) and operation not in covered_operations
    )
    if missing_operations:
        failures.append(
            f"{page}: live interaction audit missing required operation(s): "
            + ", ".join(missing_operations)
        )
    acceptance_checks = string_values(entry.get("acceptance_checks", []))
    if acceptance_checks:
        covered_checks = {
            acceptance_check_name(entry, scenario)
            for scenario in page_scenarios
            if scenario.get("passed") is True
        }
        missing_checks = sorted(
            check for check in acceptance_checks if check not in covered_checks
        )
        if missing_checks:
            failures.append(
                f"{page}: live interaction audit missing acceptance check(s): "
                + ", ".join(missing_checks)
            )
    if page == "progress-bar" and not has_progress_bar_timed_tick_contract(page_scenarios):
        failures.append(
            "progress-bar: timed_tick must pass progress_tick/progress_changed with percent=82 and repaint"
        )
    if page == "progress-bar" and not has_progress_bar_pointer_contract(page_scenarios):
        failures.append(
            "progress-bar: pointer preview click must pass progress_change/progress_changed with percent=82 and repaint"
        )
    if page == "progress-bar" and not has_progress_bar_timed_cycle_contract(page_scenarios):
        failures.append(
            "progress-bar: timed_tick must include progress_timed_cycle to percent=0 with repaint"
        )
    if page == "progress-bar" and not has_progress_bar_indeterminate_motion_contract(
        page_scenarios
    ):
        failures.append(
            "progress-bar: timed_tick must include indeterminate segment motion with repaint"
        )
    if page == "text" and not has_text_keyboard_copy_contract(page_scenarios):
        failures.append(
            "text: keyboard copy must produce clipboard payload through copy_selection/clipboard_copy"
        )
    if page == "text" and not has_text_keyboard_paste_ignored_contract(page_scenarios):
        failures.append("text: keyboard paste must be ignored for display Text")
    if page == "text-input" and not has_text_entry_paste_contract(
        page_scenarios, "text_input_paste"
    ):
        failures.append(
            "text-input: keyboard paste must replace selection through text_input_paste/clipboard_paste"
        )
    if page == "text-area" and not has_text_entry_paste_contract(
        page_scenarios, "text_area_paste"
    ):
        failures.append(
            "text-area: keyboard paste must replace selection through text_area_paste/clipboard_paste"
        )
    if page == "text" and not has_text_zero_distance_drag_contract(page_scenarios):
        failures.append(
            "text: zero-distance drag must not select, repaint selection, or copy payload"
        )
    return failures


def has_progress_bar_timed_tick_contract(
    page_scenarios: list[dict[str, Any]],
) -> bool:
    for scenario in page_scenarios:
        state = scenario.get("state")
        if (
            scenario.get("operation") == "progress_timed_tick"
            and scenario.get("operation_kind") == "timed_tick"
            and scenario.get("passed") is True
            and scenario.get("action") == "progress_tick"
            and scenario.get("event") == "progress_changed"
            and state == "percent=82"
            and scenario.get("body_pixel_diff", 0) > 0
        ):
            return True
    return False


def has_progress_bar_pointer_contract(page_scenarios: list[dict[str, Any]]) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "preview_click"
            and scenario.get("operation_kind") == "pointer"
            and scenario.get("passed") is True
            and scenario.get("action") == "progress_change"
            and scenario.get("event") == "progress_changed"
            and scenario.get("state") == "percent=82"
            and scenario.get("body_pixel_diff", 0) > 0
        ):
            return True
    return False


def acceptance_check_name(entry: dict[str, Any], scenario: dict[str, Any]) -> Any:
    if entry.get("page") == "progress-bar" and scenario.get("operation") == "preview_click":
        return "progress_preview_click"
    return scenario.get("operation")


def has_progress_bar_timed_cycle_contract(
    page_scenarios: list[dict[str, Any]],
) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "progress_timed_cycle"
            and scenario.get("operation_kind") == "timed_tick"
            and scenario.get("passed") is True
            and scenario.get("action") == "progress_tick"
            and scenario.get("event") == "progress_changed"
            and scenario.get("state") == "percent=0"
            and scenario.get("body_pixel_diff", 0) > 0
        ):
            return True
    return False


def has_progress_bar_indeterminate_motion_contract(
    page_scenarios: list[dict[str, Any]],
) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "progress_indeterminate_segment_motion"
            and scenario.get("operation_kind") == "timed_tick"
            and scenario.get("passed") is True
            and scenario.get("action") == "progress_tick"
            and scenario.get("event") == "progress_changed"
            and scenario.get("state") == "percent=82"
            and scenario.get("body_pixel_diff", 0) > 0
        ):
            return True
    return False


def has_text_keyboard_copy_contract(page_scenarios: list[dict[str, Any]]) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "text_keyboard_copy"
            and scenario.get("operation_kind") == "keyboard"
            and scenario.get("passed") is True
            and scenario.get("action") == "copy_selection"
            and scenario.get("event") == "clipboard_copy"
            and scenario.get("clipboard_text_len", 0) > 0
        ):
            return True
    return False


def has_text_keyboard_paste_ignored_contract(
    page_scenarios: list[dict[str, Any]],
) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "text_keyboard_paste"
            and scenario.get("operation_kind") == "keyboard"
            and scenario.get("passed") is True
            and scenario.get("action") == "none"
            and scenario.get("event") == "none"
        ):
            return True
    return False


def has_text_entry_paste_contract(
    page_scenarios: list[dict[str, Any]], expected_action: str
) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "text_keyboard_paste"
            and scenario.get("operation_kind") == "keyboard"
            and scenario.get("passed") is True
            and scenario.get("action") == expected_action
            and scenario.get("event") == "clipboard_paste"
            and scenario.get("state") == "value=pasted"
        ):
            return True
    return False


def has_text_zero_distance_drag_contract(page_scenarios: list[dict[str, Any]]) -> bool:
    for scenario in page_scenarios:
        if (
            scenario.get("operation") == "text_zero_distance_drag_no_selection"
            and scenario.get("operation_kind") == "drag"
            and scenario.get("passed") is True
            and scenario.get("action") == "none"
            and scenario.get("event") == "none"
            and scenario.get("body_pixel_diff", -1) == 0
            and scenario.get("clipboard_text_len", -1) == 0
        ):
            return True
    return False


def operation_kinds(manifest: dict[str, Any]) -> set[str]:
    values = manifest.get("operation_kinds", [])
    if not isinstance(values, list):
        return set()
    return {value for value in values if isinstance(value, str)}


def string_values(values: Any) -> list[str]:
    if not isinstance(values, list):
        return []
    return [value for value in values if isinstance(value, str)]


def has_manual_acceptance_pending(gaps: Any) -> bool:
    if not isinstance(gaps, list):
        return False
    return any(
        isinstance(gap, str) and "manual_acceptance_pending" in gap for gap in gaps
    )


def defaults_for(entry: dict[str, Any], manifest: dict[str, Any]) -> dict[str, Any]:
    engine = entry.get("engine")
    defaults = manifest.get("defaults_by_engine", {})
    if not isinstance(engine, str) or not isinstance(defaults, dict):
        return {}
    value = defaults.get(engine)
    if isinstance(value, dict):
        return value
    return {}


def scenario_pages(audit: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    scenarios = audit.get("scenarios", [])
    if not isinstance(scenarios, list):
        return {}
    by_page: dict[str, list[dict[str, Any]]] = {}
    for scenario in scenarios:
        if not isinstance(scenario, dict) or not isinstance(scenario.get("page"), str):
            continue
        by_page.setdefault(scenario["page"], []).append(scenario)
    return by_page


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


if __name__ == "__main__":
    raise SystemExit(main())
