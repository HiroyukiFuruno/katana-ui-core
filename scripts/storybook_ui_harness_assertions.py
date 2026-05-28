from __future__ import annotations

from pathlib import Path

from storybook_ui_harness_sources import StorybookUiHarnessSources

MIN_PRESET_COUNT = 4
MIN_OPTION_COUNT = 4


class StorybookUiHarness:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.sources = StorybookUiHarnessSources(root)

    def failures(self) -> list[str]:
        required = self.sources.required_pages()
        preset_counts = self.sources.preset_counts()
        option_pages = self.sources.option_pages()
        option_counts = self.sources.option_counts()
        menu_pages = self.sources.menu_pages()
        leaf_changes = self.sources.leaf_changes()
        leaf_statuses = self.sources.leaf_statuses()
        dedicated_pages = self.sources.dedicated_pages()
        priority_order = self.sources.priority_order()
        failures: list[str] = []
        failures.extend(self.missing_presets(required, preset_counts))
        failures.extend(self.missing_options(required, option_pages))
        failures.extend(self.low_option_counts(required, option_pages, option_counts))
        failures.extend(self.low_preset_counts(required, preset_counts))
        failures.extend(self.menu_mismatch_failures(required, menu_pages))
        failures.extend(self.leaf_change_failures(menu_pages, leaf_changes))
        failures.extend(self.dedicated_page_failures(leaf_statuses, dedicated_pages))
        failures.extend(self.priority_order_failures(menu_pages, leaf_changes, priority_order))
        failures.extend(self.text_input_runtime_state_failures())
        failures.extend(self.preset_tab_label_layout_failures())
        if "storybook_ui_option_contract::settings_rows_for" not in self.sources.read(
            "crates/katana-ui-core-storybook/src/visual/inspector_rows.rs"
        ):
            failures.append("Inspector settings must render Storybook UI option contract rows")
        return failures

    @staticmethod
    def menu_mismatch_failures(required: list[str], menu_pages: list[str]) -> list[str]:
        required_set = set(required)
        menu_set = set(menu_pages)
        failures = [
            f"{page}: required page missing from Storybook menu"
            for page in sorted(required_set - menu_set)
        ]
        failures.extend(
            f"{page}: Storybook menu page missing from requirements.rs"
            for page in sorted(menu_set - required_set)
        )
        return failures

    def leaf_change_failures(self, menu_pages: list[str], leaf_changes: dict[str, str]) -> list[str]:
        if not leaf_changes:
            return ["storybook menu leaf change split document is missing or empty"]
        menu_set = set(menu_pages)
        leaf_set = set(leaf_changes)
        failures = [
            f"{page}: Storybook menu page missing leaf change"
            for page in sorted(menu_set - leaf_set)
        ]
        failures.extend(
            f"{page}: leaf change exists without Storybook menu page"
            for page in sorted(leaf_set - menu_set)
        )
        for page in sorted(menu_set & leaf_set):
            failures.extend(self.missing_leaf_artifacts(page, leaf_changes[page]))
        return failures

    def missing_leaf_artifacts(self, page: str, change: str) -> list[str]:
        change_root = self.root / "openspec/changes" / change
        expected = [
            change_root / ".openspec.yaml",
            change_root / "proposal.md",
            change_root / "tasks.md",
            change_root / "specs" / change / "spec.md",
        ]
        return [
            f"{page}: leaf change `{change}` missing {path.relative_to(self.root)}"
            for path in expected
            if not path.exists()
        ]

    @staticmethod
    def dedicated_page_failures(leaf_statuses: dict[str, str], dedicated_pages: set[str]) -> list[str]:
        failures = [
            f"{page}: split table says page-specific rendering exists, but draw_page has no branch"
            for page, status in sorted(leaf_statuses.items())
            if status == "page別描画あり" and page not in dedicated_pages
        ]
        failures.extend(
            f"{page}: draw_page has a branch, but split table does not mark page-specific rendering"
            for page in sorted(dedicated_pages)
            if leaf_statuses.get(page) == "page別描画未作成"
        )
        return failures

    @staticmethod
    def priority_order_failures(
        menu_pages: list[str],
        leaf_changes: dict[str, str],
        priority_order: list[tuple[str, str, str]],
    ) -> list[str]:
        if not priority_order:
            return ["storybook menu priority order document is missing or empty"]
        failures: list[str] = []
        priorities = [priority for priority, _page, _change in priority_order]
        pages = [page for _priority, page, _change in priority_order]
        changes = [change for _priority, _page, change in priority_order]
        failures.extend(StorybookUiHarness.duplicate_failures("priority", priorities))
        failures.extend(StorybookUiHarness.duplicate_failures("priority page", pages))
        failures.extend(StorybookUiHarness.duplicate_failures("priority leaf change", changes))
        failures.extend(StorybookUiHarness.priority_gap_failures(priorities))
        failures.extend(StorybookUiHarness.missing_priority_failures(menu_pages, pages))
        for _priority, page, change in priority_order:
            if leaf_changes.get(page) != change:
                failures.append(f"{page}: priority leaf change `{change}` does not match split table")
        return failures

    @staticmethod
    def missing_priority_failures(menu_pages: list[str], pages: list[str]) -> list[str]:
        menu_set = set(menu_pages)
        priority_pages = set(pages)
        failures = [
            f"{page}: Storybook menu page missing priority number"
            for page in sorted(menu_set - priority_pages)
        ]
        failures.extend(
            f"{page}: priority number exists without Storybook menu page"
            for page in sorted(priority_pages - menu_set)
        )
        return failures

    @staticmethod
    def duplicate_failures(label: str, values: list[str]) -> list[str]:
        seen: set[str] = set()
        duplicates: set[str] = set()
        for value in values:
            if value in seen:
                duplicates.add(value)
            seen.add(value)
        return [f"duplicate {label}: {value}" for value in sorted(duplicates)]

    @staticmethod
    def priority_gap_failures(priorities: list[str]) -> list[str]:
        expected = [f"SB-{index:03}" for index in range(1, len(priorities) + 1)]
        if sorted(priorities) == expected:
            return []
        return ["storybook menu priority order must be contiguous from SB-001"]

    @staticmethod
    def missing_presets(required: list[str], preset_counts: dict[str, int]) -> list[str]:
        return [f"{page}: missing explicit Storybook preset labels" for page in required if page not in preset_counts]

    @staticmethod
    def missing_options(required: list[str], option_pages: dict[str, str]) -> list[str]:
        return [f"{page}: missing Storybook UI option contract" for page in required if page not in option_pages]

    @staticmethod
    def low_option_counts(
        required: list[str],
        option_pages: dict[str, str],
        option_counts: dict[str, int],
    ) -> list[str]:
        failures = []
        for page in required:
            target = option_pages.get(page)
            if target is not None and option_counts.get(target, 0) < MIN_OPTION_COUNT:
                failures.append(f"{page}: Storybook option contract must cover at least {MIN_OPTION_COUNT} options")
        return failures

    @staticmethod
    def low_preset_counts(required: list[str], preset_counts: dict[str, int]) -> list[str]:
        return [
            f"{page}: Storybook presets must expose at least {MIN_PRESET_COUNT} tabs"
            for page in required
            if preset_counts.get(page, 0) < MIN_PRESET_COUNT
        ]

    def text_input_runtime_state_failures(self) -> list[str]:
        screen_state = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/screen_state.rs"
        )
        runtime_state = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/text_input_screen_state.rs"
        )
        interaction_state = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/screen_state_text_input.rs"
        )
        if not screen_state and not runtime_state and not interaction_state:
            return []
        failures: list[str] = []
        forbidden = (
            "text_input_state:",
            "text_input_uses_live_value:",
            "text_input_caret_visible:",
        )
        for token in forbidden:
            if token in screen_state:
                failures.append(f"text-input Storybook state must be instance-scoped, not `{token}`")
        required = (
            "TextInputStateStore",
            "BTreeMap<&'static str, TextInputRuntimeState>",
        )
        for token in required:
            if token not in runtime_state:
                failures.append(f"text-input Storybook runtime state missing instance store token: {token}")
        if "register_text_input_readonly_block" not in interaction_state:
            failures.append("text-input Storybook keyboard path must block readonly mutation")
        return failures

    def preset_tab_label_layout_failures(self) -> list[str]:
        source = self.read_optional("crates/katana-ui-core-storybook/src/visual/preset_tabs.rs")
        if not source:
            return []
        required = ("measure_width", "with_clip", "tab_label_widths_for_test")
        return [
            f"preset tab labels must be measured and clipped inside each tab: missing {token}"
            for token in required
            if token not in source
        ]

    def read_optional(self, relative: str) -> str:
        path = self.root / relative
        if not path.exists():
            return ""
        return path.read_text(encoding="utf-8")
