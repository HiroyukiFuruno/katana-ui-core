from __future__ import annotations

import json
from pathlib import Path

LEDGER_PATH = Path("docs/storybook-77ui-deep-audit-ledger.md")
REPAIR_PLAN_PATH = Path("docs/storybook-77ui-repair-plan.md")
MANIFEST_PATH = Path("docs/storybook-77ui-interaction-manifest.json")

PROGRESS_BAR_REQUIRED_TOKENS = (
    "target/manual-ui-probe/native-matrix-expanded-v3/summary.json",
    "progress_preview_click",
    "progress_timed_tick",
    "progress_timed_cycle",
    "progress_bar_timed_tick_advances_via_core_progress_action",
    "progress_bar_timed_tick_cycles_after_reaching_maximum",
    "progress_bar_live_audit_reports_timed_tick_progress_contract",
    "progress_bar_live_audit_reports_timed_cycle_after_maximum",
    "progress_bar_live_audit_reports_indeterminate_segment_motion",
    "progress_bar_indeterminate_segment_moves_on_runtime_tick",
    "progress_bar_window_runtime_tick_repaints_meter_body",
    "progress_bar_window_runtime_tick_cycles_after_maximum",
    "progress_bar_dedicated_render_uses_core_progress_bar_public_api",
)


class StorybookUiHarnessLedger:
    def __init__(self, root: Path) -> None:
        self.root = root

    def failures(self) -> list[str]:
        path = self.root / LEDGER_PATH
        if not path.exists():
            return [f"{LEDGER_PATH}: deep audit ledger is missing"]
        source = path.read_text(encoding="utf-8")
        failures: list[str] = []
        progress_section = self.section_for(source, "progress-bar")
        if progress_section is None:
            return [f"{LEDGER_PATH}: progress-bar ledger entry is missing"]
        for token in PROGRESS_BAR_REQUIRED_TOKENS:
            if token not in progress_section:
                failures.append(f"{LEDGER_PATH}: progress-bar ledger must include {token}")
        failures.extend(self.manual_pending_entrypoint_failures())
        return failures

    def manual_pending_entrypoint_failures(self) -> list[str]:
        pages = self.manual_pending_pages()
        failures: list[str] = []
        if not pages:
            return failures
        path = self.root / REPAIR_PLAN_PATH
        if not path.exists():
            return [f"{REPAIR_PLAN_PATH}: repair plan is missing"]
        source = path.read_text(encoding="utf-8")
        for page in pages:
            section = self.section_for(source, page)
            if section is None:
                failures.append(f"{REPAIR_PLAN_PATH}: {page} repair plan entry is missing")
                continue
            token = f"--open-window {page}"
            if token not in section:
                failures.append(
                    f"{REPAIR_PLAN_PATH}: {page} manual confirmation entrypoint "
                    f"must include `{token}`"
                )
            smoke_token = "storybook-manual-acceptance-smoke"
            if smoke_token not in section:
                failures.append(
                    f"{REPAIR_PLAN_PATH}: {page} manual confirmation smoke "
                    f"must include `{smoke_token}`"
                )
            smoke_window_token = f"--open-window {self.minimum_observation_frames(page)} {page}"
            if smoke_window_token not in section:
                failures.append(
                    f"{REPAIR_PLAN_PATH}: {page} manual confirmation smoke "
                    f"must include `{smoke_window_token}`"
                )
        return failures

    @staticmethod
    def minimum_observation_frames(page: str) -> int:
        if page == "progress-bar":
            return 48
        return 1

    def manual_pending_pages(self) -> list[str]:
        path = self.root / MANIFEST_PATH
        if not path.exists():
            return []
        manifest = json.loads(path.read_text(encoding="utf-8"))
        entries = manifest.get("ui")
        if not isinstance(entries, list):
            return []
        pages: list[str] = []
        for entry in entries:
            if not isinstance(entry, dict):
                continue
            page = entry.get("page")
            gaps = entry.get("gaps", [])
            if not isinstance(page, str) or not isinstance(gaps, list):
                continue
            if any("manual_acceptance_pending" in gap for gap in gaps if isinstance(gap, str)):
                pages.append(page)
        return pages

    @staticmethod
    def section_for(source: str, page: str) -> str | None:
        marker = f"## UI: {page}"
        start = source.find(marker)
        if start >= 0:
            end = source.find("\n## UI:", start + len(marker))
            return source[start:] if end < 0 else source[start:end]
        lines = [line for line in source.splitlines() if f"| {page} |" in line]
        if not lines:
            return None
        return "\n".join(lines)
