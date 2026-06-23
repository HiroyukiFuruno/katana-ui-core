from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class LayoutPublicOptionRequirements:
    LAYOUT_SOURCE = "crates/katana-ui-core/src/layout/containers.rs"
    SCROLL_AREA_SOURCE = "crates/katana-ui-core/src/layout/scroll_area.rs"
    SPLIT_PANE_SOURCE = "crates/katana-ui-core/src/layout/split_pane.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.layout_models(),
            *cls.scroll_area(),
            *cls.split_pane(),
        )

    @classmethod
    def layout_models(cls) -> tuple[PublicOptionRequirement, ...]:
        pages = ("row", "column", "stack", "grid", "align-center")
        mappings = (
            ("pub fn axis", "axis"),
            ("pub fn gap", "gap"),
            ("pub fn overflow", "overflow"),
            ("pub fn align", "alignment"),
        )
        return cls.for_pages(pages, cls.LAYOUT_SOURCE, mappings)

    @classmethod
    def scroll_area(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn axis", "axis"),
            ("pub fn scrollbar_visibility", "overflow"),
            ("pub fn gap", "gap"),
            ("pub fn align", "alignment"),
        )
        return cls.for_page("scroll-area", cls.SCROLL_AREA_SOURCE, mappings)

    @classmethod
    def split_pane(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn axis", "axis"),
            ("pub fn gap", "gap"),
            ("pub fn align", "alignment"),
            ("pub fn overflow", "overflow"),
            ("pub fn ratio_percent", "split_pane.ratio_percent"),
            ("pub fn min_percent", "split_pane.min_percent"),
            ("pub fn max_percent", "split_pane.max_percent"),
            ("pub fn reset_percent", "split_pane.reset_percent"),
            ("pub fn handle_width_px", "split_pane.handle_width_px"),
            ("pub fn resize_mode", "split_pane.resize_mode"),
        )
        return cls.for_page("split-pane", cls.SPLIT_PANE_SOURCE, mappings)

    @staticmethod
    def for_page(
        page: str,
        source: str,
        mappings: tuple[tuple[str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for token, setting in mappings
        )

    @classmethod
    def for_pages(
        cls,
        pages: tuple[str, ...],
        source: str,
        mappings: tuple[tuple[str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            requirement
            for page in pages
            for requirement in cls.for_page(page, source, mappings)
        )
