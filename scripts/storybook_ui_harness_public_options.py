from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement
from storybook_ui_harness_public_options_additional import AdditionalPublicOptionRequirements
from storybook_ui_harness_public_options_atoms import AtomPublicOptionRequirements
from storybook_ui_harness_public_options_layout import LayoutPublicOptionRequirements
from storybook_ui_harness_public_options_specialized import (
    SpecializedPublicOptionRequirements,
)


class PublicOptionRequirements:
    ACTION_BUILDERS_SOURCE = "crates/katana-ui-core/src/interaction/action_builders.rs"
    ATOM_SOURCE = "crates/katana-ui-core/src/atom/mod.rs"
    ATOM_OPTIONS_SOURCE = "crates/katana-ui-core/src/atom/options.rs"
    COMMON_PROPS_SOURCE = "crates/katana-ui-core/src/render_model/common.rs"
    INPUT_SOURCE = "crates/katana-ui-core/src/atom/typed.rs"
    TEXT_AREA_BUILDER_SOURCE = "crates/katana-ui-core/src/atom/text_area/builders.rs"
    TEXT_AREA_SOURCE = "crates/katana-ui-core/src/atom/text_area/options.rs"
    SELECTION_CHOICE_SOURCE = "crates/katana-ui-core/src/molecule/selection/choice.rs"
    SELECTION_SOURCE = "crates/katana-ui-core/src/molecule/selection/options.rs"
    TABS_SOURCE = "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs"
    TABS_ACTION_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/actions.rs"
    )
    TABS_SCROLL_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/scroll.rs"
    )

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *AtomPublicOptionRequirements.all(),
            *cls.button_family_common(),
            *cls.selection_family_common(),
            *cls.text_input(),
            *cls.text_area(),
            *cls.workspace_tabs(),
            *AdditionalPublicOptionRequirements.all(),
            *LayoutPublicOptionRequirements.all(),
            *SpecializedPublicOptionRequirements.all(),
        )

    @classmethod
    def button_family_common(cls) -> tuple[PublicOptionRequirement, ...]:
        pages = ("button", "text-button", "svg-button", "icon-text-button")
        mappings = (
            (cls.ATOM_SOURCE, "pub fn new", "label"),
            (cls.ATOM_SOURCE, "pub fn visible", "visible"),
            (cls.ATOM_SOURCE, "pub fn disabled", "disabled"),
            (cls.ATOM_SOURCE, "pub fn focusable", "focusable"),
            (cls.ATOM_SOURCE, "pub fn width", "width"),
            (cls.ATOM_SOURCE, "pub fn height", "height"),
            (cls.COMMON_PROPS_SOURCE, "pub fn border", "border"),
            (cls.ATOM_SOURCE, "pub fn tab_index", "tab-index"),
            (cls.ATOM_SOURCE, "pub fn z_index", "z-index"),
            (cls.ATOM_OPTIONS_SOURCE, "pub fn command", "button.command"),
            (
                cls.ATOM_OPTIONS_SOURCE,
                "pub fn keyboard_activation",
                "button.keyboard_activation",
            ),
            (cls.ATOM_OPTIONS_SOURCE, "pub fn icon_position", "button.icon_position"),
            (cls.ATOM_OPTIONS_SOURCE, "pub fn layout_preset", "button.layout_preset"),
        )
        return cls.for_pages(pages, mappings)

    @classmethod
    def selection_family_common(cls) -> tuple[PublicOptionRequirement, ...]:
        binary_pages = ("checkbox", "radio", "toggle", "segmented-toggle")
        binary_mappings = (
            (cls.ATOM_SOURCE, "pub fn selected", "selected"),
            (cls.ATOM_SOURCE, "pub fn checked", "checked"),
            (cls.ATOM_SOURCE, "pub fn disabled", "disabled"),
            (cls.ACTION_BUILDERS_SOURCE, "pub fn focus", "focus"),
        )
        return (
            *cls.for_pages(binary_pages, binary_mappings),
            *cls.select_box(),
            *cls.selection_list(),
            *cls.menu_button(),
        )

    @classmethod
    def select_box(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.SELECTION_CHOICE_SOURCE, "pub fn item", "select.items"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn open", "interaction.open"),
            (
                cls.SELECTION_CHOICE_SOURCE,
                "pub fn selected_index",
                "interaction.selected_index",
            ),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn placeholder", "placeholder"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn disabled", "disabled"),
        )
        return cls.for_page("select-box", mappings)

    @classmethod
    def selection_list(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.SELECTION_CHOICE_SOURCE, "pub fn item", "selection_list.items"),
            (
                cls.SELECTION_CHOICE_SOURCE,
                "pub fn selected_index",
                "interaction.selected_index",
            ),
            (cls.SELECTION_SOURCE, "pub fn section", "selection_list.section"),
            (cls.SELECTION_SOURCE, "pub fn marker", "selection_list.marker"),
            (cls.SELECTION_SOURCE, "pub fn more_row", "selection_list.more_row"),
        )
        return cls.for_page("selection-list", mappings)

    @classmethod
    def menu_button(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.SELECTION_CHOICE_SOURCE, "pub fn item", "menu.items"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn open", "interaction.open"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn disabled", "disabled"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn select_action", "menu.select_action"),
        )
        return cls.for_page("menu-button", mappings)

    @classmethod
    def text_input(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            PublicOptionRequirement(
                "text-input", cls.ATOM_SOURCE, "pub fn value", "interaction.value"
            ),
            PublicOptionRequirement("text-input", cls.ATOM_SOURCE, "pub fn disabled", "disabled"),
            PublicOptionRequirement(
                "text-input", cls.ATOM_SOURCE, "pub fn font_role", "font_role"
            ),
            PublicOptionRequirement("text-input", cls.ATOM_SOURCE, "pub fn readonly", "readonly"),
            PublicOptionRequirement("text-input", cls.ATOM_SOURCE, "pub fn invalid", "validation"),
            PublicOptionRequirement(
                "text-input", cls.ATOM_SOURCE, "pub fn placeholder", "placeholder"
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn leading_slot",
                "text_entry.leading_slot_reserved",
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn leading_icon_slot",
                "text_entry.leading_slot.icon",
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn trailing_slot",
                "text_entry.trailing_slot_reserved",
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn trailing_icon_button",
                "text_entry.trailing_icon_buttons",
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn clear_action",
                "text_entry.clear_action",
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn input_background_token",
                "theme.input_bg",
            ),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn submit_on_enter",
                "text_entry.submit_on_enter",
            ),
            PublicOptionRequirement("text-input", cls.INPUT_SOURCE, "pub fn ime_enabled", "ime"),
            PublicOptionRequirement(
                "text-input",
                cls.INPUT_SOURCE,
                "pub fn emoji_enabled",
                "text_entry.emoji_enabled",
            ),
        )

    @classmethod
    def text_area(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub value:", "text_area.value"),
            ("pub placeholder:", "text_area.placeholder"),
            ("pub font_role:", "text_area.font_role"),
            ("pub disabled:", "text_area.disabled"),
            ("pub readonly:", "text_area.readonly"),
            ("pub invalid:", "text_area.invalid"),
            ("pub min_rows:", "text_area.min_rows"),
            ("pub max_rows:", "text_area.max_rows"),
            ("pub auto_grow:", "text_area.auto_grow"),
            ("pub wrap_policy:", "text_area.wrap_policy"),
            ("pub submit_key:", "text_area.submit_key"),
            ("pub newline_key:", "text_area.newline_key"),
            ("pub tab_behavior:", "text_area.tab_behavior"),
            ("pub ime_enabled:", "text_area.ime_enabled"),
            ("pub resize_enabled:", "text_area.resize_enabled"),
            ("pub vertical_scroll_enabled:", "text_area.vertical_scroll_enabled"),
            ("pub horizontal_scroll_enabled:", "text_area.horizontal_scroll_enabled"),
            ("pub vertical_scrollbar_visible:", "text_area.vertical_scrollbar_visible"),
            ("pub horizontal_scrollbar_visible:", "text_area.horizontal_scrollbar_visible"),
            ("pub leading_slot:", "text_area.leading_slot_reserved"),
            ("pub trailing_slot:", "text_area.trailing_slot_reserved"),
            ("pub trailing_icon_buttons:", "text_area.trailing_icon_buttons"),
            ("pub clear_action:", "text_area.clear_action"),
        )
        requirements = tuple(
            PublicOptionRequirement("text-area", cls.TEXT_AREA_SOURCE, token, setting)
            for token, setting in mappings
        )
        return (
            *requirements,
            PublicOptionRequirement(
                "text-area",
                cls.TEXT_AREA_BUILDER_SOURCE,
                "pub fn leading_icon_slot",
                "text_area.leading_slot.icon",
            ),
        )

    @classmethod
    def workspace_tabs(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.TABS_ACTION_SOURCE, "AddTab", "tabs.add"),
            (cls.TABS_ACTION_SOURCE, "CloseTab", "tabs.close"),
            (cls.TABS_ACTION_SOURCE, "MoveTab", "tabs.move"),
            (cls.TABS_ACTION_SOURCE, "OpenOverflow", "tabs.overflow"),
            (cls.TABS_SCROLL_SOURCE, "pub fn follow_active", "tabs.active_scroll"),
            (cls.TABS_SOURCE, "pub icon:", "tabs.icon"),
            (cls.TABS_SOURCE, "pub dirty:", "tabs.dirty"),
            (cls.TABS_SOURCE, "pub pinned:", "tabs.pin"),
            (cls.TABS_SOURCE, "pub closeable:", "tabs.closeable"),
            (cls.TABS_SOURCE, "pub tone:", "tabs.tone"),
            (cls.TABS_SOURCE, "pub tooltip:", "tabs.tooltip"),
            (cls.TABS_SOURCE, "pub group_id:", "tabs.group"),
            (cls.TABS_SOURCE, "pub accessibility_label:", "tabs.accessibility_label"),
            (cls.TABS_SOURCE, "pub color:", "tabs.group_color"),
            (cls.TABS_SOURCE, "pub collapsed:", "tabs.group_collapsed"),
            (cls.TABS_SOURCE, "pub overflow_trigger_width:", "tabs.overflow_width"),
            (
                cls.TABS_SOURCE,
                "pub collapsed_group_auto_expand_ms:",
                "tabs.group_auto_expand",
            ),
        )
        return tuple(
            PublicOptionRequirement("tabs", source, token, setting)
            for source, token, setting in mappings
        )

    @staticmethod
    def for_pages(
        pages: tuple[str, ...],
        mappings: tuple[tuple[str, str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for page in pages
            for source, token, setting in mappings
        )

    @staticmethod
    def for_page(
        page: str,
        mappings: tuple[tuple[str, str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for source, token, setting in mappings
        )
