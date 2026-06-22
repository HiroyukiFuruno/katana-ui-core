from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement
from storybook_ui_harness_public_options_composite import CompositePublicOptionRequirements
from storybook_ui_harness_public_options_disclosure import DisclosurePublicOptionRequirements
from storybook_ui_harness_public_options_forms import FormPublicOptionRequirements
from storybook_ui_harness_public_options_runtime import RuntimePublicOptionRequirements
from storybook_ui_harness_public_options_structured import StructuredPublicOptionRequirements


class MoleculePublicOptionRequirements:
    DISCLOSURE_MODEL_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/model.rs"
    DISCLOSURE_OPTIONS_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/options.rs"
    DIAGNOSTICS_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/diagnostics_list/options.rs"
    )
    DIAGNOSTICS_ACTIONS_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/diagnostics_list/actions.rs"
    )
    DIAGNOSTICS_TYPES_SOURCE = (
        "crates/katana-ui-core/src/molecule/structured/diagnostics_list/types.rs"
    )
    DRAG_SOURCE_SOURCE = "crates/katana-ui-core/src/interaction/drag_and_drop/drag_source.rs"
    DROP_TARGET_SOURCE = "crates/katana-ui-core/src/interaction/drag_and_drop/drop_target.rs"
    EMPTY_STATE_SOURCE = "crates/katana-ui-core/src/molecule/empty_state/mod.rs"
    SHORTCUT_CHEATSHEET_SOURCE = "crates/katana-ui-core/src/molecule/shortcut_cheatsheet.rs"
    SETTINGS_LIST_SOURCE = "crates/katana-ui-core/src/molecule/app_primitives/settings/mod.rs"
    SETTINGS_CONTROL_SOURCE = (
        "crates/katana-ui-core/src/molecule/app_primitives/settings/control.rs"
    )
    SETTINGS_FIELD_SOURCE = "crates/katana-ui-core/src/molecule/app_primitives/settings/field.rs"
    SKELETON_CLUSTER_SOURCE = "crates/katana-ui-core/src/molecule/skeleton_cluster.rs"
    STATUS_BAR_SOURCE = "crates/katana-ui-core/src/molecule/status_bar.rs"
    STATUS_BAR_MODEL_SOURCE = "crates/katana-ui-core/src/molecule/status_bar_parts/model.rs"
    TOOLBAR_ACTION_SOURCE = "crates/katana-ui-core/src/molecule/toolbar/action_model.rs"
    TOOLBAR_GROUP_SOURCE = "crates/katana-ui-core/src/molecule/toolbar/group_model.rs"
    TOOLBAR_SOURCE = "crates/katana-ui-core/src/molecule/toolbar/options.rs"
    TOOLBAR_SPLIT_SOURCE = "crates/katana-ui-core/src/molecule/toolbar/split_model.rs"
    CONTEXT_MENU_SOURCE = (
        "crates/katana-ui-core/src/molecule/selection/context_menu/options.rs"
    )
    WINDOW_CONTROL_SOURCE = (
        "crates/katana-ui-core/src/molecule/selection/window_control_button_group/options.rs"
    )

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.accordion(),
            *cls.context_menu(),
            *cls.diagnostics_list(),
            *cls.drag_and_drop(),
            *cls.empty_state(),
            *cls.settings_list(),
            *cls.shortcut_cheatsheet(),
            *cls.skeleton_cluster(),
            *cls.status_bar(),
            *cls.toolbar(),
            *cls.window_control_button_group(),
            *CompositePublicOptionRequirements.all(),
            *DisclosurePublicOptionRequirements.all(),
            *FormPublicOptionRequirements.all(),
            *RuntimePublicOptionRequirements.all(),
            *StructuredPublicOptionRequirements.all(),
        )

    @classmethod
    def accordion(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn open", "accordion.expanded"),
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn disabled", "accordion.disabled"),
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn controlled", "accordion.controlled"),
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn trigger_area", "accordion.trigger_area"),
            (
                cls.DISCLOSURE_OPTIONS_SOURCE,
                "pub fn reduced_motion",
                "accordion.reduced_motion",
            ),
        )
        return cls.for_page_sources("accordion", mappings)

    @classmethod
    def context_menu(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn anchor", "context_menu.anchor"),
            ("pub fn placement_priority", "context_menu.placement_priority"),
            ("pub fn placement_used", "context_menu.placement_used"),
            ("pub fn min_width", "context_menu.min_width"),
            ("pub fn max_height", "context_menu.max_height"),
        )
        return cls.for_page("context-menu", cls.CONTEXT_MENU_SOURCE, mappings)

    @classmethod
    def diagnostics_list(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.DIAGNOSTICS_SOURCE, "pub group_by:", "diagnostics.group_by"),
            (cls.DIAGNOSTICS_SOURCE, "pub sort_by:", "diagnostics.sort_by"),
            (cls.DIAGNOSTICS_SOURCE, "pub severity_filter:", "diagnostics.severity_filter"),
            (
                cls.DIAGNOSTICS_SOURCE,
                "pub wrap_error_navigation:",
                "diagnostics.wrap_error_navigation",
            ),
            (cls.DIAGNOSTICS_SOURCE, "pub virtualization:", "diagnostics.virtualization"),
            (cls.DIAGNOSTICS_ACTIONS_SOURCE, "OpenBulkPreview", "diagnostics.bulk_action"),
            (cls.DIAGNOSTICS_TYPES_SOURCE, "pub fn fix_preview", "diagnostics.fix_preview"),
        )
        return cls.for_page_sources("diagnostics-list", mappings)

    @classmethod
    def drag_and_drop(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.DROP_TARGET_SOURCE, "pub fn accepted_tag", "drag.accept_policy"),
            (cls.DROP_TARGET_SOURCE, "pub fn auto_scroll", "drag.autoscroll"),
            (cls.DRAG_SOURCE_SOURCE, "pub fn keyboard_draggable", "drag.keyboard_draggable"),
            (cls.DROP_TARGET_SOURCE, "pub fn indicator_kind", "drag.drop_indicator"),
        )
        return cls.for_page_sources("drag-and-drop", mappings)

    @classmethod
    def empty_state(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn new", "empty_state.heading"),
            ("pub fn body", "empty_state.body"),
            ("pub fn icon", "empty_state.icon"),
            ("pub fn illustration", "empty_state.illustration"),
            ("pub fn tone", "empty_state.tone"),
            ("pub fn size", "empty_state.size"),
            ("pub fn alignment", "empty_state.alignment"),
            ("pub fn primary_action", "empty_state.actions"),
            ("pub fn secondary_action", "empty_state.actions"),
        )
        return cls.for_page("empty-state", cls.EMPTY_STATE_SOURCE, mappings)

    @classmethod
    def settings_list(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.SETTINGS_LIST_SOURCE, "pub fn new", "settings_list.label"),
            (cls.SETTINGS_LIST_SOURCE, "pub const fn density", "settings_list.density"),
            (
                cls.SETTINGS_LIST_SOURCE,
                "pub fn dirty_visualization",
                "settings_list.dirty_visualization",
            ),
            (cls.SETTINGS_LIST_SOURCE, "pub fn query", "settings_list.query"),
            (cls.SETTINGS_LIST_SOURCE, "pub fn section", "settings_list.sections"),
            (cls.SETTINGS_FIELD_SOURCE, "pub fn new", "settings_list.section_label"),
            (
                cls.SETTINGS_FIELD_SOURCE,
                "pub fn description",
                "settings_list.section_description",
            ),
            (cls.SETTINGS_FIELD_SOURCE, "pub fn icon", "settings_list.section_icon"),
            (cls.SETTINGS_FIELD_SOURCE, "pub fn field", "settings_list.field_count"),
            (cls.SETTINGS_FIELD_SOURCE, "pub fn footer", "settings_list.section_footer"),
            (
                cls.SETTINGS_FIELD_SOURCE,
                "pub const fn collapsible",
                "settings_list.section_collapsible",
            ),
            (
                cls.SETTINGS_FIELD_SOURCE,
                "pub const fn default_collapsed",
                "settings_list.default_collapsed",
            ),
            (cls.SETTINGS_FIELD_SOURCE, "pub fn new", "settings_list.field_label"),
            (
                cls.SETTINGS_FIELD_SOURCE,
                "pub fn description",
                "settings_list.field_description",
            ),
            (cls.SETTINGS_CONTROL_SOURCE, "pub const fn kind", "settings_list.control_kind"),
            (cls.SETTINGS_CONTROL_SOURCE, "pub fn new", "settings_list.control_options"),
            (cls.SETTINGS_CONTROL_SOURCE, "pub fn custom", "settings_list.custom_control"),
            (cls.SETTINGS_CONTROL_SOURCE, "pub fn set_value", "settings_list.set_value"),
            (cls.SETTINGS_FIELD_SOURCE, "pub fn reset_to_default", "settings_list.reset"),
        )
        return cls.for_page_sources("settings-list", mappings)

    @classmethod
    def shortcut_cheatsheet(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (
                "pub fn new(label: impl Into<String>)",
                "shortcut_cheatsheet.label",
            ),
            ("pub fn group", "shortcut_cheatsheet.groups"),
            (
                "pub fn new(title: impl Into<String>)",
                "shortcut_cheatsheet.group_title",
            ),
            ("pub fn item", "shortcut_cheatsheet.items"),
            (
                "pub fn new(id: impl Into<String>, label: impl Into<String>, combo: KeyCombo)",
                "shortcut_cheatsheet.item_combo",
            ),
            ("pub fn query", "shortcut_cheatsheet.query"),
            ("pub fn group_layout", "shortcut_cheatsheet.group_layout"),
            ("SelectShortcut", "shortcut_cheatsheet.selected"),
            ("pub fn visible_items", "shortcut_cheatsheet.result_count"),
        )
        return cls.for_page("shortcut-cheatsheet", cls.SHORTCUT_CHEATSHEET_SOURCE, mappings)

    @classmethod
    def skeleton_cluster(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn preset", "skeleton_cluster.preset"),
            ("pub fn item", "skeleton_cluster.children"),
            ("pub fn live_region", "skeleton_cluster.live_region"),
            ("pub fn reduced_motion", "skeleton_cluster.reduced_motion"),
        )
        return cls.for_page("skeleton-cluster", cls.SKELETON_CLUSTER_SOURCE, mappings)

    @classmethod
    def status_bar(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.STATUS_BAR_SOURCE, "pub fn mode", "status_bar.mode"),
            (cls.STATUS_BAR_SOURCE, "pub fn segment", "status_bar.segments"),
            (cls.STATUS_BAR_SOURCE, "pub fn density", "status_bar.density"),
            (cls.STATUS_BAR_MODEL_SOURCE, "pub fn popover", "status_bar.progress_popover"),
            (cls.STATUS_BAR_SOURCE, "pub fn message", "status_bar.message"),
            (cls.STATUS_BAR_SOURCE, "pub fn severity", "status_bar.severity"),
            (cls.STATUS_BAR_SOURCE, "pub fn dismiss_action", "status_bar.dismiss"),
            (
                cls.STATUS_BAR_MODEL_SOURCE,
                "pub fn accessibility_label",
                "status_bar.segment_a11y",
            ),
        )
        return cls.for_page_sources("status-bar", mappings)

    @classmethod
    def toolbar(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn display_mode", "toolbar.display_mode"),
            ("pub fn density", "toolbar.density"),
            ("pub fn overflow_strategy", "toolbar.overflow_strategy"),
            ("pub fn action", "toolbar.actions"),
            ("pub fn group", "toolbar.groups"),
            ("pub fn context_menu_anchor", "toolbar.context_menu_anchor"),
        )
        return (
            *cls.for_page("toolbar", cls.TOOLBAR_SOURCE, mappings),
            *cls.for_page(
                "toolbar",
                cls.TOOLBAR_ACTION_SOURCE,
                (
                    ("pub fn priority", "toolbar.action_priority"),
                    ("pub fn accelerator", "toolbar.action_accelerator"),
                    ("pub fn split", "toolbar.action_split"),
                    ("pub fn group_id", "toolbar.action_group"),
                    ("pub fn tooltip", "toolbar.action_tooltip"),
                    ("pub fn accessibility_label", "toolbar.action_a11y"),
                    ("pub fn disabled", "toolbar.action_disabled"),
                ),
            ),
            *cls.for_page(
                "toolbar",
                cls.TOOLBAR_GROUP_SOURCE,
                (
                    ("pub fn label", "toolbar.group_label"),
                    ("pub fn divider", "toolbar.group_divider"),
                ),
            ),
            *cls.for_page(
                "toolbar",
                cls.TOOLBAR_SPLIT_SOURCE,
                (
                    ("pub fn disabled", "toolbar.split_disabled"),
                    ("pub fn tooltip", "toolbar.split_tooltip"),
                    ("pub fn accessibility_label", "toolbar.split_a11y"),
                ),
            ),
        )

    @classmethod
    def window_control_button_group(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub controls:", "window_control.controls"),
            ("pub position:", "window_control.position"),
            ("pub visibility:", "window_control.visibility"),
            ("pub size:", "window_control.size"),
        )
        return cls.for_page(
            "window-control-button-group",
            cls.WINDOW_CONTROL_SOURCE,
            mappings,
        )

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
