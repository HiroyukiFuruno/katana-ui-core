#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shlex
import subprocess
from pathlib import Path
from typing import Any, Protocol

from storybook_manual_acceptance_queue import MANIFEST_PATH, manual_acceptance_queue

AUDIT_PATH = Path("target/storybook-live-interaction-audit.json")
EVIDENCE_PATH = Path("target/storybook-manual-acceptance-evidence.json")


class CommandRunner(Protocol):
    def __call__(self, command: list[str]) -> int: ...


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--audit", type=Path, default=AUDIT_PATH)
    parser.add_argument("--evidence", type=Path, default=EVIDENCE_PATH)
    parser.add_argument("--page", action="append", default=[])
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    failures = manual_acceptance_smoke_failures(
        args.manifest,
        lambda command: subprocess.run(command, check=False).returncode,
        set(args.page),
        args.audit,
    )
    if failures:
        print("storybook manual acceptance smoke failed")
        for failure in failures:
            print(f"- {failure}")
        return 1
    pages = set(args.page)
    write_evidence_report(args.manifest, args.audit, args.evidence, pages)
    evidence_failures = manual_acceptance_evidence_report_failures(
        args.manifest,
        args.evidence,
        pages,
    )
    if evidence_failures:
        print("storybook manual acceptance evidence failed")
        for failure in evidence_failures:
            print(f"- {failure}")
        return 1
    print("storybook manual acceptance smoke passed")
    return 0


def manual_acceptance_smoke_failures(
    manifest_path: Path,
    runner: CommandRunner,
    pages: set[str] | None = None,
    audit_path: Path | None = None,
) -> list[str]:
    failures: list[str] = []
    audit = audit_scenarios_by_page_and_operation(audit_path) if audit_path else None
    for entry in manual_acceptance_queue(manifest_path):
        page = entry.get("page")
        if not isinstance(page, str):
            failures.append("manual acceptance queue entry is missing page")
            continue
        if pages and page not in pages:
            continue
        failures.extend(acceptance_contract_failures(entry))
        if audit is not None:
            failures.extend(acceptance_audit_failures(entry, audit))
        command = entry.get("smoke_command")
        if not isinstance(command, str) or not command.strip():
            failures.append(f"{page}: smoke_command is missing")
            continue
        result = runner(shlex.split(command))
        if result != 0:
            failures.append(f"{page}: smoke_command failed with exit code {result}")
    return failures


def acceptance_contract_failures(entry: dict[str, Any]) -> list[str]:
    page = entry.get("page")
    if not isinstance(page, str):
        return []
    failures: list[str] = []
    checks = entry.get("acceptance_checks")
    if not isinstance(checks, list) or not string_values(checks):
        failures.append(f"{page}: acceptance_checks must be a non-empty list")
    observations = entry.get("acceptance_observations")
    if not isinstance(observations, list) or not string_values(observations):
        failures.append(f"{page}: acceptance_observations must be a non-empty list")
    frames = entry.get("minimum_observation_frames")
    if not isinstance(frames, int) or frames <= 0:
        failures.append(f"{page}: minimum_observation_frames must be a positive integer")
    return failures


def manual_acceptance_evidence_report(
    manifest_path: Path,
    audit_path: Path,
    pages: set[str] | None = None,
) -> list[dict[str, Any]]:
    audit = audit_scenarios_by_page_and_operation(audit_path)
    report: list[dict[str, Any]] = []
    for entry in manual_acceptance_queue(manifest_path):
        page = entry.get("page")
        if not isinstance(page, str):
            continue
        if pages and page not in pages:
            continue
        checks = string_values(entry.get("acceptance_checks", []))
        report.append(
            {
                "page": page,
                "command": entry.get("command"),
                "smoke_command": entry.get("smoke_command"),
                "minimum_observation_frames": entry.get("minimum_observation_frames"),
                "acceptance_observations": string_values(
                    entry.get("acceptance_observations", [])
                ),
                "audit_evidence": [
                    audit_evidence_for_check(
                        check,
                        audit.get((page, audit_operation_for_check(check))),
                    )
                    for check in checks
                ],
            }
        )
    return report


def write_evidence_report(
    manifest_path: Path,
    audit_path: Path,
    evidence_path: Path,
    pages: set[str] | None = None,
) -> None:
    report = manual_acceptance_evidence_report(manifest_path, audit_path, pages)
    evidence_path.parent.mkdir(parents=True, exist_ok=True)
    evidence_path.write_text(
        json.dumps(report, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def manual_acceptance_evidence_report_failures(
    manifest_path: Path,
    evidence_path: Path,
    pages: set[str] | None = None,
) -> list[str]:
    queue_all = manual_acceptance_queue(manifest_path)
    queue = [entry for entry in queue_all if not pages or entry.get("page") in pages]
    evidence = load_evidence_report(evidence_path)
    if not isinstance(evidence, list):
        return ["manual acceptance evidence report must be an array"]
    allowed_pages = evidence_allowed_pages(manifest_path, queue_all)
    future_pending_pages = {
        entry.get("page")
        for entry in queue_all
        if pages and isinstance(entry.get("page"), str) and entry.get("page") not in pages
    }
    failures: list[str] = []
    evidence_by_page: dict[str, dict[str, Any]] = {}
    for index, entry in enumerate(evidence):
        if not isinstance(entry, dict):
            failures.append(f"manual acceptance evidence[{index}]: entry must be an object")
            continue
        page = entry.get("page")
        if not isinstance(page, str) or not page:
            failures.append(
                f"manual acceptance evidence[{index}]: page must be a non-empty string"
            )
            continue
        if page in future_pending_pages:
            failures.append(f"{page}: manual acceptance evidence has future pending page")
            continue
        if page not in allowed_pages:
            failures.append(f"{page}: manual acceptance evidence has unexpected page")
            continue
        if page in evidence_by_page:
            failures.append(
                f"{page}: manual acceptance evidence report entry is duplicated"
            )
            continue
        evidence_by_page[page] = entry
    for queue_entry in queue:
        page = queue_entry.get("page")
        if not isinstance(page, str):
            continue
        evidence_entry = evidence_by_page.get(page)
        if not isinstance(evidence_entry, dict):
            failures.append(f"{page}: manual acceptance evidence report entry is missing")
            continue
        failures.extend(evidence_entry_failures(queue_entry, evidence_entry))
    return failures


def evidence_allowed_pages(
    manifest_path: Path,
    queue: list[dict[str, Any]],
) -> set[str]:
    allowed = {
        entry.get("page") for entry in queue if isinstance(entry.get("page"), str)
    }
    payload = json.loads(manifest_path.read_text(encoding="utf-8"))
    entries = payload.get("ui", [])
    if not isinstance(entries, list):
        return allowed
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        if not isinstance(page, str):
            continue
        if entry.get("acceptance_checks") or entry.get("acceptance_observations"):
            allowed.add(page)
    return allowed


def evidence_entry_failures(
    queue_entry: dict[str, Any],
    evidence_entry: dict[str, Any],
) -> list[str]:
    page = queue_entry.get("page", "<missing>")
    failures: list[str] = []
    if evidence_entry.get("command") != queue_entry.get("command"):
        failures.append(f"{page}: manual acceptance evidence command does not match queue")
    if evidence_entry.get("smoke_command") != queue_entry.get("smoke_command"):
        failures.append(f"{page}: manual acceptance evidence smoke_command does not match queue")
    if evidence_entry.get("minimum_observation_frames") != queue_entry.get(
        "minimum_observation_frames"
    ):
        failures.append(
            f"{page}: manual acceptance evidence minimum_observation_frames does not match queue"
        )
    expected_observations = string_values(queue_entry.get("acceptance_observations", []))
    actual_observations = string_values(evidence_entry.get("acceptance_observations", []))
    if actual_observations != expected_observations:
        failures.append(f"{page}: manual acceptance evidence observations do not match queue")
    expected_checks = string_values(queue_entry.get("acceptance_checks", []))
    expected_check_set = set(expected_checks)
    actual_evidence = evidence_entry.get("audit_evidence", [])
    if not isinstance(actual_evidence, list):
        failures.append(f"{page}: manual acceptance audit_evidence must be an array")
        return failures
    actual_by_check: dict[str, dict[str, Any]] = {}
    for index, item in enumerate(actual_evidence):
        if not isinstance(item, dict):
            failures.append(f"{page}: manual acceptance audit_evidence[{index}] must be an object")
            continue
        check = item.get("check")
        if not isinstance(check, str) or not check:
            failures.append(
                f"{page}: manual acceptance audit_evidence[{index}] check must be a non-empty string"
            )
            continue
        if check in actual_by_check:
            failures.append(
                f"{page}: manual acceptance evidence check {check} is duplicated"
            )
            continue
        if check not in expected_check_set:
            failures.append(
                f"{page}: manual acceptance evidence has unexpected check {check}"
            )
            continue
        actual_by_check[check] = item
    for check in expected_checks:
        item = actual_by_check.get(check)
        if not isinstance(item, dict):
            failures.append(f"{page}: manual acceptance evidence missing check {check}")
            continue
        if item.get("passed") is not True:
            failures.append(f"{page}: manual acceptance evidence check {check} did not pass")
        else:
            failures.extend(evidence_item_contract_failures(page, check, item))
    return failures


def evidence_item_contract_failures(
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
            f"{page}: manual acceptance evidence {check} must include operation_kind {expected_kind}"
        )
    if check == "text_drag_selection" and int_value(item.get("body_pixel_diff")) <= 0:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include positive body_pixel_diff"
        )
    if check == "text_drag_selection":
        if item.get("state") != "selection=active":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include state selection=active"
            )
        if item.get("action") != "select_text":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include action select_text"
            )
        if item.get("event") != "text_selection_changed":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include event text_selection_changed"
            )
    if check == "text_keyboard_copy" and int_value(item.get("clipboard_text_len")) <= 0:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include positive clipboard_text_len"
        )
    if check == "text_keyboard_copy":
        if item.get("state") != "clipboard=selected_text":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include state clipboard=selected_text"
            )
        if item.get("action") != "copy_selection":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include action copy_selection"
            )
        if item.get("event") != "clipboard_copy":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include event clipboard_copy"
            )
    if check == "text_zero_distance_drag_no_selection":
        if item.get("state") != "idle":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include state idle"
            )
        if int_value(item.get("body_pixel_diff")) != 0:
            failures.append(
                f"{page}: manual acceptance evidence {check} must include zero body_pixel_diff"
            )
        if int_value(item.get("clipboard_text_len")) != 0:
            failures.append(
                f"{page}: manual acceptance evidence {check} must include zero clipboard_text_len"
            )
        if item.get("action") != "none":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include action none"
            )
        if item.get("event") != "none":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include event none"
            )
    if check == "text_keyboard_paste":
        if item.get("state") != "idle":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include state idle"
            )
        if int_value(item.get("body_pixel_diff")) != 0:
            failures.append(
                f"{page}: manual acceptance evidence {check} must not change rendered text pixels"
            )
        if int_value(item.get("clipboard_text_len")) != 0:
            failures.append(
                f"{page}: manual acceptance evidence {check} must not copy text to clipboard"
            )
        if item.get("action") != "none":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include action none"
            )
        if item.get("event") != "none":
            failures.append(
                f"{page}: manual acceptance evidence {check} must include event none"
            )
    if check in ("row_click", "checkbox_pointer_checks_both_rows"):
        failures.extend(checkbox_pointer_evidence_item_failures(page, check, item))
    if check.startswith("progress_"):
        failures.extend(progress_evidence_item_failures(page, check, item))
    return failures


def checkbox_pointer_evidence_item_failures(
    page: str,
    check: str,
    item: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    if item.get("operation_kind") != "pointer":
        failures.append(
            f"{page}: manual acceptance evidence {check} must include operation_kind pointer"
        )
    if item.get("action") != "checkbox_toggle":
        failures.append(
            f"{page}: manual acceptance evidence {check} must include action checkbox_toggle"
        )
    if item.get("event") != "checked_changed":
        failures.append(
            f"{page}: manual acceptance evidence {check} must include event checked_changed"
        )
    if int_value(item.get("body_pixel_diff")) <= 0:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include positive body_pixel_diff"
        )
    return failures


def progress_evidence_item_failures(
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
    expected_kind = "pointer" if check == "progress_preview_click" else "timed_tick"
    expected_action = (
        "progress_change" if check == "progress_preview_click" else "progress_tick"
    )
    if item.get("operation_kind") != expected_kind:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include operation_kind {expected_kind}"
        )
    if item.get("action") != expected_action:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include action {expected_action}"
        )
    if item.get("event") != "progress_changed":
        failures.append(
            f"{page}: manual acceptance evidence {check} must include event progress_changed"
        )
    if expected_state is not None and item.get("state") != expected_state:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include state {expected_state}"
        )
    if int_value(item.get("body_pixel_diff")) <= 0:
        failures.append(
            f"{page}: manual acceptance evidence {check} must include positive body_pixel_diff"
        )
    return failures


def load_evidence_report(evidence_path: Path) -> Any:
    if not evidence_path.exists():
        return None
    return json.loads(evidence_path.read_text(encoding="utf-8"))


def audit_scenarios_by_page_and_operation(
    audit_path: Path,
) -> dict[tuple[str, str], dict[str, Any]]:
    if not audit_path.exists():
        return {}
    payload = json.loads(audit_path.read_text(encoding="utf-8"))
    scenarios = payload.get("scenarios", [])
    if not isinstance(scenarios, list):
        return {}
    indexed: dict[tuple[str, str], dict[str, Any]] = {}
    for scenario in scenarios:
        if not isinstance(scenario, dict):
            continue
        page = scenario.get("page")
        operation = scenario.get("operation")
        if isinstance(page, str) and isinstance(operation, str):
            indexed[(page, operation)] = scenario
    return indexed


def audit_evidence_for_check(
    check: str,
    scenario: dict[str, Any] | None,
) -> dict[str, Any]:
    evidence: dict[str, Any] = {"check": check}
    if scenario is None:
        evidence["passed"] = False
        return evidence
    evidence["passed"] = scenario.get("passed") is True
    for key in (
        "operation_kind",
        "state",
        "action",
        "event",
        "body_pixel_diff",
        "clipboard_text_len",
    ):
        if key in scenario:
            evidence[key] = scenario[key]
    return evidence


def acceptance_audit_failures(
    entry: dict[str, Any],
    audit: dict[tuple[str, str], dict[str, Any]],
) -> list[str]:
    page = entry.get("page")
    if not isinstance(page, str):
        return []
    failures: list[str] = []
    checks = entry.get("acceptance_checks", [])
    if not isinstance(checks, list):
        return [f"{page}: acceptance_checks must be a list"]
    for check in checks:
        if not isinstance(check, str):
            continue
        scenario = audit.get((page, audit_operation_for_check(check)))
        if scenario is None:
            failures.append(f"{page}: {check} is missing from live interaction audit")
            continue
        failures.extend(scenario_contract_failures(page, check, scenario))
    return failures


def scenario_contract_failures(
    page: str,
    check: str,
    scenario: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    if scenario.get("passed") is not True:
        failures.append(f"{page}: {check} must pass live interaction audit")
    if check.startswith("progress_"):
        failures.extend(progress_contract_failures(page, check, scenario))
    if check in ("row_click", "checkbox_pointer_checks_both_rows"):
        failures.extend(checkbox_pointer_contract_failures(page, check, scenario))
    if check.startswith("text_"):
        failures.extend(text_contract_failures(page, check, scenario))
    return failures


def checkbox_pointer_contract_failures(
    page: str,
    check: str,
    scenario: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    if scenario.get("operation_kind") != "pointer":
        failures.append(f"{page}: {check} must be a pointer operation")
    if scenario.get("action") != "checkbox_toggle":
        failures.append(f"{page}: {check} must use action checkbox_toggle")
    if scenario.get("event") != "checked_changed":
        failures.append(f"{page}: {check} must emit event checked_changed")
    if int_value(scenario.get("body_pixel_diff")) <= 0:
        failures.append(f"{page}: {check} must change rendered checkbox pixels")
    return failures


def text_contract_failures(
    page: str,
    check: str,
    scenario: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    if check == "text_drag_selection" and int_value(scenario.get("body_pixel_diff")) <= 0:
        failures.append(f"{page}: {check} must change rendered selection pixels")
    if check == "text_drag_selection":
        if scenario.get("state") != "selection=active":
            failures.append(f"{page}: {check} must reach state selection=active")
        if scenario.get("action") != "select_text":
            failures.append(f"{page}: {check} must emit action select_text")
        if scenario.get("event") != "text_selection_changed":
            failures.append(f"{page}: {check} must emit event text_selection_changed")
    if check == "text_keyboard_copy" and int_value(scenario.get("clipboard_text_len")) <= 0:
        failures.append(f"{page}: {check} must copy selectable text to clipboard")
    if check == "text_keyboard_copy":
        if scenario.get("state") != "clipboard=selected_text":
            failures.append(f"{page}: {check} must reach state clipboard=selected_text")
        if scenario.get("action") != "copy_selection":
            failures.append(f"{page}: {check} must emit action copy_selection")
        if scenario.get("event") != "clipboard_copy":
            failures.append(f"{page}: {check} must emit event clipboard_copy")
    if check == "text_zero_distance_drag_no_selection":
        if scenario.get("state") != "idle":
            failures.append(f"{page}: {check} must remain state idle")
        if int_value(scenario.get("body_pixel_diff")) != 0:
            failures.append(f"{page}: {check} must not change rendered selection pixels")
        if int_value(scenario.get("clipboard_text_len")) != 0:
            failures.append(f"{page}: {check} must not copy text to clipboard")
        if scenario.get("action") != "none":
            failures.append(f"{page}: {check} must not emit a selection action")
        if scenario.get("event") != "none":
            failures.append(f"{page}: {check} must not emit a selection event")
    if check == "text_keyboard_paste":
        if scenario.get("state") != "idle":
            failures.append(f"{page}: {check} must remain state idle")
        if int_value(scenario.get("body_pixel_diff")) != 0:
            failures.append(f"{page}: {check} must not change rendered text pixels")
        if int_value(scenario.get("clipboard_text_len")) != 0:
            failures.append(f"{page}: {check} must not copy text to clipboard")
        if scenario.get("action") != "none":
            failures.append(f"{page}: {check} must not emit a paste action")
        if scenario.get("event") != "none":
            failures.append(f"{page}: {check} must not emit a paste event")
    expected_kind = {
        "text_drag_selection": "drag",
        "text_keyboard_copy": "keyboard",
        "text_keyboard_paste": "keyboard",
        "text_zero_distance_drag_no_selection": "drag",
    }.get(check)
    if expected_kind is not None and scenario.get("operation_kind") != expected_kind:
        failures.append(f"{page}: {check} must be a {expected_kind} operation")
    return failures


def progress_contract_failures(
    page: str,
    check: str,
    scenario: dict[str, Any],
) -> list[str]:
    failures: list[str] = []
    expected_state = {
        "progress_preview_click": "percent=82",
        "progress_timed_tick": "percent=82",
        "progress_timed_cycle": "percent=0",
        "progress_indeterminate_segment_motion": "percent=82",
    }.get(check)
    expected_kind = "pointer" if check == "progress_preview_click" else "timed_tick"
    expected_action = "progress_change" if check == "progress_preview_click" else "progress_tick"
    if scenario.get("operation_kind") != expected_kind:
        failures.append(f"{page}: {check} must be a {expected_kind} operation")
    if scenario.get("action") != expected_action:
        failures.append(f"{page}: {check} must use action {expected_action}")
    if scenario.get("event") != "progress_changed":
        failures.append(f"{page}: {check} must emit event progress_changed")
    if expected_state is not None and scenario.get("state") != expected_state:
        failures.append(f"{page}: {check} must reach state {expected_state}")
    if int_value(scenario.get("body_pixel_diff")) <= 0:
        failures.append(f"{page}: {check} must change rendered progress pixels")
    return failures


def audit_operation_for_check(check: str) -> str:
    if check == "progress_preview_click":
        return "preview_click"
    return check


def int_value(value: Any) -> int:
    return value if isinstance(value, int) else 0


def string_values(values: Any) -> list[str]:
    if not isinstance(values, list):
        return []
    return [value for value in values if isinstance(value, str)]


if __name__ == "__main__":
    raise SystemExit(main())
