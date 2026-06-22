from __future__ import annotations

import json
from pathlib import Path
from typing import Any

MANIFEST_PATH = Path("docs/storybook-77ui-interaction-manifest.json")
OPERATION_KINDS = {
    "pointer",
    "keyboard",
    "scroll",
    "drag",
    "context_menu",
    "focus",
    "hover",
    "resize",
    "timed_tick",
}
AUDIT_STATUSES = {"failed", "partial", "unverified", "verified"}
SURROGATE_FLAGS = (
    "uses_storybook_only_state",
    "uses_inspector_only_change",
    "uses_preset_label_only_change",
    "bypasses_core_public_api",
)
TEST_REFERENCE_KEYS = ("window_interaction", "visual_interaction", "guard")
PAGE_EVIDENCE_REQUIREMENTS = {
    "text": (
        "interactive_page_text_does_not_start_display_text_selection",
        "live_audit_covers_text_selection_and_copy_for_text_page_only",
    ),
    "progress-bar": (
        "progress_bar_timed_tick_advances_via_core_progress_action",
        "progress_bar_timed_tick_cycles_after_reaching_maximum",
        "progress_bar_live_audit_reports_timed_tick_progress_contract",
        "progress_bar_live_audit_reports_timed_cycle_after_maximum",
        "progress_bar_live_audit_reports_indeterminate_segment_motion",
        "progress_bar_indeterminate_segment_moves_on_runtime_tick",
        "progress_bar_dedicated_render_uses_core_progress_bar_public_api",
        "progress_bar_window_runtime_tick_repaints_meter_body",
        "progress_bar_window_runtime_tick_cycles_after_maximum",
    ),
    "tree-view": (
        "navigation_scroll_retained_when_selecting_tree_view_after_deep_scroll",
    ),
}


class StorybookUiHarnessManifest:
    def __init__(self, root: Path, required_pages: list[str]) -> None:
        self.root = root
        self.required_pages = required_pages

    def failures(self) -> list[str]:
        path = self.root / MANIFEST_PATH
        if not path.exists():
            return [f"{MANIFEST_PATH}: Storybook 77 UI interaction manifest is missing"]
        try:
            manifest = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as error:
            return [f"{MANIFEST_PATH}: invalid JSON: {error}"]

        failures: list[str] = []
        failures.extend(self.policy_failures(manifest))
        entries = manifest.get("ui")
        if not isinstance(entries, list):
            return failures + [f"{MANIFEST_PATH}: `ui` must be an array"]

        pages = [entry.get("page") for entry in entries if isinstance(entry, dict)]
        failures.extend(self.page_coverage_failures(pages))
        for entry in entries:
            if not isinstance(entry, dict):
                failures.append(f"{MANIFEST_PATH}: every ui entry must be an object")
                continue
            failures.extend(self.entry_failures(entry, manifest))
        return failures

    def policy_failures(self, manifest: dict[str, Any]) -> list[str]:
        policy = manifest.get("policy")
        if not isinstance(policy, dict):
            return [f"{MANIFEST_PATH}: `policy` must be an object"]
        failures: list[str] = []
        if policy.get("harness_scope") != "core_public_api_harness":
            failures.append(
                f"{MANIFEST_PATH}: policy.harness_scope must be core_public_api_harness"
            )
        for flag in SURROGATE_FLAGS:
            if policy.get(flag) is not False:
                failures.append(f"{MANIFEST_PATH}: policy.{flag} must be false")
        return failures

    def page_coverage_failures(self, pages: list[Any]) -> list[str]:
        expected = set(self.required_pages)
        actual = {page for page in pages if isinstance(page, str)}
        failures = [
            f"{MANIFEST_PATH}: required page missing from manifest: {page}"
            for page in sorted(expected - actual)
        ]
        failures.extend(
            f"{MANIFEST_PATH}: manifest page is not in requirements.rs: {page}"
            for page in sorted(actual - expected)
        )
        if len(pages) != len(set(pages)):
            failures.append(f"{MANIFEST_PATH}: duplicate ui.page entries are not allowed")
        return failures

    def entry_failures(self, entry: dict[str, Any], manifest: dict[str, Any]) -> list[str]:
        page = entry.get("page", "<missing>")
        failures: list[str] = []
        engine = entry.get("engine")
        engine_defaults = {}
        defaults_by_engine = manifest.get("defaults_by_engine")
        if isinstance(defaults_by_engine, dict) and isinstance(engine, str):
            default_value = defaults_by_engine.get(engine)
            if isinstance(default_value, dict):
                engine_defaults = default_value
        required_lists = (
            "public_props_options",
            "state",
            "action",
            "event",
            "callback",
            "required_operations",
            "evidence",
        )
        for key in required_lists:
            value = entry.get(key, engine_defaults.get(key))
            if not isinstance(value, list) or not value:
                failures.append(f"{MANIFEST_PATH}: {page}.{key} must be a non-empty array")
        gaps = entry.get("gaps")
        if not isinstance(gaps, list):
            failures.append(f"{MANIFEST_PATH}: {page}.gaps must be an array")
        operations = set(entry.get("required_operations", engine_defaults.get("required_operations", [])))
        unknown_operations = sorted(operations - OPERATION_KINDS)
        if unknown_operations:
            failures.append(
                f"{MANIFEST_PATH}: {page}.required_operations has unknown operation(s): "
                + ", ".join(unknown_operations)
            )
        tests = entry.get("tests", engine_defaults.get("tests"))
        if not isinstance(tests, dict):
            failures.append(f"{MANIFEST_PATH}: {page}.tests must be an object")
        else:
            for key in TEST_REFERENCE_KEYS:
                value = tests.get(key)
                if not isinstance(value, list) or not value:
                    failures.append(
                        f"{MANIFEST_PATH}: {page}.tests.{key} must be a non-empty array"
                    )
                    continue
                failures.extend(self.test_reference_failures(page, key, value))
            failures.extend(self.manual_pending_test_failures(page, gaps, tests))
        failures.extend(self.manual_pending_acceptance_contract_failures(page, gaps, entry))
        status = entry.get("audit_status")
        if status not in AUDIT_STATUSES:
            failures.append(
                f"{MANIFEST_PATH}: {page}.audit_status must be one of "
                + ", ".join(sorted(AUDIT_STATUSES))
            )
        if status == "verified" and gaps:
            failures.append(f"{MANIFEST_PATH}: {page} cannot be verified while gaps remain")
        if status != "verified" and gaps == []:
            failures.append(f"{MANIFEST_PATH}: {page}.gaps must explain remaining audit work")
        failures.extend(self.page_evidence_failures(page, entry.get("evidence", [])))
        return failures

    @staticmethod
    def manual_pending_test_failures(
        page: Any,
        gaps: Any,
        tests: dict[str, Any],
    ) -> list[str]:
        if not isinstance(page, str) or not isinstance(gaps, list):
            return []
        if not any(
            isinstance(gap, str) and "manual_acceptance_pending" in gap for gap in gaps
        ):
            return []
        guard = tests.get("guard")
        guard_text = "\n".join(item for item in guard if isinstance(item, str))
        required = (
            "scripts/storybook_manual_acceptance_smoke.py",
            "scripts/test_storybook_manual_acceptance_smoke.py",
        )
        return [
            f"{MANIFEST_PATH}: {page} manual pending tests.guard must include {needle}"
            for needle in required
            if needle not in guard_text
        ]

    @staticmethod
    def manual_pending_acceptance_contract_failures(
        page: Any,
        gaps: Any,
        entry: dict[str, Any],
    ) -> list[str]:
        if not isinstance(page, str) or not isinstance(gaps, list):
            return []
        if not any(
            isinstance(gap, str) and "manual_acceptance_pending" in gap for gap in gaps
        ):
            return []
        failures: list[str] = []
        for key in ("acceptance_checks", "acceptance_observations"):
            value = entry.get(key)
            if not isinstance(value, list) or not value:
                failures.append(
                    f"{MANIFEST_PATH}: {page}.{key} must be a non-empty array while manual acceptance is pending"
                )
        frames = entry.get("minimum_observation_frames")
        if not isinstance(frames, int) or frames <= 0:
            failures.append(
                f"{MANIFEST_PATH}: {page}.minimum_observation_frames must be a positive integer while manual acceptance is pending"
            )
        return failures

    @staticmethod
    def page_evidence_failures(page: Any, evidence: Any) -> list[str]:
        if not isinstance(page, str):
            return []
        required = PAGE_EVIDENCE_REQUIREMENTS.get(page, ())
        if not required:
            return []
        evidence_text = "\n".join(item for item in evidence if isinstance(item, str))
        return [
            f"{MANIFEST_PATH}: {page} evidence must include {needle}"
            for needle in required
            if needle not in evidence_text
        ]

    def test_reference_failures(self, page: Any, key: str, references: list[Any]) -> list[str]:
        failures: list[str] = []
        for reference in references:
            if not isinstance(reference, str) or not reference.strip():
                failures.append(
                    f"{MANIFEST_PATH}: {page}.tests.{key} entries must be non-empty strings"
                )
                continue
            if "<page>" in reference:
                failures.append(
                    f"{MANIFEST_PATH}: {page}.tests.{key} must not use <page> placeholders"
                )
                continue
            if reference.startswith(("shared:", "source:")):
                continue
            if not (self.root / reference).exists():
                failures.append(
                    f"{MANIFEST_PATH}: {page}.tests.{key} reference does not exist: {reference}"
                )
        return failures
