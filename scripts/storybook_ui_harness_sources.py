from __future__ import annotations

import re
from pathlib import Path

PAGE_TOKEN = re.compile(r'"([a-z0-9-]+)"')
LABEL_TOKEN = re.compile(r'"([^"]+)"')
STORY_PATH_PAGE = re.compile(r'page:\s*"([a-z0-9-]+)"')
LEAF_CHANGE_ROW = re.compile(
    r"\|\s*[^|]+\|\s*`([a-z0-9-]+)`\s*\|\s*`(storybook-page-[a-z0-9-]+)`\s*\|"
)
LEAF_CHANGE_STATUS_ROW = re.compile(
    r"\|\s*[^|]+\|\s*`([a-z0-9-]+)`\s*\|\s*`(storybook-page-[a-z0-9-]+)`\s*\|[^|]*\|\s*([^|]+)\|"
)
PRIORITY_ROW = re.compile(
    r"\|\s*(SB-\d{3})\s*\|\s*`([a-z0-9-]+)`\s*\|\s*`(storybook-page-[a-z0-9-]+)`\s*\|"
)
SPLIT_SUMMARY_COUNT = re.compile(r"`draw_page` page 別描画(あり|未作成):\s*(\d+)")
DEDICATED_PAGE_ARM = re.compile(r'((?:"[a-z0-9-]+"(?:\s*\|\s*)?)+)\s*=>\s*')
OPTION_ARM = re.compile(
    r'((?:"[a-z0-9-]+"(?:\s*\|\s*)?)+)\s*=>\s*(?:&([A-Z_]+)|\{\s*&([A-Z_]+)\s*\})',
    re.S,
)
PRESET_ARM = re.compile(
    r'((?:"[a-z0-9-]+"(?:\s*\|\s*)?)+)\s*=>\s*(?:&\[(.*?)\]|\{\s*&\[(.*?)\]\s*\})',
    re.S,
)
OPTION_ARRAY = re.compile(
    r"const\s+([A-Z_]+):\s+\[StorybookUiOptionContract;\s+\d+\]\s*=\s*\[(.*?)\];",
    re.S,
)
OPTION_SETTING = re.compile(r'StorybookUiOptionContract::new\(\s*"([^"]+)"')


class StorybookUiHarnessSources:
    def __init__(self, root: Path) -> None:
        self.root = root

    def required_pages(self) -> list[str]:
        source = self.read("crates/katana-ui-core-storybook/src/requirements.rs")
        return PAGE_TOKEN.findall(source.split("const MIN_SINGLE_NODE", 1)[0])

    def preset_counts(self) -> dict[str, int]:
        counts: dict[str, int] = {}
        for source in self.preset_sources():
            for arm in PRESET_ARM.finditer(source):
                labels = LABEL_TOKEN.findall(arm.group(2) or arm.group(3))
                for page in PAGE_TOKEN.findall(arm.group(1)):
                    counts[page] = len(labels)
        return counts

    def preset_sources(self) -> list[str]:
        paths = [
            "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
            "crates/katana-ui-core-storybook/src/catalog/preset_label_extra.rs",
            "crates/katana-ui-core-storybook/src/catalog/preset_label_extra_feedback.rs",
        ]
        return [
            self.read(relative)
            for relative in paths
            if (self.root / relative).exists()
        ]

    def menu_pages(self) -> list[str]:
        pages: list[str] = []
        catalog = self.root / "crates/katana-ui-core-storybook/src/catalog"
        for path in sorted(catalog.glob("story_paths_*.rs")):
            pages.extend(STORY_PATH_PAGE.findall(path.read_text(encoding="utf-8")))
        return pages

    def leaf_changes(self) -> dict[str, str]:
        path = (
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md"
        )
        if not path.exists():
            return {}
        return {
            page: change
            for page, change in LEAF_CHANGE_ROW.findall(path.read_text(encoding="utf-8"))
        }

    def leaf_statuses(self) -> dict[str, str]:
        path = (
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md"
        )
        if not path.exists():
            return {}
        return {
            page: status.strip()
            for page, _change, status in LEAF_CHANGE_STATUS_ROW.findall(
                path.read_text(encoding="utf-8")
            )
        }

    def split_summary_counts(self) -> dict[str, int]:
        path = (
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md"
        )
        if not path.exists():
            return {}
        return {
            label: int(count)
            for label, count in SPLIT_SUMMARY_COUNT.findall(path.read_text(encoding="utf-8"))
        }

    def dedicated_pages(self) -> set[str]:
        source = self.read("crates/katana-ui-core-storybook/src/visual/dedicated.rs")
        draw_page_source = source.split("\npub(super) fn draw(\n", 1)[0]
        pages: set[str] = set()
        for arm in DEDICATED_PAGE_ARM.finditer(draw_page_source):
            pages.update(PAGE_TOKEN.findall(arm.group(1)))
        return pages

    def priority_order(self) -> list[tuple[str, str, str]]:
        path = (
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md"
        )
        if not path.exists():
            return []
        return PRIORITY_ROW.findall(path.read_text(encoding="utf-8"))

    def option_pages(self) -> dict[str, str]:
        source = self.read(
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs"
        )
        pages: dict[str, str] = {}
        for arm in OPTION_ARM.finditer(source):
            target = arm.group(2) or arm.group(3)
            for page in PAGE_TOKEN.findall(arm.group(1)):
                pages[page] = target
        return pages

    def option_counts(self) -> dict[str, int]:
        counts: dict[str, int] = {}
        for source in self.option_sources():
            counts.update(
                {
                    name: body.count("StorybookUiOptionContract::new(")
                    for name, body in OPTION_ARRAY.findall(source)
                }
            )
        return counts

    def option_settings_by_page(self) -> dict[str, set[str]]:
        options_by_array = self.option_settings_by_array()
        return {
            page: options_by_array.get(array_name, set())
            for page, array_name in self.option_pages().items()
        }

    def option_settings_by_array(self) -> dict[str, set[str]]:
        settings: dict[str, set[str]] = {}
        for source in self.option_sources():
            settings.update(
                {
                    name: set(OPTION_SETTING.findall(body))
                    for name, body in OPTION_ARRAY.findall(source)
                }
            )
        return settings

    def option_sources(self) -> list[str]:
        paths = [
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs",
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_form_options.rs",
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_foundation_options.rs",
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_molecule_options.rs",
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_runtime_options.rs",
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_surface_options.rs",
            "crates/katana-ui-core-storybook/src/visual/storybook_ui_tabs_options.rs",
        ]
        return [self.read(relative) for relative in paths if (self.root / relative).exists()]

    def read(self, relative: str) -> str:
        return (self.root / relative).read_text(encoding="utf-8")
