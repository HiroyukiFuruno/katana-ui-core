from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class CompositePublicOptionRequirements:
    BASIC_LIST_SOURCE = "crates/katana-ui-core/src/molecule/basic_list.rs"
    BASIC_SOURCE = "crates/katana-ui-core/src/molecule/basic.rs"
    CARD_SOURCE = "crates/katana-ui-core/src/molecule/card.rs"
    SELECTION_CHOICE_SOURCE = "crates/katana-ui-core/src/molecule/selection/choice.rs"
    SELECTION_OPTIONS_SOURCE = "crates/katana-ui-core/src/molecule/selection/options.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.breadcrumb_and_side_menu(),
            *cls.menu(),
            *cls.form_field(),
            *cls.list(),
            *cls.card(),
            *cls.combo_box(),
        )

    @classmethod
    def breadcrumb_and_side_menu(cls) -> tuple[PublicOptionRequirement, ...]:
        breadcrumb = (
            (cls.SELECTION_CHOICE_SOURCE, "pub fn item", "breadcrumb.items"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn child", "children"),
            (
                cls.SELECTION_CHOICE_SOURCE,
                "pub fn selected_index",
                "interaction.selected_index",
            ),
            (
                cls.SELECTION_CHOICE_SOURCE,
                "pub fn crumb_action",
                "breadcrumb.crumb_action",
            ),
        )
        side_menu = (
            (cls.SELECTION_CHOICE_SOURCE, "pub fn item", "side_menu.items"),
            (cls.SELECTION_CHOICE_SOURCE, "pub fn child", "children"),
            (
                cls.SELECTION_CHOICE_SOURCE,
                "pub fn selected_index",
                "interaction.selected_index",
            ),
            (
                cls.SELECTION_OPTIONS_SOURCE,
                "pub fn hover_expansion",
                "side_menu.hover_expansion",
            ),
        )
        return (
            *tuple(
                PublicOptionRequirement("breadcrumb", source, token, setting)
                for source, token, setting in breadcrumb
            ),
            *tuple(
                PublicOptionRequirement("side-menu", source, token, setting)
                for source, token, setting in side_menu
            ),
        )

    @classmethod
    def menu(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn common", "menu.common_props"),
            ("pub fn child", "children"),
            ("pub fn selected_index", "interaction.selected_index"),
            ("pub fn resolve_panel_placement", "menu.panel_placement"),
        )
        return cls.for_page("menu", cls.BASIC_SOURCE, mappings)

    @classmethod
    def form_field(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn common", "form_field.common_props"),
            ("pub fn child", "children"),
            ("pub fn invalid", "form_field.invalid"),
            ("pub fn helper_text", "form_field.helper_text"),
        )
        return cls.for_page("form-field", cls.BASIC_SOURCE, mappings)

    @classmethod
    def list(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn child", "list.rows"),
            ("pub fn selected_index", "list.selection"),
            ("pub fn empty_state", "list.empty_state"),
            ("pub fn virtualization", "list.virtualization"),
            ("pub fn row_theme_slot", "list.theme_row"),
        )
        return cls.for_page("list", cls.BASIC_LIST_SOURCE, mappings)

    @classmethod
    def card(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn new", "card.label"),
            ("pub fn header", "card.header"),
            ("pub fn footer", "card.footer"),
            ("pub fn variant", "card.variant"),
            ("pub fn padding", "card.padding"),
            ("pub fn interactive", "card.clickable"),
            ("pub fn child", "card.nested_controls"),
            ("pub fn child", "card.child_state"),
        )
        return cls.for_page("card", cls.CARD_SOURCE, mappings)

    @classmethod
    def combo_box(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn item", "combo.items"),
            ("pub fn item_count", "combo.items"),
            ("pub fn open", "interaction.open"),
            ("pub fn selected_index", "interaction.selected_index"),
            ("pub fn value", "interaction.value"),
            ("pub fn placeholder", "placeholder"),
            ("pub fn disabled", "disabled"),
            ("pub fn readonly", "readonly"),
            ("pub fn input_value", "combo.input_value"),
            ("pub fn filter_result", "combo.filter_result"),
            ("pub fn free_input", "combo.free_input"),
            ("pub fn keyboard_navigation", "combo.keyboard_navigation"),
            ("pub fn placement", "combo.placement"),
            ("pub fn highlighted_index", "combo.highlighted_index"),
            ("pub fn long_list", "combo.long_list"),
            ("pub fn outside_click_dismiss", "combo.outside_click_dismiss"),
            ("pub fn framed", "combo.framed"),
            ("pub fn trigger_summary", "combo.trigger_summary"),
            ("pub fn select_action", "combo.select_action"),
        )
        return cls.for_page("combo-box", cls.SELECTION_CHOICE_SOURCE, mappings)

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
    def for_pages_same_source(
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
