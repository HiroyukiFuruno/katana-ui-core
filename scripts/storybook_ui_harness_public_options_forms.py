from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class FormPublicOptionRequirements:
    COMBO_BOX_SOURCE = "crates/katana-ui-core/src/molecule/selection/choice.rs"
    SEARCH_BOX_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/search_box.rs"
    SEARCH_CONTROL_STRIP_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/search_control_strip/mod.rs"
    )

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.combo_box(),
            *cls.search_box(),
            *cls.search_control_strip(),
        )

    @classmethod
    def combo_box(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (("pub fn invalid", "validation"),)
        return cls.for_page("combo-box", cls.COMBO_BOX_SOURCE, mappings)

    @classmethod
    def search_box(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn value", "text_entry.value"),
            ("pub fn submit_on_enter", "text_entry.submit_on_enter"),
            ("pub fn clear_action", "text_entry.clear_button"),
            ("pub fn case_sensitive", "text_entry.regex_case"),
        )
        return cls.for_page("search-box", cls.SEARCH_BOX_SOURCE, mappings)

    @classmethod
    def search_control_strip(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn query", "search_control.query"),
            ("pub fn options", "search_control.match_case"),
            ("pub fn options", "search_control.whole_word"),
            ("pub fn options", "search_control.use_regex"),
            ("pub fn replace_mode", "search_control.replace_mode"),
            ("pub fn result_position", "search_control.result_count"),
            ("pub fn result_position", "search_control.active_index"),
        )
        return cls.for_page("search-control-strip", cls.SEARCH_CONTROL_STRIP_SOURCE, mappings)

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
