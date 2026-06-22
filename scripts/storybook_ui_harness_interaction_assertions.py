from __future__ import annotations

import re
from pathlib import Path

PAGE_TOKEN = re.compile(r'"([a-z0-9-]+)"')


class StorybookUiInteractionHarness:
    def __init__(self, root: Path) -> None:
        self.root = root

    def failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/required_page_tests.rs"
        )
        if not source:
            return ["required Storybook pages must have window_interaction tests"]
        failures: list[str] = []
        for token in self.required_tokens():
            if token not in source:
                failures.append(
                    f"window_interaction required-page tests missing token: {token}"
                )
        for token in self.required_test_names():
            if token not in source:
                failures.append(
                    f"window_interaction required-page tests missing case: {token}"
                )
        failures.extend(self.layout_page_interaction_contract_failures())
        failures.extend(self.navigation_page_interaction_contract_failures())
        failures.extend(self.inspector_option_contract_failures())
        failures.extend(self.preset_distinct_contract_failures())
        failures.extend(self.preset_tab_scroll_clip_hit_contract_failures())
        failures.extend(self.screen_state_store_contract_failures())
        failures.extend(self.button_family_interaction_contract_failures())
        failures.extend(self.button_family_instance_interaction_contract_failures())
        failures.extend(self.text_input_instance_interaction_contract_failures())
        failures.extend(self.text_area_instance_interaction_contract_failures())
        failures.extend(self.binary_choice_instance_interaction_contract_failures())
        failures.extend(self.toggle_instance_interaction_contract_failures())
        failures.extend(self.segmented_toggle_instance_interaction_contract_failures())
        failures.extend(self.selection_instance_interaction_contract_failures())
        failures.extend(self.selection_list_preset_instance_contract_failures())
        failures.extend(self.menu_instance_interaction_contract_failures())
        failures.extend(self.side_menu_instance_interaction_contract_failures())
        failures.extend(self.tree_view_instance_interaction_contract_failures())
        failures.extend(self.color_picker_instance_interaction_contract_failures())
        failures.extend(self.settings_list_instance_interaction_contract_failures())
        failures.extend(self.diagnostics_list_instance_interaction_contract_failures())
        failures.extend(self.breadcrumb_instance_interaction_contract_failures())
        failures.extend(self.status_bar_instance_interaction_contract_failures())
        failures.extend(self.toolbar_instance_interaction_contract_failures())
        failures.extend(self.tabs_instance_interaction_contract_failures())
        failures.extend(self.closeable_tab_strip_instance_interaction_contract_failures())
        failures.extend(self.context_menu_interaction_contract_failures())
        failures.extend(self.context_menu_instance_interaction_contract_failures())
        failures.extend(self.closeable_tab_strip_context_menu_contract_failures())
        failures.extend(self.dynamic_array_instance_interaction_contract_failures())
        failures.extend(self.drag_and_drop_instance_interaction_contract_failures())
        failures.extend(self.panel_instance_interaction_contract_failures())
        failures.extend(self.command_palette_instance_interaction_contract_failures())
        failures.extend(self.option_semantic_state_contract_failures())
        return failures

    @staticmethod
    def required_tokens() -> tuple[str, ...]:
        return (
            "StoryRequirements::required_pages()",
            "StorybookInteractionSpec::for_page(page)",
            "preview_detail::component_action_hit_rect(page)",
            "click_rect",
            "component_body_pixel_diff",
            "other_preset_for(page, original_preset_index, stored_preset_index)",
        )

    @staticmethod
    def required_test_names() -> tuple[str, ...]:
        return (
            "every_required_page_has_screen_action_and_settings_paths",
            "every_required_page_click_repaints_component_body",
            "every_required_page_setting_repaints_component_body",
            "every_required_page_preset_tab_repaints_component_body",
            "every_required_page_keeps_action_state_separate_from_other_pages",
            "every_required_page_keeps_window_interaction_instances_separate",
            "every_required_page_keeps_settings_state_separate_from_other_pages",
            "every_required_page_keeps_action_and_settings_state_separate_between_presets",
        )

    def layout_page_interaction_contract_failures(self) -> list[str]:
        sources = self.visual_test_sources()
        required_pages = set(self.required_pages())
        contracts = {
            "align-center": (
                "align_center_window_interaction_click_updates_preview_state",
                "StorybookWindowState",
                "apply_click",
                '"align-center"',
                "align_measure",
                "alignment_changed",
            ),
            "scroll-area": (
                "scroll_area_window_interaction_scroll_updates_preview_state",
                "scroll_area_window_interaction_keeps_instance_scroll_state_isolated",
                "StorybookWindowState",
                "apply_scroll_delta_at_for_test",
                '"scroll-area"',
                "panel_scroll.preview_y",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "component_body_pixel_diff",
            ),
        }
        failures: list[str] = []
        for page, tokens in contracts.items():
            if page not in required_pages:
                continue
            for token in tokens:
                if token not in sources:
                    failures.append(
                        f"{page}: missing specific window_interaction test token: {token}"
                    )
        return failures

    def navigation_page_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/preview_action_tests.rs"
        )
        if not source:
            return ["window_interaction preview action tests are missing"]
        tokens = (
            "side_menu_window_interaction_selects_route_and_repaints",
            '"side-menu"',
            "side_menu_select",
            "select_box_selected",
            "route=1 focus=1",
            "pixel_diff(&before, &after)",
        )
        return [
            f"side-menu window interaction contract missing token: {token}"
            for token in tokens
            if token not in source
        ]

    def visual_test_sources(self) -> str:
        visual_root = self.root / "crates/katana-ui-core-storybook/src/visual"
        sources: list[str] = []
        for pattern in ("visual_*_tests.rs", "window_interaction/tests/*.rs"):
            for path in sorted(visual_root.glob(pattern)):
                sources.append(path.read_text(encoding="utf-8"))
        return "\n".join(sources)

    def inspector_option_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_inspector_option_contract_tests.rs"
        )
        if not source:
            return ["Inspector option contract test is missing"]
        text_entry_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_inspector_text_entry_preset_tests.rs"
        )
        button_preset_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_inspector_button_preset_tests.rs"
        )
        preset_follow_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_inspector_preset_follow_tests.rs"
        )
        fallback_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_inspector_fallback_status_tests.rs"
        )
        combined_source = (
            f"{source}\n{text_entry_source}\n{button_preset_source}\n"
            f"{preset_follow_source}\n{fallback_source}"
        )
        failures: list[str] = []
        for token in self.required_inspector_option_contract_tokens():
            if token not in combined_source:
                failures.append(
                    f"Inspector option contract test missing token: {token}"
                )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        if "visual_inspector_option_contract_tests" not in visual_mod:
            failures.append("Inspector option contract test is not wired into visual/mod.rs")
        if "visual_inspector_button_preset_tests" not in visual_mod:
            failures.append("Inspector button preset test is not wired into visual/mod.rs")
        if "visual_inspector_preset_follow_tests" not in visual_mod:
            failures.append("Inspector preset follow test is not wired into visual/mod.rs")
        if "visual_inspector_text_entry_preset_tests" not in visual_mod:
            failures.append("Inspector text-entry preset test is not wired into visual/mod.rs")
        if not fallback_source:
            failures.append("Inspector fallback status test is missing")
        elif "visual_inspector_fallback_status_tests" not in visual_mod:
            failures.append("Inspector fallback status test is not wired into visual/mod.rs")
        return failures

    def preset_distinct_contract_failures(self) -> list[str]:
        files = sorted(
            (
                self.root
                / "crates/katana-ui-core-storybook/src/visual"
            ).glob("visual_*_tests.rs")
        )
        sources = [(path.name, path.read_text(encoding="utf-8")) for path in files]
        if not sources:
            return ["Storybook visual preset distinct tests are missing"]
        failures: list[str] = []
        for page in self.required_pages():
            if not self.page_has_preset_distinct_test(page, sources):
                failures.append(
                    f"{page}: preset tabs must have a distinct rendering contract test"
                )
        return failures

    def required_pages(self) -> list[str]:
        source = self.read_optional("crates/katana-ui-core-storybook/src/requirements.rs")
        return PAGE_TOKEN.findall(source.split("const MIN_SINGLE_NODE", 1)[0])

    @staticmethod
    def page_has_preset_distinct_test(
        page: str, sources: list[tuple[str, str]]
    ) -> bool:
        for file_name, source in sources:
            if "presets_render_distinct" not in source:
                continue
            if f'"{page}"' in source:
                return True
            if page.replace("-", "_") in file_name:
                return True
        return False

    def screen_state_store_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store.rs"
        )
        test_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store_tests.rs"
        )
        if not source:
            return ["Storybook screen state store is missing"]
        failures: list[str] = []
        if not test_source:
            failures.append("Storybook screen state store instance contract tests are missing")
        required = (
            "StorybookScreenStateKey",
            "component_id: &'static str",
            "preset_index: usize",
            "instance_id: &'static str",
            "save_instance",
            "restore_instance",
            "screen_state_store_keeps_page_and_preset_state_separate",
            "screen_state_store_removes_default_state_for_page_preset_key_only",
            "screen_state_store_keeps_non_input_component_instances_separate",
            "screen_state_store_keeps_selection_component_instances_separate",
            "TabsScreenAction::AddTab",
            "SelectionScreenAction::ComboFilter",
        )
        failures.extend(
            f"Storybook screen state store contract missing token: {token}"
            for token in required
            if token not in source
        )
        required_tests = (
            "every_required_page_keeps_screen_state_instances_separate",
            "screen_state_store_removes_default_instance_key_only_for_required_pages",
            "StoryRequirements::required_pages",
            'store.save_instance(page, 0, "primary"',
            'store.restore_instance(page, 0, "secondary")',
        )
        failures.extend(
            f"Storybook screen state store instance test missing token: {token}"
            for token in required_tests
            if token not in test_source
        )
        return failures

    def preset_tab_scroll_clip_hit_contract_failures(self) -> list[str]:
        scroll_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/preset_tab_scroll.rs"
        )
        tab_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/preset_tabs.rs"
        )
        label_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/preset_tab_label.rs"
        )
        test_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_preset_tab_scroll_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        contracts = (
            (
                "preset tab overflow scroll contract",
                scroll_source,
                (
                    "max_scroll_x_for_page",
                    "scroll_delta",
                    "clamp_offset",
                    "viewport_rect",
                    "visible_index_range",
                ),
            ),
            (
                "preset tab current follow contract",
                scroll_source,
                (
                    "ensure_index_visible",
                    "active_index_scroll_x",
                    "tab_left < offset",
                    "tab_right > offset + viewport_width()",
                ),
            ),
            (
                "preset tab clipping contract",
                tab_source,
                (
                    "canvas.with_clip",
                    "preset_tab_label::fit",
                ),
            ),
            (
                "preset tab label fitting contract",
                label_source,
                (
                    "measure_width",
                    "TRUNCATION_MARKER",
                    "measured_width_for_test",
                ),
            ),
            (
                "preset tab hit bounds contract",
                scroll_source,
                (
                    "hit_index_at",
                    "viewport.contains(x, y)",
                    "visual_rect_for_index(page, index, false, scroll_x)",
                    "rect.contains(x, y)",
                ),
            ),
            (
                "preset tab scroll guard tests",
                test_source,
                (
                    "overflowing_preset_tabs_have_horizontal_scroll_range",
                    "visible_preset_tab_rects_stay_fully_inside_viewport",
                    "rendered_preset_tabs_are_clipped_at_preview_right_edge",
                    "external_preset_selection_scrolls_current_tab_into_view",
                    "clicking_scrolled_preset_tab_uses_logical_tab_index",
                    "wheel_over_preset_tabs_scrolls_tabs_without_scrolling_root",
                    "external_render_preset_scrolls_active_overflow_tab_into_view",
                    "apply_scroll_delta_at_for_test",
                    "state.select_preset(last_preset)",
                    "state.scroll_y",
                    "active_tab_is_inside_viewport",
                    "pixel_at(&canvas",
                ),
            ),
            (
                "preset tab scroll guard wiring",
                visual_mod,
                ("visual_preset_tab_scroll_tests",),
            ),
        )
        for label, source, tokens in contracts:
            if not source:
                failures.append(f"{label} is missing")
                continue
            failures.extend(
                f"{label} missing token: {token}"
                for token in tokens
                if token not in source
            )
        return failures

    def button_family_interaction_contract_failures(self) -> list[str]:
        hover_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_button_hover_tests.rs"
        )
        button_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_button_center_tests.rs"
        )
        menu_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_menu_button_tests.rs"
        )
        cursor_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/button_operation_tests.rs"
        )
        operation_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/button_operation.rs"
        )
        required_sources = (
            (
                "button hover contract",
                hover_source,
                (
                    "hover_draws_visible_border_for_all_button_surfaces",
                    "hover_border",
                    "must not use text color",
                ),
            ),
            (
                "button measured center contract",
                button_source,
                (
                    "button_label_center_uses_measured_text_width",
                    "measure_button_label_width",
                    "centered_label_x_for_test",
                ),
            ),
            (
                "menu-button hover contract",
                menu_source,
                (
                    "menu_button_hover_draws_shared_button_family_border_token",
                    "hover_border",
                    "ThemeSnapshot::dark",
                ),
            ),
            (
                "button cursor contract",
                cursor_source,
                (
                    "BUTTON_FAMILY_CURSOR_PAGES",
                    '"menu-button"',
                    "StorybookCursorStyle::PointingHand",
                ),
            ),
            (
                "button preview cursor route",
                operation_source,
                (
                    "uses_clickable_preview_cursor",
                    'page == "menu-button"',
                ),
            ),
            (
                "button option state contract",
                self.read_optional(
                    "crates/katana-ui-core-storybook/src/visual/visual_inspector_button_preset_tests.rs"
                ),
                (
                    "button_inspector_rows_apply_action_event_and_state_for_every_button_page",
                    "button_option_apply",
                    "button_option_changed",
                    "control.state_label(state.screen_state.button_options)",
                ),
            ),
        )
        failures: list[str] = []
        for label, source, tokens in required_sources:
            if not source:
                failures.append(f"{label} is missing")
                continue
            failures.extend(
                f"{label} missing token: {token}"
                for token in tokens
                if token not in source
            )
        return failures

    def button_family_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "button family",
            "visual_interaction_button_instance_tests.rs",
            (
                "button_family_window_interaction_keeps_instance_state_isolated_across_presets",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "StorybookButtonOptionControl::Label",
                "button_option_apply",
                "button_pressed",
                "component_body_pixel_diff",
            ),
        )

    def selection_instance_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_selection_instance_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        if not source:
            failures.append("selection instance interaction tests are missing")
            return failures
        required = (
            "select_box_window_interaction_keeps_instance_state_isolated",
            "combo_box_window_interaction_keeps_instance_state_isolated",
            "search_box_window_interaction_keeps_instance_state_isolated",
            "selection_list_window_interaction_keeps_instance_state_isolated",
            "state.select_instance(PRIMARY_INSTANCE)",
            "state.select_instance(SECONDARY_INSTANCE)",
            "component_body_pixel_diff",
        )
        failures.extend(
            f"selection instance interaction test missing token: {token}"
            for token in required
            if token not in source
        )
        if "visual_interaction_selection_instance_tests" not in visual_mod:
            failures.append(
                "selection instance interaction tests are not wired into visual/mod.rs"
            )
        return failures

    def selection_list_preset_instance_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "selection-list preset",
            "visual_interaction_selection_list_preset_tests.rs",
            (
                "selection_list_window_interaction_keeps_instance_state_isolated_across_presets",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "state.select_preset(MULTI_PRESET)",
                "selection_list_multi_mask",
                "component_body_pixel_diff",
            ),
        )

    def menu_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "menu",
            "visual_interaction_menu_tests.rs",
            (
                "menu_window_interaction_keeps_open_select_shortcut_instance_isolated",
                "menu_shortcut_activation_keeps_instance_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "menu_open",
                "menu_select",
                "menu_shortcut_activate",
                "component_body_pixel_diff",
            ),
        )

    def text_input_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "text-input",
            "visual_interaction_text_input_state_tests.rs",
            (
                "text_input_preset_tab_switching_keeps_runtime_state_isolated",
                "text_input_keyboard_routes_to_selected_instance_state",
                'state.select_instance("text-input.primary")',
                'state.select_instance("text-input.secondary")',
                "apply_text_input_key",
                "text_input_value_for",
                "assert_ne!",
            ),
        )

    def text_area_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "text-area",
            "visual_interaction_text_area_state_tests.rs",
            (
                "text_area_state_store_keeps_instance_value_focus_and_caret_isolated",
                "text_area_keyboard_routes_to_selected_instance_state",
                'state.select_instance("text-area.primary")',
                'state.select_instance("text-area.secondary")',
                "apply_text_area_key",
                "text_area_value_for",
                "assert_ne!",
            ),
        )

    def side_menu_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "side-menu",
            "visual_interaction_side_menu_tests.rs",
            (
                "side_menu_window_interaction_keeps_instance_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "side_menu_select",
                "route=1 focus=1",
                "component_body_pixel_diff",
            ),
        )

    def tree_view_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "tree-view",
            "visual_interaction_tree_view_tests.rs",
            (
                "tree_view_window_interaction_keeps_instance_state_isolated",
                "tree_view_context_menu_keeps_instance_state_isolated",
                "tree_view_setting_action_keeps_instance_setting_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "tree_click_toggle",
                "tree_context_menu",
                "tree.context_menu=enabled",
                "apply_context_click_for_test",
                "layout_metrics::inspector_setting_row_hit_rect",
                "component_body_pixel_diff",
            ),
        )

    def color_picker_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "color-picker-rgba",
            "visual_interaction_color_picker_options_tests.rs",
            (
                "color_picker_window_interaction_keeps_drag_value_callback_and_blocked_state_isolated",
                "color_picker_readonly_and_disabled_preview_clicks_do_not_mutate_color",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "color_drag",
                "rgba_changed",
                "color_picker_readonly_blocked",
                "color_picker_disabled_blocked",
                "color_picker.rgba_label()",
                "assert_color_picker_runtime",
                "option_state()",
                "has_committed_color",
                "callback_action()",
                "blocks_writes",
                "blocks_focus",
                "component_body_pixel_diff",
            ),
        )

    def settings_list_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "settings-list",
            "visual_interaction_settings_list_options_tests.rs",
            (
                "settings_list_window_interaction_keeps_query_field_collapse_and_reset_instance_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "settings_filter_update_collapse",
                "settings_field_changed",
                "settings_update_field",
                "settings_reset_field",
                "has_dirty_font_size",
                "has_query_filter",
                "has_collapsed_chat_section",
                "assert_settings_list_runtime",
                "option_state()",
                "options.label_workspace",
                "options.control_option_count",
                "options.reset_default",
                "component_body_pixel_diff",
            ),
        )

    def diagnostics_list_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "diagnostics-list",
            "visual_interaction_diagnostics_list_options_tests.rs",
            (
                "diagnostics_list_window_interaction_keeps_filter_bulk_fix_preview_instance_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "diagnostic_fix_preview",
                "diagnostic_fix_preview_toggled",
                "diagnostic_bulk_apply",
                "has_error_filter",
                "has_bulk_applied",
                "has_fix_preview",
                "assert_diagnostics_list_runtime",
                "option_state()",
                "settings_diagnostics_option",
                "molecule_settings_changed",
                "state.screen_state.last_setting_value",
                "options.group_by_source",
                "options.sort_by_location",
                "options.virtualization_windowed",
                "options.fix_preview_collapsed",
                "component_body_pixel_diff",
            ),
        )

    def breadcrumb_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "breadcrumb",
            "visual_interaction_breadcrumb_state_tests.rs",
            (
                "breadcrumb_window_interaction_keeps_instance_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "breadcrumb_selected_index",
                "route=2",
                "component_body_pixel_diff",
            ),
        )

    def status_bar_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "status-bar",
            "visual_interaction_status_bar_state_tests.rs",
            (
                "status_bar_window_interaction_keeps_instance_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "status_bar_segment_popover",
                "open_popover=progress",
                "component_body_pixel_diff",
            ),
        )

    def toolbar_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "toolbar",
            "visual_interaction_toolbar_state_tests.rs",
            (
                "toolbar_window_interaction_keeps_instance_state_isolated",
                "toolbar_window_interaction_disabled_action_does_not_mutate_state",
                "toolbar_window_interaction_disabled_split_does_not_mutate_state",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "tool_toggle",
                "hovered_toolbar_action_index",
                "ACTION_DISABLED_PRESET_INDEX",
                "SPLIT_DISABLED_PRESET_INDEX",
                "component_body_pixel_diff",
            ),
        )

    def tabs_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "tabs",
            "visual_interaction_tabs_state_tests.rs",
            (
                "tabs_window_interaction_keeps_instance_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "TabsScreenAction::AddTab",
                "TabsScreenAction::TogglePinActive",
                "component_body_pixel_diff",
            ),
        )

    def closeable_tab_strip_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "closeable-tab-strip",
            "visual_interaction_closeable_tab_strip_state_tests.rs",
            (
                "closeable_tab_strip_window_interaction_keeps_instance_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "TabsScreenAction::AddTab",
                "TabsScreenAction::CloseActive",
                "component_body_pixel_diff",
            ),
        )

    def closeable_tab_strip_context_menu_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_closeable_tab_strip_context_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        tokens = (
            "closeable_tab_strip_tab_context_menu_applies_workspace_tab_commands",
            "closeable_tab_strip_context_menu_keeps_pinned_tabs_fixed_until_unpinned",
            "CLOSE_OTHERS_INDEX",
            "CLOSE_ALL_INDEX",
            "CLOSE_RIGHT_INDEX",
            "CLOSE_LEFT_INDEX",
            "NEW_GROUP_INDEX",
            "MOVE_TO_GROUP_INDEX",
            "apply_context_click_for_test",
            "click_tab_context_command",
        )
        failures: list[str] = []
        if not source:
            return ["closeable-tab-strip context menu interaction tests are missing"]
        failures.extend(
            f"closeable-tab-strip context menu test missing token: {token}"
            for token in tokens
            if token not in source
        )
        if "visual_interaction_closeable_tab_strip_context_tests" not in visual_mod:
            failures.append(
                "closeable-tab-strip context menu tests are not wired into visual/mod.rs"
            )
        return failures

    def context_menu_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_context_menu_tests.rs"
        )
        interaction = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/window_interaction/context_click.rs"
        )
        screen_state = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/screen_state_context_menu.rs"
        )
        tokens = (
            "context_menu_preview_submenu_and_item_selection_use_real_core_actions",
            "apply_context_click_for_test",
            "dedicated_context_menu_popup::insert_row_rect",
            "dedicated_context_menu_popup::submenu_link_rect",
            "context_menu_submenu_opened",
            "context_menu_item_selected",
            "ContextMenuAction::OpenSubmenu",
            "ContextMenuAction::Activate",
            "context_menu_command_at",
        )
        combined = "\n".join((source, interaction, screen_state))
        if not source:
            return ["context-menu live submenu selection tests are missing"]
        return [
            f"context-menu interaction contract missing token: {token}"
            for token in tokens
            if token not in combined
        ]

    def context_menu_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "context-menu",
            "visual_interaction_context_menu_tests.rs",
            (
                "context_menu_window_interaction_keeps_context_action_instance_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "apply_context_click_for_test",
                "dedicated_context_menu_popup::insert_row_rect",
                "dedicated_context_menu_popup::submenu_link_rect",
                "context_menu_select_item",
                "component_body_pixel_diff",
            ),
        )

    def specific_instance_interaction_contract_failures(
        self, label: str, file_name: str, tokens: tuple[str, ...]
    ) -> list[str]:
        source = self.read_optional(
            f"crates/katana-ui-core-storybook/src/visual/{file_name}"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        module = file_name.removesuffix(".rs")
        failures: list[str] = []
        if not source:
            failures.append(f"{label} instance interaction tests are missing")
            return failures
        failures.extend(
            f"{label} instance interaction test missing token: {token}"
            for token in tokens
            if token not in source
        )
        if module not in visual_mod:
            failures.append(f"{label} instance interaction tests are not wired into visual/mod.rs")
        return failures

    def binary_choice_instance_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_binary_choice_state_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        if not source:
            failures.append("binary choice instance interaction tests are missing")
            return failures
        required = (
            "checkbox_window_interaction_keeps_instance_state_isolated",
            "radio_window_interaction_keeps_instance_state_isolated",
            "checkbox_window_interaction_disabled_toggle_does_not_mutate_state",
            "state.select_instance(CHECKBOX_PRIMARY_INSTANCE)",
            "state.select_instance(CHECKBOX_SECONDARY_INSTANCE)",
            "state.select_instance(RADIO_PRIMARY_INSTANCE)",
            "state.select_instance(RADIO_SECONDARY_INSTANCE)",
            "checkbox_toggle",
            "radio_select",
            "DISABLED_PRESET_INDEX",
            "component_body_pixel_diff",
        )
        failures.extend(
            f"binary choice instance interaction test missing token: {token}"
            for token in required
            if token not in source
        )
        if "visual_interaction_binary_choice_state_tests" not in visual_mod:
            failures.append(
                "binary choice instance interaction tests are not wired into visual/mod.rs"
            )
        return failures

    def toggle_instance_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_toggle_state_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        if not source:
            failures.append("toggle instance interaction tests are missing")
            return failures
        required = (
            "toggle_window_interaction_keeps_instance_state_isolated",
            "toggle_window_interaction_disabled_click_does_not_mutate_state",
            "state.select_instance(PRIMARY_INSTANCE)",
            "state.select_instance(SECONDARY_INSTANCE)",
            "toggle_change",
            "checked=true",
            "DISABLED_PRESET_INDEX",
            "component_body_pixel_diff",
        )
        failures.extend(
            f"toggle instance interaction test missing token: {token}"
            for token in required
            if token not in source
        )
        if "visual_interaction_toggle_state_tests" not in visual_mod:
            failures.append(
                "toggle instance interaction tests are not wired into visual/mod.rs"
            )
        return failures

    def segmented_toggle_instance_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_segmented_toggle_state_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        if not source:
            failures.append("segmented-toggle instance interaction tests are missing")
            return failures
        required = (
            "segmented_toggle_window_interaction_keeps_instance_state_isolated",
            "segmented_toggle_window_interaction_disabled_click_does_not_mutate_state",
            "state.select_instance(PRIMARY_INSTANCE)",
            "state.select_instance(SECONDARY_INSTANCE)",
            "segment_select",
            "segment=1",
            "DISABLED_PRESET_INDEX",
            "component_body_pixel_diff",
        )
        failures.extend(
            f"segmented-toggle instance interaction test missing token: {token}"
            for token in required
            if token not in source
        )
        if "visual_interaction_segmented_toggle_state_tests" not in visual_mod:
            failures.append(
                "segmented-toggle instance interaction tests are not wired into visual/mod.rs"
            )
        return failures

    def dynamic_array_instance_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_dynamic_array_editor_state_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        if not source:
            failures.append("dynamic-array-editor instance interaction tests are missing")
            return failures
        required = (
            "dynamic_array_editor_window_interaction_keeps_instance_state_isolated",
            "state.select_instance(PRIMARY_INSTANCE)",
            "state.select_instance(SECONDARY_INSTANCE)",
            "array_add",
            "array_reorder",
            "component_body_pixel_diff",
        )
        failures.extend(
            f"dynamic-array-editor instance interaction test missing token: {token}"
            for token in required
            if token not in source
        )
        if "visual_interaction_dynamic_array_editor_state_tests" not in visual_mod:
            failures.append(
                "dynamic-array-editor instance interaction tests are not wired into visual/mod.rs"
            )
        return failures

    def drag_and_drop_instance_interaction_contract_failures(self) -> list[str]:
        source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/visual_interaction_drag_and_drop_state_tests.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        failures: list[str] = []
        if not source:
            failures.append("drag-and-drop instance interaction tests are missing")
            return failures
        required = (
            "drag_and_drop_window_interaction_keeps_instance_state_isolated",
            "state.select_instance(PRIMARY_INSTANCE)",
            "state.select_instance(SECONDARY_INSTANCE)",
            "dragging=true",
            "committed=true",
            "component_body_pixel_diff",
        )
        failures.extend(
            f"drag-and-drop instance interaction test missing token: {token}"
            for token in required
            if token not in source
        )
        if "visual_interaction_drag_and_drop_state_tests" not in visual_mod:
            failures.append(
                "drag-and-drop instance interaction tests are not wired into visual/mod.rs"
            )
        return failures

    def command_palette_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "command-palette",
            "visual_interaction_command_palette_options_tests.rs",
            (
                "command_palette_window_interaction_keeps_query_and_highlight_instance_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "command_palette.query=theme",
                "command_palette.highlight=2",
                "assert_command_palette_runtime",
                "option_state()",
                "settings_command_palette_option",
                "molecule_settings_changed",
                "state.screen_state.last_setting_value",
                "command_palette.query()",
                "command_palette.highlighted_index()",
                "options.row_count",
                "options.provider_group_workspace_editor_app",
                "options.shortcut_display_visible",
                "component_body_pixel_diff",
            ),
        )

    def panel_instance_interaction_contract_failures(self) -> list[str]:
        return self.specific_instance_interaction_contract_failures(
            "panel",
            "panel_in_panel_state_tests.rs",
            (
                "panel_window_interaction_keeps_instance_scroll_and_nested_state_isolated",
                "state.select_instance(PRIMARY_INSTANCE)",
                "state.select_instance(SECONDARY_INSTANCE)",
                "PanelChildKey::Details",
                "component_body_pixel_diff",
            ),
        )

    def option_semantic_state_contract_failures(self) -> list[str]:
        semantic_source = self.semantic_state_sources()
        screen_state_source = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/screen_state.rs"
        ) + self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/screen_state_settings_contract.rs"
        )
        visual_mod = self.read_optional(
            "crates/katana-ui-core-storybook/src/visual/mod.rs"
        )
        contracts = (
            (
                "toolbar option semantic state",
                "visual_interaction_toolbar_options_tests.rs",
                (
                    "toolbar_inspector_options_mutate_action_split_and_group_semantic_state",
                    "assert_inspector_option_contract_state",
                    "toolbar.action.disabled=true",
                    "toolbar.split.a11y=Open menu",
                ),
            ),
            (
                "settings-list option semantic state",
                "visual_interaction_settings_list_options_tests.rs",
                (
                    "settings_list_inspector_options_mutate_field_control_and_reset_semantic_state",
                    "settings_list.label=Workspace settings",
                    "settings_list.dirty=Highlight",
                    "settings_list.section.description=visible",
                    "settings_list.field.label=Font size",
                    "settings_list.control.kind=Number",
                    "settings_list.control.options=4",
                    "settings_list.reset=default",
                    "assert_settings_list_runtime",
                    "option_state()",
                    "options.label_workspace",
                    "options.density_compact",
                    "options.dirty_highlight",
                    "options.sections_app_lint",
                    "options.section_label_editor",
                    "options.section_description_visible",
                    "options.section_icon_gear",
                    "options.field_count",
                    "options.section_footer_policy",
                    "options.section_collapsible",
                    "options.default_collapsed",
                    "options.field_label_font_size",
                    "options.field_description_visible",
                    "options.control_kind_number",
                    "options.control_option_count",
                    "options.custom_control_button",
                    "options.value_changed",
                    "options.reset_default",
                ),
            ),
            (
                "color-picker option semantic state",
                "visual_interaction_color_picker_options_tests.rs",
                (
                    "color_picker_inspector_options_mutate_hue_alpha_block_and_callback_semantic_state",
                    "color_picker.rgba=rgba(64,128,255,.8)",
                    "color_picker.color_area=saturation/value",
                    "color_picker.trigger.border=false",
                    "color_picker.readonly.blocks_writes",
                    "color_picker.disabled.blocks_focus",
                    "assert_color_picker_runtime",
                    "option_state()",
                    "options.panel_open",
                    "options.blending_multiply",
                    "options.color_area_visible",
                    "options.trigger_large",
                    "options.title_customized",
                    "options.panel_scale_percent",
                    "options.trigger_border",
                ),
            ),
            (
                "virtualization option semantic state",
                "visual_interaction_virtualization_options_tests.rs",
                (
                    "virtualization_inspector_options_mutate_range_focus_and_measurement_semantic_state",
                    "assert_inspector_option_contract_state",
                    "virtualization.overscan=4",
                    "virtualization.focused_index=42",
                    "virtualization.measured_correction=+8",
                ),
            ),
            (
                "search-control option semantic state",
                "visual_interaction_search_control_options_tests.rs",
                (
                    "search_control_inspector_options_mutate_match_replace_and_active_result_semantic_state",
                    "assert_inspector_option_contract_state",
                    "search_control.query=heading",
                    "search_control.match_case=true",
                    "search_control.result_count=0",
                    "search_control.active_index=none",
                ),
            ),
            (
                "status-bar option semantic state",
                "visual_interaction_status_bar_options_tests.rs",
                (
                    "status_bar_inspector_options_mutate_segment_and_message_semantic_state",
                    "assert_inspector_option_contract_state",
                    "status_bar.progress_popover=true",
                    "status_bar.segment_a11y=custom",
                ),
            ),
            (
                "chip option semantic state",
                "visual_interaction_chip_options_tests.rs",
                (
                    "chip_inspector_options_mutate_label_icon_variant_and_state_semantic_state",
                    "assert_inspector_option_contract_state",
                    "chip.leading_icon=tag",
                    "chip.a11y_label=Filter chip",
                    "chip.focused=true",
                ),
            ),
            (
                "chip family option semantic state",
                "visual_interaction_chip_family_options_tests.rs",
                (
                    "attachment_chip_inspector_options_mutate_kind_status_and_retry_semantic_state",
                    "chip_group_inspector_options_mutate_overflow_reorder_and_width_semantic_state",
                    "assert_inspector_option_contract_state",
                    "attachment.retry=visible",
                    "chip_group.overflow_trigger_width=32",
                ),
            ),
            (
                "text-entry option semantic state",
                "visual_interaction_text_entry_options_tests.rs",
                (
                    "text_input_inspector_options_mutate_value_slot_icon_and_blocking_semantic_state",
                    "text_area_inspector_options_mutate_multiline_scroll_slot_and_blocking_semantic_state",
                    "assert_inspector_option_contract_state",
                    "text_input.leading_slot.icon=search-svg",
                    "text_area.horizontal_scrollbar_visible=true",
                ),
            ),
            (
                "icon option semantic state",
                "visual_interaction_icon_options_tests.rs",
                (
                    "icon_inspector_options_mutate_svg_source_role_paint_and_token_semantic_state",
                    "assert_inspector_option_contract_state",
                    "icon.svg_source=custom-svg",
                    "icon.paint_policy=currentColor",
                    "icon.theme_token=muted",
                ),
            ),
            (
                "foundation option semantic state",
                "visual_interaction_foundation_options_tests.rs",
                (
                    "text_inspector_options_mutate_role_script_metrics_and_wrap_semantic_state",
                    "progress_bar_inspector_options_mutate_progress_loading_tone_and_size_semantic_state",
                    "loading_indicator_inspector_options_mutate_animation_label_tone_and_size_semantic_state",
                    "assert_inspector_option_state",
                    "text.script=jp+emoji",
                    "progress_bar.percent=82",
                    "loading_dots.dot_count=5",
                    "spinner.animation_state=Paused",
                ),
            ),
            (
                "foundation extra option semantic state",
                "visual_interaction_foundation_extra_options_tests.rs",
                (
                    "foundation_extra_inspector_options_mutate_theme_key_cap_and_motion_semantic_state",
                    "assert_inspector_option_state",
                    "theme.color.accent=green",
                    "key_cap.theme.color=accent",
                    "motion.reduced_policy=ForceReduced",
                ),
            ),
            (
                "skeleton option semantic state",
                "visual_interaction_skeleton_options_tests.rs",
                (
                    "skeleton_inspector_options_mutate_shape_motion_size_and_a11y_semantic_state",
                    "assert_inspector_option_state",
                    "skeleton.line_thickness=12",
                    "skeleton.reduced_motion=true",
                    "skeleton.aspect_ratio=16:9",
                ),
            ),
            (
                "split-pane option semantic state",
                "visual_interaction_split_pane_options_tests.rs",
                (
                    "split_pane_inspector_options_mutate_axis_ratio_bounds_and_resize_semantic_state",
                    "assert_inspector_option_state",
                    "split_pane.ratio_percent=64",
                    "split_pane.handle_width_px=10",
                    "split_pane.resize_mode=KeyboardOnly",
                ),
            ),
            (
                "layout option semantic state",
                "visual_interaction_layout_options_tests.rs",
                (
                    "layout_inspector_options_mutate_axis_gap_alignment_and_overflow_semantic_state",
                    "assert_layout_option_state",
                    "assert_inspector_option_state_with_event",
                    "layout_option_changed",
                    "row.alignment=center",
                    "grid.overflow=scroll",
                    "scroll_area.overflow=scroll",
                    "align_center.alignment=center",
                ),
            ),
            (
                "primitive option semantic state",
                "visual_interaction_primitive_options_tests.rs",
                (
                    "primitive_inspector_options_mutate_variant_tone_size_and_theme_slot_semantic_state",
                    "assert_inspector_option_state",
                    "divider.variant=alternate",
                    "color_swatch.tone=accent",
                    "slide_control.theme.slot=custom",
                ),
            ),
            (
                "binary choice option semantic state",
                "visual_interaction_binary_choice_options_tests.rs",
                (
                    "binary_choice_inspector_options_mutate_selected_disabled_focus_and_checked_semantic_state",
                    "binary_choice_disabled_option_blocks_preview_mutation",
                    "checkbox.checked=true",
                    "radio.focus=visible",
                    "toggle.disabled=true",
                    "segmented_toggle.selected=true",
                    "assert_component_state(page, setting, &state.screen_state)",
                    "checkbox_state_snapshot",
                    "radio_state_snapshot",
                    "assert_binary_component_state",
                    "state.common.disabled",
                    "state.interaction.focused",
                    "settings_disabled",
                    "selection_settings_changed",
                ),
            ),
            (
                "closeable-tab-strip option state",
                "visual_interaction_closeable_tab_strip_options_tests.rs",
                (
                    "closeable_tab_strip_inspector_options_mutate_active_overflow_pin_and_group_semantic_state",
                    "tabs.active=settings",
                    "tabs.pinned=true left-fixed",
                    "tabs.group=Docs",
                    "tabs.overflow=menu",
                    "assert_closeable_tab_event",
                    "state.screen_state.last_action",
                    "state.screen_state.last_event",
                    "starts_with(\"closeable_tab\")",
                ),
            ),
            (
                "tabs option state",
                "visual_interaction_tabs_options_tests.rs",
                (
                    "tabs_inspector_options_mutate_tab_model_state",
                    "tabs.count=6 active=notes.md",
                    "tabs.pinned=true left-fixed",
                    "tabs.group=Docs",
                    "tabs.overflow=menu",
                    "tabs.active_scroll=follow",
                    "assert_tabs_option_event",
                    "state.screen_state.last_action",
                    "state.screen_state.last_event",
                    "starts_with(\"closeable_tab\")",
                ),
            ),
            (
                "surface option semantic state",
                "visual_interaction_surface_options_tests.rs",
                (
                    "badge_inspector_options_mutate_status_size_icon_and_variant_semantic_state",
                    "card_inspector_options_mutate_slot_click_and_child_semantic_state",
                    "empty_state_inspector_options_mutate_content_alignment_and_action_semantic_state",
                    "assert_inspector_option_contract_state",
                    "badge.leading_icon=dot",
                    "card.child_state=changed",
                    "empty_state.actions=Primary+Secondary",
                ),
            ),
            (
                "banner option semantic state",
                "visual_interaction_banner_options_tests.rs",
                (
                    "banner_inspector_options_mutate_feedback_details_icon_and_placement_semantic_state",
                    "assert_inspector_option_state",
                    "banner.severity=warning",
                    "banner.details=expanded",
                    "banner.leading_icon=custom",
                    "banner.placement=sticky",
                ),
            ),
            (
                "feedback option semantic state",
                "visual_interaction_feedback_options_tests.rs",
                (
                    "feedback_inspector_options_mutate_severity_duration_action_and_dismiss_semantic_state",
                    "assert_inspector_option_state",
                    "toast_stack.duration=custom",
                    "notification_toast.action=visible",
                    "notification_toast.dismiss=true",
                ),
            ),
            (
                "collection option semantic state",
                "visual_interaction_collection_options_tests.rs",
                (
                    "collection_inspector_options_mutate_list_collapsible_hover_and_panel_semantic_state",
                    "assert_collection_option_state",
                    "assert_inspector_option_state_with_event",
                    "panel_active_select",
                    "panel_scrollbar_hide",
                    "list.virtualization=visible_range",
                    "collapsible_panel.resize_handle=true",
                    "hover_card.pointer_follow=true",
                    "panel.horizontal_scroll=changed",
                    "panel.nested_state=independent",
                ),
            ),
            (
                "navigation option semantic state",
                "visual_interaction_navigation_options_tests.rs",
                (
                    "navigation_inspector_options_mutate_menu_form_breadcrumb_side_and_tree_semantic_state",
                    "assert_navigation_option_state",
                    "breadcrumb_click",
                    "field_validate",
                    "form_field_helper_text",
                    "menu.selected_index=1",
                    "menu.panel_placement=resolved",
                    "form_field.helper_text=long",
                    "breadcrumb.crumb_action=callback",
                    "side_menu.hover_expansion=true",
                    "tree.context_menu=enabled",
                ),
            ),
            (
                "overlay option semantic state",
                "visual_interaction_overlay_options_tests.rs",
                (
                    "tooltip_inspector_options_mutate_overlay_semantic_state",
                    "popover_inspector_options_mutate_overlay_semantic_state",
                    "modal_inspector_options_mutate_overlay_semantic_state",
                    "modal_overlay_inspector_options_mutate_overlay_semantic_state",
                    "assert_inspector_option_state",
                    "tooltip.open=true",
                    "popover.placement=edge",
                    "modal.focus=first",
                    "modal_overlay.dismiss=outside",
                ),
            ),
            (
                "selection option semantic state",
                "visual_interaction_selection_options_tests.rs",
                (
                    "combo_box_inspector_options_mutate_choice_semantic_state",
                    "select_box_inspector_options_mutate_choice_semantic_state",
                    "selection_list_inspector_options_mutate_list_semantic_state",
                    "menu_button_inspector_options_mutate_menu_semantic_state",
                    "search_box_inspector_options_mutate_search_semantic_state",
                    "assert_inspector_option_contract_state",
                    "combo.outside_click_dismiss=true",
                    "selection_list.more_row=true",
                    "menu_button.select_action=callback",
                    "search_box.regex_case=true/true",
                ),
            ),
            (
                "command-palette option semantic state",
                "visual_interaction_command_palette_options_tests.rs",
                (
                    "command_palette_inspector_options_mutate_query_highlight_provider_semantic_state",
                    "command_palette.query=theme",
                    "command_palette.highlight=2",
                    "command_palette.provider_group=workspace/editor/app",
                    "assert_command_palette_runtime",
                    "option_state()",
                    "settings_command_palette_option",
                    "molecule_settings_changed",
                    "state.screen_state.last_setting_value",
                    "command_palette.query()",
                    "command_palette.highlighted_index()",
                    "options.row_count",
                    "options.provider_group_workspace_editor_app",
                    "options.shortcut_display_visible",
                ),
            ),
            (
                "shortcut-cheatsheet option semantic state",
                "visual_interaction_shortcut_cheatsheet_options_tests.rs",
                (
                    "shortcut_cheatsheet_inspector_options_mutate_filter_selection_and_count_semantic_state",
                    "shortcut_cheatsheet.query=カテゴリ",
                    "shortcut_cheatsheet.selected=format",
                    "shortcut_cheatsheet.result_count=1",
                    "assert_shortcut_cheatsheet_runtime",
                    "option_state()",
                    "settings_shortcut_cheatsheet_option",
                    "runtime_settings_changed",
                    "state.screen_state.last_setting_value",
                    "options.label_editor_keys",
                    "options.group_count",
                    "options.item_count",
                    "options.group_layout_one_column",
                    "options.query_category",
                    "options.selected_format",
                    "options.result_count",
                    "cheatsheet.visible_item_count()",
                ),
            ),
            (
                "runtime option semantic state",
                "visual_interaction_runtime_options_tests.rs",
                (
                    "context_menu_inspector_options_mutate_anchor_placement_and_size_semantic_state",
                    "startup_state_inspector_options_mutate_error_progress_and_action_semantic_state",
                    "code_diff_inspector_options_mutate_mode_layout_and_sync_semantic_state",
                    "assert_inspector_option_contract_state",
                    "context_menu.placement_used=AboveEnd",
                    "startup_state.retry=true",
                    "code_diff.scroll_sync=false",
                ),
            ),
            (
                "runtime structured option semantic state",
                "visual_interaction_runtime_structured_options_tests.rs",
                (
                    "shortcut_combo_inspector_options_mutate_display_size_tone_and_a11y_semantic_state",
                    "skeleton_cluster_inspector_options_mutate_preset_children_and_motion_semantic_state",
                    "window_control_inspector_options_mutate_position_size_controls_and_visibility_semantic_state",
                    "accordion_inspector_options_mutate_controlled_trigger_and_motion_semantic_state",
                    "shortcut_combo.platform_display=MacOS",
                    "skeleton_cluster.reduced_motion=true",
                    "window_control.visibility=Hover",
                    "accordion.trigger_area=full-row",
                    "assert_runtime_structured_state",
                    "expected_action(page)",
                    "runtime_settings_changed",
                    "runtime_structured.shortcut_combo",
                    "runtime_structured.skeleton_cluster",
                    "runtime_structured.window_control",
                    "runtime_structured.accordion",
                    "platform_display_macos",
                    "reduced_motion",
                    "visibility_hover",
                    "trigger_area_full_row",
                ),
            ),
            (
                "live component option semantic state",
                "visual_interaction_live_component_options_tests.rs",
                (
                    "live_component_inspector_options_mutate_array_and_drag_semantic_state",
                    "assert_live_component_runtime",
                    "state.screen_state.last_event",
                    "dynamic_array.item_count()",
                    "dynamic_array.order_label()",
                    "drag_and_drop.is_dragging()",
                    "drag_and_drop.committed()",
                    "array.order=2,1,3",
                    "array.theme_row=accent",
                    "drag.drop_indicator=after",
                    "drag.keyboard_draggable=true",
                ),
            ),
            (
                "diagnostics-list option semantic state",
                "visual_interaction_diagnostics_list_options_tests.rs",
                (
                    "diagnostics_list_inspector_options_mutate_filter_bulk_and_fix_preview_semantic_state",
                    "diagnostics.virtualization=Windowed",
                    "diagnostics.bulk_action=Apply",
                    "diagnostics.fix_preview=Collapsed",
                    "assert_diagnostics_list_runtime",
                    "option_state()",
                    "settings_diagnostics_option",
                    "molecule_settings_changed",
                    "state.screen_state.last_setting_value",
                    "options.group_by_source",
                    "options.sort_by_location",
                    "options.severity_filter_error_only",
                    "options.wrap_error_navigation_disabled",
                    "options.virtualization_windowed",
                    "options.bulk_action_apply",
                    "options.fix_preview_collapsed",
                ),
            ),
        )
        failures: list[str] = []
        if not semantic_source:
            failures.append("Storybook option semantic state mapper is missing")
        else:
            for token in self.required_option_semantic_tokens():
                if token not in semantic_source:
                    failures.append(
                        f"Storybook option semantic state mapper missing token: {token}"
                    )
        if "semantic_setting_state(page, option)" not in screen_state_source:
            failures.append("Storybook settings registration must use semantic option state")
        for label, file_name, tokens in contracts:
            source = self.read_optional(
                f"crates/katana-ui-core-storybook/src/visual/{file_name}"
            )
            if file_name == "visual_interaction_runtime_structured_options_tests.rs":
                source = (
                    f"{source}\n"
                    f"{self.read_optional('crates/katana-ui-core-storybook/src/visual/visual_interaction_runtime_structured_assertions.rs')}"
                )
            if not source:
                failures.append(f"{label} test is missing")
                continue
            failures.extend(
                f"{label} test missing token: {token}"
                for token in tokens
                if token not in source
            )
            module = file_name.removesuffix(".rs")
            if module not in visual_mod:
                failures.append(f"{label} test is not wired into visual/mod.rs")
        return failures

    def semantic_state_sources(self) -> str:
        visual_root = self.root / "crates/katana-ui-core-storybook/src/visual"
        sources: list[str] = []
        for path in sorted(visual_root.glob("screen_state_setting_semantics*.rs")):
            sources.append(path.read_text(encoding="utf-8"))
        return "\n".join(sources)

    @staticmethod
    def required_option_semantic_tokens() -> tuple[str, ...]:
        return (
            "semantic_setting_state",
            '"toolbar"',
            '"settings-list"',
            '"color-picker-rgba"',
            '"theme-tokens"',
            '"text"',
            '"key-cap"',
            '"motion"',
            '"skeleton"',
            '"loading-dots"',
            '"spinner"',
            '"progress-bar"',
            '"split-pane"',
            '"scroll-area"',
            '"align-center"',
            '"divider"',
            '"spacer"',
            '"color-swatch"',
            '"slide-control"',
            '"checkbox"',
            '"radio"',
            '"toggle"',
            '"segmented-toggle"',
            '"icon"',
            '"text-input"',
            '"text-area"',
            '"badge"',
            '"banner"',
            '"card"',
            '"empty-state"',
            '"toast-stack-manager"',
            '"notification-toast"',
            '"hover-card"',
            '"menu"',
            '"form-field"',
            '"breadcrumb"',
            '"side-menu"',
            '"list"',
            '"collapsible-panel"',
            '"tree-view"',
            '"panel"',
            '"virtualization"',
            '"search-control-strip"',
            '"status-bar"',
            '"chip"',
            '"attachment-chip"',
            '"chip-group"',
            '"command-palette"',
            '"shortcut-cheatsheet"',
            '"context-menu"',
            '"startup-state-panel"',
            '"code-diff"',
            '"shortcut-combo"',
            '"skeleton-cluster"',
            '"window-control-button-group"',
            '"accordion"',
            '"tooltip"',
            '"popover"',
            '"modal"',
            '"modal-overlay"',
            '"diagnostics-list"',
            '"dynamic-array-editor"',
            '"drag-and-drop"',
            '"combo-box"',
            '"select-box"',
            '"selection-list"',
            '"menu-button"',
            '"search-box"',
            "toolbar.action.disabled=true",
            "settings_list.control.options=4",
            "settings_list.label=Workspace settings",
            "settings_list.dirty=Highlight",
            "settings_list.section.description=visible",
            "settings_list.field.label=Font size",
            "settings_list.control.kind=Number",
            "color_picker.eyedropper=storybook-eyedropper",
            "color_picker.rgba=rgba(64,128,255,.8)",
            "color_picker.color_area=saturation/value",
            "color_picker.trigger.border=false",
            "text.script=jp+emoji",
            "skeleton.aspect_ratio=16:9",
            "progress_bar.percent=82",
            "loading_dots.dot_count=5",
            "spinner.animation_state=Paused",
            "theme.color.accent=green",
            "key_cap.theme.color=accent",
            "motion.reduced_policy=ForceReduced",
            "split_pane.resize_mode=KeyboardOnly",
            "scroll_area.overflow=scroll",
            "align_center.alignment=center",
            "divider.variant=alternate",
            "color_swatch.tone=accent",
            "slide_control.theme.slot=custom",
            "checkbox.checked=true",
            "radio.focus=visible",
            "toggle.disabled=true",
            "segmented_toggle.selected=true",
            "icon.svg_source=custom-svg",
            "text_input.value=typed 日本語",
            "text_area.resize_enabled=true",
            "badge.leading_icon=dot",
            "banner.leading_icon=custom",
            "toast_stack.duration=custom",
            "notification_toast.action=visible",
            "list.virtualization=visible_range",
            "collapsible_panel.resize_handle=true",
            "hover_card.pointer_follow=true",
            "panel.horizontal_scroll=changed",
            "menu.panel_placement=resolved",
            "form_field.helper_text=long",
            "breadcrumb.crumb_action=callback",
            "side_menu.hover_expansion=true",
            "tree.context_menu=enabled",
            "tooltip.open=true",
            "popover.placement=edge",
            "modal.focus=first",
            "modal_overlay.dismiss=outside",
            "card.child_state=changed",
            "empty_state.actions=Primary+Secondary",
            "virtualization.viewport.offset=1260",
            "virtualization.overscan=4",
            "search_control.regex=true",
            "search_control.query=heading",
            "search_control.result_count=0",
            "status_bar.segment_a11y=custom",
            "chip.a11y_label=Filter chip",
            "attachment.retry=visible",
            "chip_group.overflow_trigger_width=32",
            "command_palette.provider_group=workspace/editor/app",
            "shortcut_cheatsheet.query=カテゴリ",
            "context_menu.placement_used=AboveEnd",
            "startup_state.retry=true",
            "code_diff.scroll_sync=false",
            "shortcut_combo.platform_display=MacOS",
            "skeleton_cluster.reduced_motion=true",
            "window_control.visibility=Hover",
            "accordion.trigger_area=full-row",
            "diagnostics.bulk_action=Apply",
            "array.order=2,1,3",
            "array.theme_row=accent",
            "drag.drop_indicator=after",
            "drag.keyboard_draggable=true",
            "combo.outside_click_dismiss=true",
            "selection_list.more_row=true",
            "menu_button.select_action=callback",
            "search_box.regex_case=true/true",
        )

    @staticmethod
    def required_inspector_option_contract_tokens() -> tuple[str, ...]:
        return (
            "inspector_settings_rows_include_every_option_contract_for_each_story",
            "inspector_setting_rows_apply_each_clicked_option_contract",
            "inspector_setting_rows_repaint_preview_for_each_clicked_option_contract",
            "button_option_controls_match_storybook_option_contract",
            "button_inspector_controls_apply_each_button_option_contract",
            "button_inspector_rows_select_matching_preset_tabs",
            "button_options::preset_index_for_control",
            "inspector_rows_select_preset_tabs_for_every_non_button_option",
            "expected_preset_index(page, option.setting, option_index)",
            "inspector_rows_select_matching_preset_tabs_for_option_focused_pages",
            "required_page_inspector_options_do_not_use_generic_fallback_status",
            "assert_not_generic_inspector_fallback",
            "state.preset_index",
            "text_area.vertical_scrollbar_visible",
            "text_area.horizontal_scrollbar_visible",
            "text_area.leading_slot.icon",
            "text_area.trailing_icon_buttons",
            "text_area.clear_action",
            "tabs.active_scroll",
            "StoryCatalog.examples()",
            "storybook_ui_option_contract::options_for_page",
            "inspector_rows::settings_rows",
            "apply_click(&mut state",
            "option.after",
            "option.setting, state.screen_state.state_label",
            "setting_is_visible",
            "ROW_MAX_CHARS",
            "StorybookButtonOptionControl::all()",
            "button_options::control_rect",
            "control.setting_name()",
        )

    def read_optional(self, relative: str) -> str:
        path = self.root / relative
        if not path.exists():
            return ""
        return path.read_text(encoding="utf-8")
