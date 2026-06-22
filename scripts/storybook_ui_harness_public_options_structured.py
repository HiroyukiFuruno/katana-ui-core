from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class StructuredPublicOptionRequirements:
    COLLAPSIBLE_PANEL_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/collapsible_panel/mod.rs"
    )
    COLLAPSIBLE_PANEL_TYPES_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/collapsible_panel/types.rs"
    )
    STRUCTURED_MODEL_SOURCE = "crates/katana-ui-core/src/molecule/structured/model.rs"
    STRUCTURED_OPTIONS_SOURCE = "crates/katana-ui-core/src/molecule/structured/options.rs"
    COMMAND_ROW_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/command_launcher_results/row.rs"
    )
    STRUCTURED_ACTIONS_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/options_actions.rs"
    )
    STRUCTURED_EXTRA_SOURCE = "crates/katana-ui-core/src/molecule/structured/options_extra.rs"
    STRUCTURED_ITEMS_SOURCE = "crates/katana-ui-core/src/molecule/structured/items.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.collapsible_panel(),
            *cls.command_palette(),
            *cls.dynamic_array_editor(),
            *cls.tree_view(),
        )

    @classmethod
    def collapsible_panel(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.COLLAPSIBLE_PANEL_SOURCE, "pub fn mode", "collapsible_panel.mode"),
            (cls.COLLAPSIBLE_PANEL_TYPES_SOURCE, "pub fn new", "collapsible_panel.width"),
            (cls.COLLAPSIBLE_PANEL_SOURCE, "pub fn pinned", "collapsible_panel.pinned"),
            (
                cls.COLLAPSIBLE_PANEL_SOURCE,
                "pub fn expand_on_hover",
                "collapsible_panel.expand_on_hover",
            ),
            (
                cls.COLLAPSIBLE_PANEL_SOURCE,
                "pub fn resize_handle",
                "collapsible_panel.resize_handle",
            ),
        )
        return tuple(
            PublicOptionRequirement("collapsible-panel", source, token, setting)
            for source, token, setting in mappings
        )

    @classmethod
    def command_palette(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.STRUCTURED_EXTRA_SOURCE, "pub fn query", "command_palette.query"),
            (cls.STRUCTURED_MODEL_SOURCE, "pub fn selected_index", "command_palette.highlight"),
            (cls.STRUCTURED_MODEL_SOURCE, "pub fn item_count", "command_palette.row_count"),
            (
                cls.COMMAND_ROW_SOURCE,
                "pub fn provider_id",
                "command_palette.provider_group",
            ),
            (
                cls.STRUCTURED_ITEMS_SOURCE,
                "pub fn shortcut",
                "command_palette.shortcut_display",
            ),
        )
        return tuple(
            PublicOptionRequirement("command-palette", source, token, setting)
            for source, token, setting in mappings
        )

    @classmethod
    def dynamic_array_editor(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.STRUCTURED_MODEL_SOURCE, "pub fn item", "array.rows"),
            (cls.STRUCTURED_ACTIONS_SOURCE, "pub fn add_action", "array.add_remove"),
            (cls.STRUCTURED_ACTIONS_SOURCE, "pub fn delete_action", "array.add_remove"),
            (cls.STRUCTURED_ACTIONS_SOURCE, "pub fn reorder_action", "array.reorder"),
            (cls.STRUCTURED_EXTRA_SOURCE, "pub fn tree_theme_id", "array.theme_row"),
        )
        return tuple(
            PublicOptionRequirement("dynamic-array-editor", source, token, setting)
            for source, token, setting in mappings
        )

    @classmethod
    def tree_view(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.STRUCTURED_EXTRA_SOURCE, "pub fn empty_area_context_menu", "context_menu"),
            (cls.STRUCTURED_EXTRA_SOURCE, "pub fn toggle_trigger_area", "trigger"),
            (cls.STRUCTURED_ITEMS_SOURCE, "pub fn directory", "node_marker"),
            (cls.STRUCTURED_OPTIONS_SOURCE, "pub fn line_display", "line"),
        )
        return tuple(
            PublicOptionRequirement("tree-view", source, token, setting)
            for source, token, setting in mappings
        )
