from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class SpecializedPublicOptionRequirements:
    CLOSEABLE_TAB_BAR_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/bar.rs"
    )
    CLOSEABLE_TAB_OPTIONS_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs"
    )
    COLOR_PICKER_SOURCE = "crates/katana-ui-core/src/molecule/color/picker.rs"
    HOVER_CARD_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/hover_card.rs"
    MOTION_SOURCE = "crates/katana-ui-core/src/interaction/motion_tokens.rs"
    THEME_SOURCE = "crates/katana-ui-core/src/theme/mod.rs"
    THEME_PRESET_SOURCE = "crates/katana-ui-core/src/theme/preset.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.closeable_tab_strip(),
            *cls.color_picker_rgba(),
            *cls.hover_card(),
            *cls.motion(),
            *cls.theme_tokens(),
        )

    @classmethod
    def closeable_tab_strip(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.CLOSEABLE_TAB_BAR_SOURCE, "pub fn active_tab_id", "active_tab_id"),
            (cls.CLOSEABLE_TAB_OPTIONS_SOURCE, "pub fn pinned", "tabs.pin"),
            (cls.CLOSEABLE_TAB_OPTIONS_SOURCE, "pub fn group_id", "tabs.group"),
            (
                cls.CLOSEABLE_TAB_OPTIONS_SOURCE,
                "pub overflow_trigger_width:",
                "tabs.overflow",
            ),
        )
        return cls.for_page_sources("closeable-tab-strip", mappings)

    @classmethod
    def color_picker_rgba(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn rgba", "color_picker.rgba"),
            ("pub fn value", "color_picker.value"),
            ("pub fn open", "color_picker.open"),
            ("pub fn hue", "color_picker.hue"),
            ("pub fn alpha", "color_picker.alpha"),
            ("pub fn blending", "color_picker.blending"),
            ("pub fn color_area", "color_picker.color_area"),
            ("pub fn trigger_size", "color_picker.trigger_size"),
            ("pub fn title", "color_picker.title"),
            ("pub fn rgba_mode", "color_picker.rgba_mode"),
            ("pub fn panel_scale_percent", "color_picker.panel_scale_percent"),
            ("pub fn trigger_border", "color_picker.trigger_border"),
            ("pub fn eyedropper_callback", "color_picker.eyedropper_callback"),
            ("pub fn readonly", "color_picker.readonly"),
            ("pub fn disabled", "color_picker.disabled"),
        )
        return cls.for_page("color-picker-rgba", cls.COLOR_PICKER_SOURCE, mappings)

    @classmethod
    def hover_card(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn open_delay_ms", "hover_card.open_delay_ms"),
            ("pub fn close_delay_ms", "hover_card.close_delay_ms"),
            ("pub fn pointer_follow", "hover_card.pointer_follow"),
            ("pub fn slot_action", "hover_card.slot_action"),
        )
        return cls.for_page("hover-card", cls.HOVER_CARD_SOURCE, mappings)

    @classmethod
    def motion(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub primitive:", "motion.primitive"),
            ("pub duration:", "motion.duration"),
            ("pub distance:", "motion.distance"),
            ("pub policy:", "motion.reduced_policy"),
        )
        return cls.for_page("motion", cls.MOTION_SOURCE, mappings)

    @classmethod
    def theme_tokens(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.THEME_SOURCE, "pub id:", "theme.id"),
            (cls.THEME_PRESET_SOURCE, 'color_token("background"', "color.background"),
            (cls.THEME_PRESET_SOURCE, 'color_token("surface"', "color.surface"),
            (cls.THEME_PRESET_SOURCE, 'color_token("accent"', "color.accent"),
        )
        return cls.for_page_sources("theme-tokens", mappings)

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

    @staticmethod
    def for_page_sources(
        page: str,
        mappings: tuple[tuple[str, str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for source, token, setting in mappings
        )
