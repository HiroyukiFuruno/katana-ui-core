from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class RuntimePublicOptionRequirements:
    BANNER_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/banner.rs"
    CODE_DIFF_ACCESSORS_SOURCE = "crates/katana-ui-core/src/molecule/diff/accessors.rs"
    CODE_DIFF_MODEL_SOURCE = "crates/katana-ui-core/src/molecule/diff/model.rs"
    DISCLOSURE_MODEL_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/model.rs"
    DISCLOSURE_OPTIONS_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/options.rs"
    PANEL_SOURCE = "crates/katana-ui-core/src/panel/mod.rs"
    STARTUP_ACTIONS_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/startup_state_panel/actions.rs"
    )
    STARTUP_MODEL_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/startup_state_panel/mod.rs"
    )
    STARTUP_STATE_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/startup_state_panel/state.rs"
    )
    TOAST_EVENTS_SOURCE = "crates/katana-ui-core/src/molecule/toast_stack_manager/events.rs"
    TOAST_DISCLOSURE_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/toast.rs"
    TOAST_TYPES_SOURCE = "crates/katana-ui-core/src/molecule/toast_stack_manager/types.rs"
    VIRTUALIZATION_SOURCE = "crates/katana-ui-core/src/interaction/virtualization.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.banner(),
            *cls.notification_toast(),
            *cls.toast_stack_manager(),
            *cls.startup_state_panel(),
            *cls.code_diff(),
            *cls.panel(),
            *cls.virtualization(),
        )

    @classmethod
    def banner(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn severity", "severity"),
            ("pub fn density", "density"),
            ("pub fn action", "action"),
            ("pub fn dismissible", "dismiss"),
            ("pub fn expanded_details", "banner.details"),
            ("pub fn title", "banner.title"),
            ("pub fn leading_icon", "banner.leading_icon"),
            ("pub fn placement_hint", "banner.placement"),
        )
        return cls.for_page("banner", cls.BANNER_SOURCE, mappings)

    @classmethod
    def notification_toast(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.TOAST_DISCLOSURE_SOURCE, "pub fn severity", "severity"),
            (cls.DISCLOSURE_OPTIONS_SOURCE, "pub fn timer_summary", "duration"),
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn child", "action"),
            (cls.TOAST_DISCLOSURE_SOURCE, "pub fn dismiss_action", "dismiss"),
        )
        return cls.for_page_sources("notification-toast", mappings)

    @classmethod
    def toast_stack_manager(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.TOAST_TYPES_SOURCE, "pub fn severity", "severity"),
            (cls.TOAST_TYPES_SOURCE, "pub fn duration_ms", "duration"),
            (cls.TOAST_TYPES_SOURCE, "pub fn action", "action"),
            (cls.TOAST_EVENTS_SOURCE, "Dismiss(", "dismiss"),
        )
        return cls.for_page_sources("toast-stack-manager", mappings)

    @classmethod
    def startup_state_panel(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.STARTUP_MODEL_SOURCE, "pub fn state", "startup_state.state"),
            (cls.STARTUP_MODEL_SOURCE, "pub fn live_region_label", "startup_state.label"),
            (cls.STARTUP_STATE_SOURCE, "pub fn loading", "startup_state.progress"),
            (cls.STARTUP_STATE_SOURCE, "retry:", "startup_state.retry"),
            (cls.STARTUP_STATE_SOURCE, "cancel:", "startup_state.cancel"),
        )
        return cls.for_page_sources("startup-state-panel", mappings)

    @classmethod
    def code_diff(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.CODE_DIFF_MODEL_SOURCE, "pub fn mode", "code_diff.mode"),
            (cls.CODE_DIFF_MODEL_SOURCE, "pub fn whitespace", "code_diff.whitespace"),
            (cls.CODE_DIFF_MODEL_SOURCE, "pub fn direction", "code_diff.direction"),
            (cls.CODE_DIFF_MODEL_SOURCE, "pub fn item_count", "code_diff.item_count"),
            (cls.CODE_DIFF_MODEL_SOURCE, "pub fn source_texts", "code_diff.context_lines"),
            (cls.CODE_DIFF_ACCESSORS_SOURCE, "pub fn scroll_sync_enabled", "code_diff.scroll_sync"),
            (cls.CODE_DIFF_MODEL_SOURCE, "pub fn language", "code_diff.language"),
        )
        return cls.for_page_sources("code-diff", mappings)

    @classmethod
    def panel(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn active_panel", "active_panel"),
            ("pub fn vertical_scroll", "vertical_scroll"),
            ("pub fn horizontal_scroll", "horizontal_scroll"),
            ("pub fn scrollbar", "scrollbar_visibility"),
            ("pub fn horizontal_scrollbar", "scrollbar_visibility"),
            ("pub fn child", "nested_state"),
        )
        return cls.for_page("panel", cls.PANEL_SOURCE, mappings)

    @classmethod
    def virtualization(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub viewport_offset:", "viewport.offset"),
            ("pub overscan:", "virtualization.overscan"),
            ("pub row_height_provider:", "virtualization.row_height_provider"),
            ("pub focused_index:", "virtualization.focused_index"),
            ("pub fn correct_scroll_offset_after_measurement", "virtualization.measured_correction"),
        )
        return cls.for_page("virtualization", cls.VIRTUALIZATION_SOURCE, mappings)

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
