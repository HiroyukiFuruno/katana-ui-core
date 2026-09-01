#!/usr/bin/env python3
import tempfile
import unittest
from pathlib import Path

from kuc_guardrails import KucGuardrails
from kuc_workspace_tab_guardrails import WorkspaceTabGuardrails
from storybook_ui_harness_interaction_assertions import StorybookUiInteractionHarness


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


def write_repo_policy(root: Path, spec_extra: str = "") -> None:
    baseline = "KUC repo `scripts/` `kal` 側\n"
    spec = (
        "KUC-specific guards MUST live in this repository\n"
        "Storybook is an interactive feedback surface\n"
        f"{spec_extra}"
    )
    paths = (
        "docs/architecture/ui-separation/owned-ui-task-map.md",
        "openspec/changes/establish-kuc-atoms-molecules-catalog/quality-gates-contract.md",
    )
    for path in paths:
        write_text(root / path, baseline)
    write_text(
        root
        / "openspec/changes/establish-kuc-atoms-molecules-catalog/specs/kuc-quality-gates/spec.md",
        spec,
    )


def write_generic_ui_contract(root: Path) -> None:
    source = (
        "汎用 Rust app\n"
        "KUC 本体は Katana を知ってはならない\n"
        "Katana は参照実装\n"
        "外部から渡される `svg_source`\n"
        "framework 固有依存を排除\n"
    )
    write_text(
        root / "openspec/changes/establish-kuc-atoms-molecules-catalog/quality-gates-contract.md",
        source,
    )
    write_text(
        root / "crates/katana-ui-core/tests/generic_rust_app_contract.rs",
        "fn generic_rust_app_can_compose_shell_from_public_kuc_api() {}\n"
        "fn generic_app_inputs_keep_internal_state_per_instance() {}\n"
        "fn generic_app_readonly_input_rejects_write_actions() {}\n"
        "fn generic_app_readonly_input_allows_selection_without_write_mutation() {}\n"
        "fn generic_app_readonly_text_area_allows_selection_and_submit_without_write_mutation() {}\n"
        "fn generic_app_tabs_support_add_close_move_group_and_pin_contracts() {}\n",
    )
    write_text(
        root / "crates/katana-ui-core/tests/generic_rust_app_layout_contract.rs",
        "fn generic_app_can_build_resizable_scrollable_layout_from_public_kuc_api() {}\n"
        "fn generic_app_scroll_area_uses_typed_public_action_and_state() {}\n"
        "fn generic_app_split_pane_uses_typed_public_action_and_state() {}\n"
        "fn generic_app_facade_exposes_theme_state_and_render_context() {}\n",
    )
    write_text(
        root / "crates/katana-ui-core/tests/generic_rust_app_action_contract.rs",
        "fn generic_app_input_icon_button_invokes_callback_without_mutating_text() {}\n"
        "fn generic_app_disabled_input_blocks_icon_button_callback() {}\n"
        "fn generic_adapter_dispatch_targets_stable_state_id_after_redraw() {}\n"
        "fn generic_adapter_dispatches_closeable_tab_typed_actions() {}\n"
        "fn generic_adapter_dispatches_closeable_tab_context_bulk_actions() {}\n"
        "fn generic_adapter_dispatches_closeable_tab_add_and_group_actions() {}\n"
        "fn generic_adapter_dispatches_closeable_tab_typed_event_log() {}\n"
        "fn generic_adapter_dispatches_closeable_tab_visual_index_selection() {}\n"
        "fn generic_app_tabs_support_bulk_context_actions_from_public_api() {}\n"
        "fn generic_app_tabs_context_commands_map_to_typed_actions() {}\n"
        "CloseableTabContextMenu::menu\n"
        "CloseableTabContextCommand::from_id\n"
        "CloseableTabGroupContextCommand::from_id\n"
        "to_group_action\n"
        "ContextMenuItem::action\n"
        "fn generic_app_tabs_emit_typed_events_for_pin_and_group_changes() {}\n",
    )
    write_text(
        root / "examples/kuc-consumer-app/src/lib.rs",
        "quick_search: SearchBox\n"
        "workspace_select: SelectBox\n"
        "symbol_combo: ComboBox\n",
    )
    write_text(
        root / "examples/kuc-consumer-app/src/fixtures.rs",
        "SearchBox::new\n"
        "SelectBox::new\n"
        "ComboBox::new\n"
        ".submit_on_enter(true)\n"
        ".free_input(true)\n",
    )
    write_text(
        root / "examples/kuc-consumer-app/src/actions.rs",
        "fn set_quick_search() { UiAction::search_submitted; }\n"
        "fn select_workspace() { UiAction::select_box_selected; }\n"
        "fn select_symbol() {}\n",
    )
    write_text(
        root / "examples/kuc-consumer-app/src/queries.rs",
        "fn quick_search_query() {}\n"
        "fn workspace_value() {}\n"
        "fn symbol_value() {}\n",
    )
    write_text(
        root / "examples/kuc-consumer-app/src/tests.rs",
        "UiNodeKind::SearchBox\n"
        "UiNodeKind::SelectBox\n"
        "UiNodeKind::ComboBox\n"
        "quick_search_log[0].action\n"
        "workspace_value\n"
        "symbol_value\n",
    )


def write_storybook_live_component_contract(root: Path) -> None:
    source = (
        "Storybook は絵ではない\n"
        "KUC の実部品\n"
        "props / state / event / action / callback\n"
        "replay surface\n"
    )
    write_text(
        root
        / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-catalog-contract.md",
        source,
    )


class KucGuardrailsTest(unittest.TestCase):
    def test_rejects_format_or_framework_tokens_in_generic_grid(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            grid = root / "crates/katana-ui-core/src/molecule/generic_grid"
            write_text(grid / "mod.rs", "struct Grid { formula: String }\n")
            write_text(
                root / "crates/katana-ui-core/src/render_model/typed_grid.rs",
                "struct Props;\n",
            )

            failures = KucGuardrails(root).generic_grid_boundary_failures()

            self.assertTrue(
                any("forbidden format or framework token `formula`" in it for it in failures),
                failures,
            )

    def test_accepts_neutral_generic_grid_boundary(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            grid = root / "crates/katana-ui-core/src/molecule/generic_grid"
            required = (
                grid / "mod.rs",
                grid / "axis.rs",
                grid / "axis_types.rs",
                grid / "component.rs",
                grid / "component_types.rs",
                grid / "geometry.rs",
                grid / "selection.rs",
                root / "crates/katana-ui-core/src/render_model/typed_grid.rs",
                root / "crates/katana-ui-core/src/render_model/typed_grid_types.rs",
                root / "crates/katana-ui-core/tests/generic_grid_axis_contract.rs",
                root / "crates/katana-ui-core/tests/generic_grid_component_contract.rs",
                root / "examples/kuc-consumer-app/tests/generic_public_contract.rs",
            )
            for path in required:
                write_text(path, "struct NeutralGrid;\n")
            write_text(root / "Cargo.toml", "[workspace]\n")
            write_text(root / "crates/katana-ui-core/Cargo.toml", "[dependencies]\n")

            failures = KucGuardrails(root).generic_grid_boundary_failures()

            self.assertEqual([], failures)

    def test_allows_optional_egui_dependency_outside_neutral_grid_module(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            grid = root / "crates/katana-ui-core/src/molecule/generic_grid"
            required = (
                grid / "mod.rs",
                grid / "axis.rs",
                grid / "axis_types.rs",
                grid / "component.rs",
                grid / "component_types.rs",
                grid / "geometry.rs",
                grid / "selection.rs",
                root / "crates/katana-ui-core/src/render_model/typed_grid.rs",
                root / "crates/katana-ui-core/src/render_model/typed_grid_types.rs",
                root / "crates/katana-ui-core/tests/generic_grid_axis_contract.rs",
                root / "crates/katana-ui-core/tests/generic_grid_component_contract.rs",
                root / "examples/kuc-consumer-app/tests/generic_public_contract.rs",
            )
            for path in required:
                write_text(path, "struct NeutralGrid;\n")
            write_text(
                root / "crates/katana-ui-core/Cargo.toml",
                "[dependencies]\negui = { workspace = true, optional = true }\n",
            )

            failures = KucGuardrails(root).generic_grid_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_preset_tab_scroll_clip_hit_guard_gap(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(base / "preset_tab_scroll.rs", "fn max_scroll_x_for_page() {}\n")
            write_text(base / "preset_tabs.rs", "fn draw() { canvas.with_clip(); }\n")
            write_text(
                base / "visual_preset_tab_scroll_tests.rs",
                "fn overflowing_preset_tabs_have_horizontal_scroll_range() {}\n",
            )
            write_text(base / "mod.rs", "")

            failures = (
                StorybookUiInteractionHarness(root)
                .preset_tab_scroll_clip_hit_contract_failures()
            )

            self.assertTrue(
                any("overflow scroll contract missing token: scroll_delta" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("current follow contract missing token: ensure_index_visible" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("clipping contract missing token: preset_tab_label::fit" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("label fitting contract is missing" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("hit bounds contract missing token: hit_index_at" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("scroll guard tests missing token: wheel_over_preset_tabs_scrolls_tabs_without_scrolling_root" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("scroll guard wiring is missing" in it for it in failures),
                failures,
            )

    def test_accepts_preset_tab_scroll_clip_hit_guard_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "preset_tab_scroll.rs",
                "fn max_scroll_x_for_page() {}\n"
                "fn scroll_delta() { clamp_offset(); viewport_rect(); visible_index_range(); }\n"
                "fn ensure_index_visible() {\n"
                "    if tab_left < offset {}\n"
                "    if tab_right > offset + viewport_width() {}\n"
                "}\n"
                "fn active_index_scroll_x() {}\n"
                "fn hit_index_at() {\n"
                "    viewport.contains(x, y);\n"
                "    visual_rect_for_index(page, index, false, scroll_x);\n"
                "    rect.contains(x, y);\n"
                "}\n",
            )
            write_text(
                base / "preset_tabs.rs",
                "fn draw() { canvas.with_clip(); preset_tab_label::fit(); }\n",
            )
            write_text(
                base / "preset_tab_label.rs",
                "fn fit() { measure_width(); }\n"
                "const TRUNCATION_MARKER: &str = \"...\";\n"
                "fn measured_width_for_test() {}\n",
            )
            write_text(
                base / "visual_preset_tab_scroll_tests.rs",
                "fn overflowing_preset_tabs_have_horizontal_scroll_range() {}\n"
                "fn visible_preset_tab_rects_stay_fully_inside_viewport() {}\n"
                "fn rendered_preset_tabs_are_clipped_at_preview_right_edge() {\n"
                "    pixel_at(&canvas);\n"
                "}\n"
                "fn external_preset_selection_scrolls_current_tab_into_view() {\n"
                "    state.select_preset(last_preset);\n"
                "    active_tab_is_inside_viewport();\n"
                "}\n"
                "fn clicking_scrolled_preset_tab_uses_logical_tab_index() {}\n"
                "fn wheel_over_preset_tabs_scrolls_tabs_without_scrolling_root() {\n"
                "    apply_scroll_delta_at_for_test();\n"
                "    state.scroll_y;\n"
                "}\n"
                "fn external_render_preset_scrolls_active_overflow_tab_into_view() {}\n",
            )
            write_text(base / "mod.rs", "mod visual_preset_tab_scroll_tests;\n")

            failures = (
                StorybookUiInteractionHarness(root)
                .preset_tab_scroll_clip_hit_contract_failures()
            )

            self.assertEqual([], failures)

    def test_rejects_option_semantic_state_guard_gap(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(base / "screen_state_setting_semantics.rs", "fn semantic_setting_state() {}\n")
            write_text(base / "screen_state.rs", "fn register() {}\n")
            write_text(base / "visual_interaction_toolbar_options_tests.rs", "fn test() {}\n")
            write_text(base / "mod.rs", "")

            failures = (
                StorybookUiInteractionHarness(root)
                .option_semantic_state_contract_failures()
            )

            self.assertTrue(
                any("semantic state mapper missing token" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("settings registration must use semantic option state" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("color-picker option semantic state test is missing" in it for it in failures),
                failures,
            )

    def test_accepts_option_semantic_state_guard_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "screen_state_setting_semantics.rs",
                "fn semantic_setting_state() {\n"
                "    \"toolbar\"; \"settings-list\"; \"color-picker-rgba\";\n"
                "    \"theme-tokens\";\n"
                "    \"text\"; \"skeleton\"; \"loading-dots\"; \"spinner\";\n"
                "    \"key-cap\"; \"motion\";\n"
                "    \"progress-bar\";\n"
                "    \"split-pane\";\n"
                "    \"scroll-area\"; \"align-center\";\n"
                "    \"divider\"; \"spacer\"; \"color-swatch\"; \"slide-control\";\n"
                "    \"checkbox\"; \"radio\"; \"toggle\"; \"segmented-toggle\";\n"
                "    \"icon\";\n"
                "    \"text-input\"; \"text-area\";\n"
                "    \"badge\"; \"banner\"; \"card\"; \"empty-state\";\n"
                "    \"toast-stack-manager\"; \"notification-toast\";\n"
                "    \"hover-card\"; \"menu\"; \"form-field\"; \"breadcrumb\";\n"
                "    \"side-menu\"; \"list\"; \"collapsible-panel\"; \"tree-view\";\n"
                "    \"panel\";\n"
                "    \"virtualization\"; \"search-control-strip\";\n"
                "    \"status-bar\";\n"
                "    \"chip\";\n"
                "    \"attachment-chip\"; \"chip-group\";\n"
                "    \"command-palette\";\n"
                "    \"shortcut-cheatsheet\";\n"
                "    \"context-menu\"; \"startup-state-panel\"; \"code-diff\";\n"
                "    \"shortcut-combo\"; \"skeleton-cluster\";\n"
                "    \"window-control-button-group\"; \"accordion\";\n"
                "    \"tooltip\"; \"popover\"; \"modal\"; \"modal-overlay\";\n"
                "    \"diagnostics-list\";\n"
                "    \"dynamic-array-editor\"; \"drag-and-drop\";\n"
                "    \"combo-box\"; \"select-box\"; \"selection-list\";\n"
                "    \"menu-button\"; \"search-box\";\n"
                "    toolbar.action.disabled=true;\n"
                "    settings_list.control.options=4;\n"
                "    settings_list.label=Workspace settings;\n"
                "    settings_list.dirty=Highlight;\n"
                "    settings_list.section.description=visible;\n"
                "    settings_list.field.label=Font size;\n"
                "    settings_list.control.kind=Number;\n"
                "    color_picker.eyedropper=storybook-eyedropper;\n"
                "    color_picker.rgba=rgba(64,128,255,.8);\n"
                "    color_picker.color_area=saturation/value;\n"
                "    color_picker.trigger.border=false;\n"
                "    text.script=jp+emoji;\n"
                "    skeleton.aspect_ratio=16:9;\n"
                "    progress_bar.percent=82;\n"
                "    loading_dots.dot_count=5;\n"
                "    spinner.animation_state=Paused;\n"
                "    theme.color.accent=green;\n"
                "    key_cap.theme.color=accent;\n"
                "    motion.reduced_policy=ForceReduced;\n"
                "    split_pane.resize_mode=KeyboardOnly;\n"
                "    scroll_area.overflow=scroll;\n"
                "    align_center.alignment=center;\n"
                "    divider.variant=alternate;\n"
                "    color_swatch.tone=accent;\n"
                "    slide_control.theme.slot=custom;\n"
                "    checkbox.checked=true;\n"
                "    radio.focus=visible;\n"
                "    toggle.disabled=true;\n"
                "    segmented_toggle.selected=true;\n"
                "    icon.svg_source=custom-svg;\n"
                "    icon.paint_policy=currentColor;\n"
                "    icon.theme_token=muted;\n"
                "    text_input.value=typed 日本語;\n"
                "    text_area.resize_enabled=true;\n"
                "    badge.leading_icon=dot;\n"
                "    banner.leading_icon=custom;\n"
                "    toast_stack.duration=custom;\n"
                "    notification_toast.action=visible;\n"
                "    list.virtualization=visible_range;\n"
                "    collapsible_panel.resize_handle=true;\n"
                "    hover_card.pointer_follow=true;\n"
                "    panel.horizontal_scroll=changed;\n"
                "    panel.nested_state=independent;\n"
                "    menu.selected_index=1;\n"
                "    menu.panel_placement=resolved;\n"
                "    form_field.helper_text=long;\n"
                "    breadcrumb.crumb_action=callback;\n"
                "    side_menu.hover_expansion=true;\n"
                "    tree.context_menu=enabled;\n"
                "    tooltip.open=true;\n"
                "    popover.placement=edge;\n"
                "    modal.focus=first;\n"
                "    modal_overlay.dismiss=outside;\n"
                "    card.child_state=changed;\n"
                "    empty_state.actions=Primary+Secondary;\n"
                "    virtualization.viewport.offset=1260;\n"
                "    virtualization.overscan=4;\n"
                "    search_control.regex=true;\n"
                "    search_control.query=heading;\n"
                "    search_control.result_count=0;\n"
                "    status_bar.segment_a11y=custom;\n"
                "    chip.a11y_label=Filter chip;\n"
                "    attachment.retry=visible;\n"
                "    chip_group.overflow_trigger_width=32;\n"
                "    command_palette.provider_group=workspace/editor/app;\n"
                "    shortcut_cheatsheet.query=カテゴリ;\n"
                "    context_menu.placement_used=AboveEnd;\n"
                "    startup_state.retry=true;\n"
                "    code_diff.scroll_sync=false;\n"
                "    shortcut_combo.platform_display=MacOS;\n"
                "    skeleton_cluster.reduced_motion=true;\n"
                "    window_control.visibility=Hover;\n"
                "    accordion.trigger_area=full-row;\n"
                "    diagnostics.bulk_action=Apply;\n"
                "    array.order=2,1,3;\n"
                "    array.theme_row=accent;\n"
                "    drag.drop_indicator=after;\n"
                "    drag.keyboard_draggable=true;\n"
                "    combo.outside_click_dismiss=true;\n"
                "    selection_list.more_row=true;\n"
                "    menu_button.select_action=callback;\n"
                "    search_box.regex_case=true/true;\n"
                "}\n",
            )
            write_text(
                base / "screen_state.rs",
                "fn register() { semantic_setting_state(page, option); }\n",
            )
            write_text(
                base / "visual_interaction_text_entry_options_tests.rs",
                "fn text_input_inspector_options_mutate_value_slot_icon_and_blocking_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    text_input.leading_slot.icon=search-svg;\n"
                "}\n"
                "fn text_area_inspector_options_mutate_multiline_scroll_slot_and_blocking_semantic_state() {\n"
                "    text_area.horizontal_scrollbar_visible=true;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_foundation_options_tests.rs",
                "fn text_inspector_options_mutate_role_script_metrics_and_wrap_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    text.script=jp+emoji;\n"
                "}\n"
                "fn progress_bar_inspector_options_mutate_progress_loading_tone_and_size_semantic_state() {\n"
                "    progress_bar.percent=82;\n"
                "}\n"
                "fn loading_indicator_inspector_options_mutate_animation_label_tone_and_size_semantic_state() {\n"
                "    loading_dots.dot_count=5;\n"
                "    spinner.animation_state=Paused;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_foundation_extra_options_tests.rs",
                "fn foundation_extra_inspector_options_mutate_theme_key_cap_and_motion_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    theme.color.accent=green;\n"
                "    key_cap.theme.color=accent;\n"
                "    motion.reduced_policy=ForceReduced;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_skeleton_options_tests.rs",
                "fn skeleton_inspector_options_mutate_shape_motion_size_and_a11y_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    skeleton.line_thickness=12;\n"
                "    skeleton.reduced_motion=true;\n"
                "    skeleton.aspect_ratio=16:9;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_split_pane_options_tests.rs",
                "fn split_pane_inspector_options_mutate_axis_ratio_bounds_and_resize_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    split_pane.ratio_percent=64;\n"
                "    split_pane.handle_width_px=10;\n"
                "    split_pane.resize_mode=KeyboardOnly;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_layout_options_tests.rs",
                "fn layout_inspector_options_mutate_axis_gap_alignment_and_overflow_semantic_state() {\n"
                "    assert_layout_option_state;\n"
                "    assert_inspector_option_state_with_event;\n"
                "    layout_option_changed;\n"
                "    row.alignment=center;\n"
                "    grid.overflow=scroll;\n"
                "    scroll_area.overflow=scroll;\n"
                "    align_center.alignment=center;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_primitive_options_tests.rs",
                "fn primitive_inspector_options_mutate_variant_tone_size_and_theme_slot_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    divider.variant=alternate;\n"
                "    color_swatch.tone=accent;\n"
                "    slide_control.theme.slot=custom;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_binary_choice_options_tests.rs",
                "fn binary_choice_inspector_options_mutate_selected_disabled_focus_and_checked_semantic_state() {\n"
                "    checkbox.checked=true;\n"
                "    radio.focus=visible;\n"
                "    toggle.disabled=true;\n"
                "    segmented_toggle.selected=true;\n"
                "    binary_choice_disabled_option_blocks_preview_mutation;\n"
                "    assert_component_state(page, setting, &state.screen_state);\n"
                "    checkbox_state_snapshot;\n"
                "    radio_state_snapshot;\n"
                "    assert_binary_component_state;\n"
                "    state.common.disabled;\n"
                "    state.interaction.focused;\n"
                "    settings_disabled;\n"
                "    selection_settings_changed;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_icon_options_tests.rs",
                "fn icon_inspector_options_mutate_svg_source_role_paint_and_token_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    icon.svg_source=custom-svg;\n"
                "    icon.paint_policy=currentColor;\n"
                "    icon.theme_token=muted;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_options_tests.rs",
                "fn closeable_tab_strip_inspector_options_mutate_active_overflow_pin_and_group_semantic_state() {\n"
                "    tabs.active=settings;\n"
                "    tabs.pinned=true left-fixed;\n"
                "    tabs.group=Docs;\n"
                "    tabs.overflow=menu;\n"
                "    assert_closeable_tab_event;\n"
                "    state.screen_state.last_action;\n"
                "    state.screen_state.last_event;\n"
                "    starts_with(\"closeable_tab\");\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_options_tests.rs",
                "fn tabs_inspector_options_mutate_tab_model_state() {\n"
                "    tabs.count=6 active=notes.md;\n"
                "    tabs.pinned=true left-fixed;\n"
                "    tabs.group=Docs;\n"
                "    tabs.overflow=menu;\n"
                "    tabs.active_scroll=follow;\n"
                "    assert_tabs_option_event;\n"
                "    state.screen_state.last_action;\n"
                "    state.screen_state.last_event;\n"
                "    starts_with(\"closeable_tab\");\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_surface_options_tests.rs",
                "fn badge_inspector_options_mutate_status_size_icon_and_variant_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    badge.leading_icon=dot;\n"
                "}\n"
                "fn card_inspector_options_mutate_slot_click_and_child_semantic_state() {\n"
                "    card.child_state=changed;\n"
                "}\n"
                "fn empty_state_inspector_options_mutate_content_alignment_and_action_semantic_state() {\n"
                "    empty_state.actions=Primary+Secondary;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_feedback_options_tests.rs",
                "fn feedback_inspector_options_mutate_severity_duration_action_and_dismiss_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    toast_stack.duration=custom;\n"
                "    notification_toast.action=visible;\n"
                "    notification_toast.dismiss=true;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_collection_options_tests.rs",
                "fn collection_inspector_options_mutate_list_collapsible_hover_and_panel_semantic_state() {\n"
                "    assert_collection_option_state;\n"
                "    assert_inspector_option_state_with_event;\n"
                "    panel_active_select;\n"
                "    panel_scrollbar_hide;\n"
                "    list.virtualization=visible_range;\n"
                "    collapsible_panel.resize_handle=true;\n"
                "    hover_card.pointer_follow=true;\n"
                "    panel.horizontal_scroll=changed;\n"
                "    panel.nested_state=independent;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_navigation_options_tests.rs",
                "fn navigation_inspector_options_mutate_menu_form_breadcrumb_side_and_tree_semantic_state() {\n"
                "    assert_navigation_option_state;\n"
                "    breadcrumb_click;\n"
                "    field_validate;\n"
                "    form_field_helper_text;\n"
                "    menu.selected_index=1;\n"
                "    menu.panel_placement=resolved;\n"
                "    form_field.helper_text=long;\n"
                "    breadcrumb.crumb_action=callback;\n"
                "    side_menu.hover_expansion=true;\n"
                "    tree.context_menu=enabled;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_banner_options_tests.rs",
                "fn banner_inspector_options_mutate_feedback_details_icon_and_placement_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    banner.severity=warning;\n"
                "    banner.details=expanded;\n"
                "    banner.leading_icon=custom;\n"
                "    banner.placement=sticky;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_overlay_options_tests.rs",
                "fn tooltip_inspector_options_mutate_overlay_semantic_state() {\n"
                "    assert_inspector_option_state;\n"
                "    tooltip.open=true;\n"
                "}\n"
                "fn popover_inspector_options_mutate_overlay_semantic_state() {\n"
                "    popover.placement=edge;\n"
                "}\n"
                "fn modal_inspector_options_mutate_overlay_semantic_state() {\n"
                "    modal.focus=first;\n"
                "}\n"
                "fn modal_overlay_inspector_options_mutate_overlay_semantic_state() {\n"
                "    modal_overlay.dismiss=outside;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_toolbar_options_tests.rs",
                "fn toolbar_inspector_options_mutate_action_split_and_group_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    toolbar.action.disabled=true;\n"
                "    toolbar.split.a11y=Open menu;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_settings_list_options_tests.rs",
                "fn settings_list_inspector_options_mutate_field_control_and_reset_semantic_state() {\n"
                "    settings_list.control.options=4;\n"
                "    settings_list.label=Workspace settings;\n"
                "    settings_list.dirty=Highlight;\n"
                "    settings_list.section.description=visible;\n"
                "    settings_list.field.label=Font size;\n"
                "    settings_list.control.kind=Number;\n"
                "    settings_list.reset=default;\n"
                "    assert_settings_list_runtime;\n"
                "    option_state();\n"
                "    options.label_workspace;\n"
                "    options.density_compact;\n"
                "    options.dirty_highlight;\n"
                "    options.sections_app_lint;\n"
                "    options.section_label_editor;\n"
                "    options.section_description_visible;\n"
                "    options.section_icon_gear;\n"
                "    options.field_count;\n"
                "    options.section_footer_policy;\n"
                "    options.section_collapsible;\n"
                "    options.default_collapsed;\n"
                "    options.field_label_font_size;\n"
                "    options.field_description_visible;\n"
                "    options.control_kind_number;\n"
                "    options.control_option_count;\n"
                "    options.custom_control_button;\n"
                "    options.value_changed;\n"
                "    options.reset_default;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_color_picker_options_tests.rs",
                "fn color_picker_inspector_options_mutate_hue_alpha_block_and_callback_semantic_state() {\n"
                "    color_picker.rgba=rgba(64,128,255,.8);\n"
                "    color_picker.color_area=saturation/value;\n"
                "    color_picker.trigger.border=false;\n"
                "    color_picker.readonly.blocks_writes;\n"
                "    color_picker.disabled.blocks_focus;\n"
                "    assert_color_picker_runtime;\n"
                "    option_state();\n"
                "    options.panel_open;\n"
                "    options.blending_multiply;\n"
                "    options.color_area_visible;\n"
                "    options.trigger_large;\n"
                "    options.title_customized;\n"
                "    options.panel_scale_percent;\n"
                "    options.trigger_border;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_virtualization_options_tests.rs",
                "fn virtualization_inspector_options_mutate_range_focus_and_measurement_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    virtualization.focused_index=42;\n"
                "    virtualization.measured_correction=+8;\n"
                "    virtualization.overscan=4;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_search_control_options_tests.rs",
                "fn search_control_inspector_options_mutate_match_replace_and_active_result_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    search_control.query=heading;\n"
                "    search_control.match_case=true;\n"
                "    search_control.result_count=0;\n"
                "    search_control.active_index=none;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_status_bar_options_tests.rs",
                "fn status_bar_inspector_options_mutate_segment_and_message_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    status_bar.progress_popover=true;\n"
                "    status_bar.segment_a11y=custom;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_chip_options_tests.rs",
                "fn chip_inspector_options_mutate_label_icon_variant_and_state_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    chip.leading_icon=tag;\n"
                "    chip.a11y_label=Filter chip;\n"
                "    chip.focused=true;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_chip_family_options_tests.rs",
                "fn attachment_chip_inspector_options_mutate_kind_status_and_retry_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    attachment.retry=visible;\n"
                "}\n"
                "fn chip_group_inspector_options_mutate_overflow_reorder_and_width_semantic_state() {\n"
                "    chip_group.overflow_trigger_width=32;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_command_palette_options_tests.rs",
                "fn command_palette_inspector_options_mutate_query_highlight_provider_semantic_state() {\n"
                "    command_palette.query=theme;\n"
                "    command_palette.highlight=2;\n"
                "    command_palette.provider_group=workspace/editor/app;\n"
                "    assert_command_palette_runtime;\n"
                "    option_state();\n"
                "    settings_command_palette_option;\n"
                "    molecule_settings_changed;\n"
                "    state.screen_state.last_setting_value;\n"
                "    command_palette.query();\n"
                "    command_palette.highlighted_index();\n"
                "    options.row_count;\n"
                "    options.provider_group_workspace_editor_app;\n"
                "    options.shortcut_display_visible;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_shortcut_cheatsheet_options_tests.rs",
                "fn shortcut_cheatsheet_inspector_options_mutate_filter_selection_and_count_semantic_state() {\n"
                "    shortcut_cheatsheet.query=カテゴリ;\n"
                "    shortcut_cheatsheet.selected=format;\n"
                "    shortcut_cheatsheet.result_count=1;\n"
                "    assert_shortcut_cheatsheet_runtime;\n"
                "    option_state();\n"
                "    settings_shortcut_cheatsheet_option;\n"
                "    runtime_settings_changed;\n"
                "    state.screen_state.last_setting_value;\n"
                "    options.label_editor_keys;\n"
                "    options.group_count;\n"
                "    options.item_count;\n"
                "    options.group_layout_one_column;\n"
                "    options.query_category;\n"
                "    options.selected_format;\n"
                "    options.result_count;\n"
                "    cheatsheet.visible_item_count();\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_runtime_options_tests.rs",
                "fn context_menu_inspector_options_mutate_anchor_placement_and_size_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    context_menu.placement_used=AboveEnd;\n"
                "}\n"
                "fn startup_state_inspector_options_mutate_error_progress_and_action_semantic_state() {\n"
                "    startup_state.retry=true;\n"
                "}\n"
                "fn code_diff_inspector_options_mutate_mode_layout_and_sync_semantic_state() {\n"
                "    code_diff.scroll_sync=false;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_runtime_structured_options_tests.rs",
                "fn shortcut_combo_inspector_options_mutate_display_size_tone_and_a11y_semantic_state() {\n"
                "    shortcut_combo.platform_display=MacOS;\n"
                "    assert_runtime_structured_state;\n"
                "    expected_action(page);\n"
                "    runtime_settings_changed;\n"
                "    runtime_structured.shortcut_combo;\n"
                "    platform_display_macos;\n"
                "}\n"
                "fn skeleton_cluster_inspector_options_mutate_preset_children_and_motion_semantic_state() {\n"
                "    skeleton_cluster.reduced_motion=true;\n"
                "    runtime_structured.skeleton_cluster;\n"
                "    reduced_motion;\n"
                "}\n"
                "fn window_control_inspector_options_mutate_position_size_controls_and_visibility_semantic_state() {\n"
                "    window_control.visibility=Hover;\n"
                "    runtime_structured.window_control;\n"
                "    visibility_hover;\n"
                "}\n"
                "fn accordion_inspector_options_mutate_controlled_trigger_and_motion_semantic_state() {\n"
                "    accordion.trigger_area=full-row;\n"
                "    runtime_structured.accordion;\n"
                "    trigger_area_full_row;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_live_component_options_tests.rs",
                "fn live_component_inspector_options_mutate_array_and_drag_semantic_state() {\n"
                "    assert_live_component_runtime;\n"
                "    state.screen_state.last_event;\n"
                "    dynamic_array.item_count();\n"
                "    dynamic_array.order_label();\n"
                "    drag_and_drop.is_dragging();\n"
                "    drag_and_drop.committed();\n"
                "    array.order=2,1,3;\n"
                "    array.theme_row=accent;\n"
                "    drag.drop_indicator=after;\n"
                "    drag.keyboard_draggable=true;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_diagnostics_list_options_tests.rs",
                "fn diagnostics_list_inspector_options_mutate_filter_bulk_and_fix_preview_semantic_state() {\n"
                "    diagnostics.virtualization=Windowed;\n"
                "    diagnostics.bulk_action=Apply;\n"
                "    diagnostics.fix_preview=Collapsed;\n"
                "    assert_diagnostics_list_runtime;\n"
                "    option_state();\n"
                "    settings_diagnostics_option;\n"
                "    molecule_settings_changed;\n"
                "    state.screen_state.last_setting_value;\n"
                "    options.group_by_source;\n"
                "    options.sort_by_location;\n"
                "    options.severity_filter_error_only;\n"
                "    options.wrap_error_navigation_disabled;\n"
                "    options.virtualization_windowed;\n"
                "    options.bulk_action_apply;\n"
                "    options.fix_preview_collapsed;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_selection_options_tests.rs",
                "fn combo_box_inspector_options_mutate_choice_semantic_state() {\n"
                "    assert_inspector_option_contract_state;\n"
                "    combo.outside_click_dismiss=true;\n"
                "}\n"
                "fn select_box_inspector_options_mutate_choice_semantic_state() {}\n"
                "fn selection_list_inspector_options_mutate_list_semantic_state() {\n"
                "    selection_list.more_row=true;\n"
                "}\n"
                "fn menu_button_inspector_options_mutate_menu_semantic_state() {\n"
                "    menu_button.select_action=callback;\n"
                "}\n"
                "fn search_box_inspector_options_mutate_search_semantic_state() {\n"
                "    search_box.regex_case=true/true;\n"
                "}\n",
            )
            write_text(
                base / "mod.rs",
                "mod visual_interaction_toolbar_options_tests;\n"
                "mod visual_interaction_text_entry_options_tests;\n"
                "mod visual_interaction_surface_options_tests;\n"
                "mod visual_interaction_foundation_options_tests;\n"
                "mod visual_interaction_foundation_extra_options_tests;\n"
                "mod visual_interaction_skeleton_options_tests;\n"
                "mod visual_interaction_split_pane_options_tests;\n"
                "mod visual_interaction_layout_options_tests;\n"
                "mod visual_interaction_primitive_options_tests;\n"
                "mod visual_interaction_binary_choice_options_tests;\n"
                "mod visual_interaction_banner_options_tests;\n"
                "mod visual_interaction_feedback_options_tests;\n"
                "mod visual_interaction_collection_options_tests;\n"
                "mod visual_interaction_navigation_options_tests;\n"
                "mod visual_interaction_overlay_options_tests;\n"
                "mod visual_interaction_settings_list_options_tests;\n"
                "mod visual_interaction_color_picker_options_tests;\n"
                "mod visual_interaction_virtualization_options_tests;\n"
                "mod visual_interaction_search_control_options_tests;\n"
                "mod visual_interaction_status_bar_options_tests;\n"
                "mod visual_interaction_chip_options_tests;\n"
                "mod visual_interaction_chip_family_options_tests;\n"
                "mod visual_interaction_icon_options_tests;\n"
                "mod visual_interaction_closeable_tab_strip_options_tests;\n"
                "mod visual_interaction_tabs_options_tests;\n"
                "mod visual_interaction_command_palette_options_tests;\n"
                "mod visual_interaction_shortcut_cheatsheet_options_tests;\n"
                "mod visual_interaction_runtime_options_tests;\n"
                "mod visual_interaction_runtime_structured_options_tests;\n"
                "mod visual_interaction_live_component_options_tests;\n"
                "mod visual_interaction_diagnostics_list_options_tests;\n"
                "mod visual_interaction_selection_options_tests;\n",
            )

            failures = (
                StorybookUiInteractionHarness(root)
                .option_semantic_state_contract_failures()
            )

            self.assertEqual([], failures)

    def test_rejects_workspace_tab_visual_order_action_guard_gap(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = (
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar"
            )
            write_text(
                base / "ordering.rs",
                "tabs.iter()\n"
                "    .filter(|tab| tab.pinned)\n"
                "    .chain(tabs.iter().filter(|tab| !tab.pinned && tab.group_id.is_some()))\n"
                "    .chain(tabs.iter().filter(|tab| !tab.pinned && tab.group_id.is_none()))\n",
            )
            write_text(
                base / "bulk_close.rs",
                "fn close_tabs_to_right() {}\nfn close_tabs_to_left() {}\n",
            )
            write_text(base / "actions.rs", "if dragged.pinned {}\n")
            write_text(
                base / "tests/state_action_contract.rs",
                "fn old_visual_order_test() {}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertTrue(
                any("workspace tab visual order missing" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("bulk close must use the shared visual tab order" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("drop rules must keep grouped" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("state/action tests must cover visual order" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("typed group actions" in it for it in failures),
                failures,
            )

    def test_accepts_workspace_tab_visual_order_action_guard_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = (
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar"
            )
            write_text(
                base / "ordering.rs",
                "use super::options::{WorkspaceTab, WorkspaceTabGroup};\n"
                "fn ordered_tabs(tabs: &[WorkspaceTab], groups: &[WorkspaceTabGroup]) {\n"
                "    push_pinned_tabs();\n"
                "    push_group_tabs();\n"
                "    push_unknown_group_tabs();\n"
                "    tabs.iter().filter(|tab| !tab.pinned && tab.group_id.is_none());\n"
                "}\n"
                "fn push_pinned_tabs() { tabs.iter().filter(|tab| tab.pinned); }\n"
                "fn push_group_tabs(tabs: &[WorkspaceTab], groups: &[WorkspaceTabGroup]) {\n"
                "    for group in groups {}\n"
                "    tabs.iter().filter(|tab| tab.group_id.as_ref() == Some(&group.id));\n"
                "}\n"
                "fn push_unknown_group_tabs() {}\n",
            )
            write_text(
                base / "bulk_close.rs",
                "use super::ordering::ordered_tabs;\n"
                "fn close_tabs_to_right() {}\n"
                "fn close_tabs_to_left() {}\n"
                "fn visual_tab_ids(&self) -> Vec<WorkspaceTabId> {\n"
                "    ordered_tabs(&self.options.tabs, &self.options.groups);\n"
                "}\n"
            )
            write_text(
                base / "actions.rs",
                "let grouped_without_dragged = 0;\n"
                "let grouped_start = 0;\n"
                "let ungrouped_start = 0;\n"
                "if dragged.pinned {}\n"
                "if dragged.group_id.is_some() {}\n"
                "MoveGroup { group_id: WorkspaceTabGroupId, to_index: usize }\n"
                "SetGroupColor { group_id: WorkspaceTabGroupId, color: String }\n"
                "Ungroup { group_id: WorkspaceTabGroupId }\n"
                "CloseGroup { group_id: WorkspaceTabGroupId }\n",
            )
            write_text(
                base / "bar.rs",
                "WorkspaceTabBarAction::MoveGroup => self.move_group(group_id, to_index)\n"
                "WorkspaceTabBarAction::SetGroupColor => self.set_group_color(group_id, color)\n"
                "WorkspaceTabBarAction::Ungroup => self.ungroup(group_id)\n"
                "WorkspaceTabBarAction::CloseGroup => self.close_group(group_id)\n",
            )
            write_text(
                base / "context_commands.rs",
                "pub fn move_group_action() { WorkspaceTabBarAction::MoveGroup; }\n"
                "pub fn set_group_color_action() { WorkspaceTabBarAction::SetGroupColor; }\n"
                "fn commands() { WorkspaceTabBarAction::Ungroup; WorkspaceTabBarAction::CloseGroup; }\n",
            )
            write_text(
                base / "events.rs",
                "GroupReordered\ncloseable_tab_group_reordered\n"
                "GroupColorChanged\ncloseable_tab_group_color_changed\n"
                "GroupRemoved\ncloseable_tab_group_removed\n",
            )
            write_text(
                base / "group_mutations.rs",
                "fn move_group() { WorkspaceTabBarEvent::GroupReordered; }\n"
                "fn set_group_color() { WorkspaceTabBarEvent::GroupColorChanged; }\n"
                "fn ungroup() { WorkspaceTabBarEvent::GroupRemoved; }\n"
                "fn close_group() { WorkspaceTabBarEvent::GroupRemoved; }\n",
            )
            write_text(
                base / "tests/state_action_contract.rs",
                "fn pinned_tabs_are_before_grouped_tabs_and_bulk_close_uses_that_visual_order() {\n"
                "    WorkspaceTabBarAction::CloseToLeft;\n"
                "}\n"
                "fn visual_tabs_keep_pinned_before_declared_group_order() {}\n"
                "fn close_to_right_after_pin_uses_pinned_before_group_visual_order() {}\n"
                "fn pinning_grouped_tab_removes_group_membership_and_moves_to_fixed_region() {\n"
                "    WorkspaceTabBarEvent::TabGroupChanged;\n"
                "}\n"
                "fn move_to_group_rejects_pinned_and_ungroupable_tabs() {}\n"
                "fn closed_tab_history_restores_last_closed_tab_through_typed_action() {\n"
                "    WorkspaceTabBarAction::RestoreClosedTab;\n"
                "    WorkspaceTabBarEvent::TabRestored;\n"
                "}\n"
                "fn drop_rules_keep_grouped_prefix_and_pinned_region_distinct() {\n"
                "    WorkspaceTabDropRules::can_accept();\n"
                "}\n"
                "fn move_group_reorders_declared_groups_and_visual_tabs() {\n"
                "    WorkspaceTabBarAction::MoveGroup;\n"
                "    WorkspaceTabBarEvent::GroupReordered;\n"
                "}\n"
                "fn move_group_clamps_out_of_range_target_index_to_last_declared_group() {}\n"
                "fn group_color_ungroup_and_close_group_emit_typed_events() {\n"
                "    WorkspaceTabBarAction::SetGroupColor;\n"
                "    WorkspaceTabBarAction::Ungroup;\n"
                "    WorkspaceTabBarAction::CloseGroup;\n"
                "    WorkspaceTabBarEvent::GroupColorChanged;\n"
                "    WorkspaceTabBarEvent::GroupRemoved;\n"
                "}\n",
            )
            write_text(
                base / "tests/api_contract.rs",
                "fn all_tab_context_command_ids_round_trip_to_public_actions() {\n"
                "    WorkspaceTabContextCommand::from_id(command.id());\n"
                "    WorkspaceTabContextCommand::CloseToLeft;\n"
                "    WorkspaceTabContextCommand::RestoreClosed;\n"
                "    WorkspaceTabContextCommand::Unpin;\n"
                "    WorkspaceTabContextCommand::MoveToNewGroup;\n"
                "    WorkspaceTabContextCommand::MoveToGroup;\n"
                "}\n",
            )
            storybook = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                storybook / "screen_state_tabs_group_context.rs",
                "fn move_group_from_context() {\n"
                "    CloseableTabGroupContextCommand::move_group_action();\n"
                "}\n"
                "fn set_group_color_from_context() {\n"
                "    CloseableTabGroupContextCommand::set_group_color_action();\n"
                "}\n"
                "fn ungroup_from_context() { CloseableTabGroupContextCommand::Ungroup; }\n"
                "fn close_group_from_context() { CloseableTabGroupContextCommand::Close; }\n",
            )
            write_text(
                storybook / "visual_interaction_tabs_group_move_tests.rs",
                "fn tabs_group_header_context_menu_move_reorders_groups_through_core_action() {\n"
                "    closeable_tab_group_reordered;\n"
                "    target_index=1;\n"
                "}\n"
                "fn tabs_group_header_context_menu_move_wraps_last_group_to_first_through_core_action() {\n"
                "    target_index=0;\n"
                "}\n",
            )
            write_text(
                storybook / "visual_interaction_tabs_context_group_tests.rs",
                "fn tabs_group_header_context_menu_applies_color_ungroup_and_close() {\n"
                "    closeable_tab_group_color_changed;\n"
                "    closeable_tab_group_removed;\n"
                "}\n",
            )
            write_text(
                storybook / "visual_interaction_closeable_tab_strip_group_context_tests.rs",
                "fn closeable_tab_strip_group_header_context_menu_moves_ungroups_and_uses_rendered_item_ids() {\n"
                "    items[1];\n"
                "    id = \"rename\";\n"
                "    GROUP_MOVE_INDEX;\n"
                "    GROUP_UNGROUP_INDEX;\n"
                "    closeable_tab_group_reordered;\n"
                "    closeable_tab_group_removed;\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_context_close_pinned_before_group_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = (
                root
                / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_context_close.rs"
            )
            write_text(
                path,
                "fn visual_context_tab_ids(&self) {\n"
                "    self.tabs.iter().filter(|tab| tab.pinned);\n"
                "    for group in &self.groups {}\n"
                "    self.tabs.iter().filter(|tab| !tab.pinned && tab.group_id.is_none());\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual(1, len(failures))
            self.assertIn("core visual tab order", failures[0])

    def test_accepts_storybook_context_close_group_before_pinned_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = (
                root
                / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_context_close.rs"
            )
            write_text(
                path,
                "fn visual_context_tab_ids(&self) {\n"
                "    self.core_visual_tab_ids();\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tab_move_without_visual_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs"
            write_text(
                path,
                "fn move_active_right(&mut self) {\n"
                "    let from = self.tabs.iter().position(|tab| tab.id == self.active_tab_id);\n"
                "    CloseableTabStripAction::MoveTab { to_visual_index: from + 1 };\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual(1, len(failures))
            self.assertIn("shared visual tab order", failures[0])

    def test_accepts_storybook_tab_move_with_visual_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs"
            write_text(
                path,
                "fn move_active_right(&mut self) {\n"
                "    let visual_ids = self.core_visual_tab_ids();\n"
                "    CloseableTabStripAction::MoveTab { to_visual_index: from + 1 };\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tab_drag_without_core_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "window_interaction/tabs_drag.rs",
                "fn apply_drag_at() { dedicated_tabs::tab_hit_at(); }\n",
            )
            write_text(base / "window_interaction.rs", "fn apply_mouse_click() {}\n")
            write_text(base / "screen_state_tabs_drag.rs", "fn start_drag_tab() {}\n")

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertGreaterEqual(len(failures), 3)
            self.assertTrue(
                any("tab drag must use hit-tested visual order" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("window interaction must route tab drag" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("tab drag state must bridge to core" in it for it in failures),
                failures,
            )

    def test_accepts_storybook_tab_drag_with_core_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "window_interaction/tabs_drag.rs",
                "fn start_at() { dedicated_tabs::tab_hit_at(); register_tabs_drag_start(); }\n"
                "fn apply_drag_at() { drop_visual_index(); register_tabs_drag_move(); }\n"
                "fn release() { register_tabs_drag_end(); }\n",
            )
            write_text(
                base / "window_interaction.rs",
                "tabs_drag::start_at(); tabs_drag::apply_drag_at(); tabs_drag::release();\n",
            )
            write_text(
                base / "screen_state_tabs_drag.rs",
                "CloseableTabStripAction::StartDrag\n"
                "CloseableTabStripAction::MoveTab\n"
                "apply_core_tab_drag_end\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tabs_keyboard_without_core_route(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            core = root / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar"
            write_text(core / "keyboard.rs", "fn keyboard() {}\n")
            write_text(core / "tests/keyboard_contract.rs", "fn test() {}\n")
            write_text(base / "window_keyboard.rs", "fn apply_keyboard() {}\n")
            write_text(base / "window_interaction/tabs_keyboard.rs", "fn shortcut() {}\n")
            write_text(base / "screen_state_tabs_keyboard.rs", "fn keyboard() {}\n")
            write_text(base / "screen_state_tabs_bridge.rs", "fn bridge() {}\n")
            write_text(base / "visual_interaction_tabs_keyboard_tests.rs", "fn test() {}\n")
            write_text(
                base / "visual_interaction_closeable_tab_strip_keyboard_tests.rs",
                "fn test() {}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertGreaterEqual(len(failures), 8)
            self.assertTrue(
                any("tabs keyboard route" in it for it in failures),
                failures,
            )

    def test_accepts_storybook_tabs_keyboard_with_core_route(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            core = root / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar"
            write_text(
                core / "keyboard.rs",
                "fn keyboard() {\n"
                "    WorkspaceTabKeyboardInput::NextTab;\n"
                "    WorkspaceTabKeyboardInput::PreviousTab;\n"
                "    WorkspaceTabKeyboardInput::SelectLastVisible;\n"
                "}\n",
            )
            write_text(
                core / "tests/keyboard_contract.rs",
                "fn keyboard_ctrl_tab_cycles_visible_tabs() {}\n"
                "fn keyboard_shift_ctrl_tab_cycles_backwards() {}\n"
                "fn keyboard_digit_zero_selects_last_visible_tab() {}\n",
            )
            write_text(
                base / "window_keyboard.rs",
                "fn tabs_keyboard_shortcut() {\n"
                "    apply_tabs_keyboard_shortcut(); command_or_control; Key::LeftSuper;\n"
                "}\n",
            )
            write_text(
                base / "window_interaction/tabs_keyboard.rs",
                "fn apply_tabs_keyboard_shortcut() {\n"
                "    is_tab_story_page(); page == \"closeable-tab-strip\";\n"
                "    CloseableTabKeyboardInput::from_shortcut();\n"
                "    register_tabs_keyboard_input();\n"
                "    tabs_drag_target.take();\n"
                "    register_tabs_drag_end(&target.tab_id, false);\n"
                "}\n",
            )
            write_text(
                base / "screen_state_tabs_keyboard.rs",
                "fn apply_keyboard_input() {\n"
                "    apply_core_tab_keyboard_input();\n"
                "    CloseableTabKeyboardInput::CloseActiveTab;\n"
                "    tab_keyboard_select_visible; tab_keyboard_close;\n"
                "}\n",
            )
            write_text(
                base / "screen_state_tabs_bridge.rs",
                "fn register_tabs_keyboard_input() {\n"
                "    self.tabs.apply_keyboard_input(input);\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_keyboard_tests.rs",
                "fn tabs_keyboard_shortcuts_route_through_storybook_window_interaction() {\n"
                "    CloseableTabKeyboardShortcut;\n"
                "    CloseableTabKey::Digit(2);\n"
                "    closeable_tab_close_requested;\n"
                "    CloseableTabKey::Escape;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_keyboard_tests.rs",
                "fn closeable_tab_strip_keyboard_shortcuts_route_through_storybook_window_interaction() {\n"
                "    CloseableTabKeyboardShortcut;\n"
                "    CloseableTabKey::Digit(2);\n"
                "    CloseableTabKey::Tab;\n"
                "    closeable_tab_close_requested;\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_accepts_storybook_tabs_order_options_and_instance_guards(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            core = root / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar"
            write_text(
                core / "scroll.rs",
                "impl WorkspaceTabScrollPlanner {\n"
                "    pub fn follow_active() {}\n"
                "}\n",
            )
            write_text(
                core / "tests/overflow_contract.rs",
                "fn scroll_planner_follows_active_tab_when_external_selection_moves_right() {\n"
                "    WorkspaceTabScrollPlanner::follow_active();\n"
                "}\n"
                "fn scroll_planner_follows_active_tab_when_external_selection_moves_left() {}\n",
            )
            write_text(
                base / "dedicated_tabs.rs",
                "fn layout_item_ids_for_test() { format!(\"group:{}\", group.id); }\n",
            )
            write_text(
                base / "dedicated_closeable_tab_strip.rs",
                "fn scroll_x_for_test() {}\n"
                "fn strip_rect_for_test() {}\n",
            )
            write_text(
                base / "dedicated_tabs_scroll.rs",
                "fn measured_item_ids_for_test() { measured_items(state); }\n",
            )
            write_text(
                base / "visual_interaction_tabs_order_tests.rs",
                "fn tabs_storybook_layout_order_matches_core_visual_order_for_declared_unknown_pinned_ungrouped() {\n"
                "    core_visual_tab_ids(); dedicated_tabs::tab_ids_for_test();\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_scroll_tests.rs",
                "fn tabs_scroll_measured_order_matches_render_layout_order() {\n"
                "    layout_item_ids_for_test(); measured_item_ids_for_test();\n"
                "}\n"
                "fn tabs_active_follow_preset_scrolls_current_tab_into_strip() {}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_scroll_tests.rs",
                "fn closeable_tab_strip_active_follow_preset_scrolls_current_tab_into_strip() {\n"
                "    dedicated_closeable_tab_strip::scroll_x_for_test();\n"
                "    dedicated_closeable_tab_strip::strip_rect_for_test();\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_options_tests.rs",
                "fn tabs_inspector_options_mutate_tab_model_state() {\n"
                "    tabs.overflow_width; overflow_trigger_width;\n"
                "    tabs.group_auto_expand; collapsed_group_auto_expand_ms;\n"
                "    tabs.count=6 active=notes.md;\n"
                "    tabs.pinned=true left-fixed;\n"
                "    tabs.group=Docs;\n"
                "    tabs.overflow=menu;\n"
                "    tabs.active_scroll=follow;\n"
                "    assert_tabs_option_event;\n"
                "    state.screen_state.last_action;\n"
                "    state.screen_state.last_event;\n"
                "    starts_with(\"closeable_tab\");\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_state_tests.rs",
                "fn tabs_window_interaction_keeps_instance_state_isolated() {}\n",
            )
            write_text(
                base / "screen_state_tabs_bridge.rs",
                "fn setting() {\n"
                "    option.setting != \"active_tab_id\";\n"
                "    CloseableTabStripAction::SelectTab;\n"
                "    tabs.active_scroll;\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tabs_context_menu_item_id_drift_gap(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "screen_state_tabs_context_menu_types.rs",
                "fn for_tab() { CloseableTabContextMenu::tab_commands(); }\n",
            )
            write_text(
                base / "dedicated_tabs_context_menu.rs",
                "fn command_at() { menu.commands.get(index); }\n",
            )
            write_text(
                base / "visual_interaction_tabs_context_tests.rs",
                "fn tabs_context_menu_commands_apply_close_pin_and_group_actions() {}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_context_tests.rs",
                "fn closeable_tab_strip_tab_context_menu_applies_workspace_tab_commands() {}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_context_no_group_tests.rs",
                "fn closeable_tab_strip_context_menu_without_existing_groups_uses_direct_new_group_action() {}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertTrue(any("rendered item ids" in it for it in failures))
            self.assertTrue(any("parallel command index" in it for it in failures))

    def test_accepts_storybook_tabs_context_menu_item_id_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "screen_state_tabs_context_menu_types.rs",
                "fn from_item_id() {\n"
                "    CloseableTabContextCommand::from_id();\n"
                "    CloseableTabGroupContextCommand::from_id();\n"
                "    move_to_group_id_from_item_id();\n"
                "    MoveToExistingGroup;\n"
                "    to_context_menu_items();\n"
                "    group_submenu_item();\n"
                "    Self::NewGroup.id();\n"
                "}\n",
            )
            write_text(
                base / "dedicated_tabs_context_menu.rs",
                "fn command_at() {\n"
                "    items.get(index);\n"
                "    TabsContextMenuCommand::from_item_id();\n"
                "    visible_items(menu);\n"
                "    push_visible_item();\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_context_tests.rs",
                "fn tabs_context_menu_click_uses_rendered_item_id_not_parallel_index() {\n"
                "    items[0].id = \"pin\";\n"
                "}\n"
                "fn tabs_context_menu_restores_last_closed_tab_through_core_action() {\n"
                "    RESTORE_CLOSED_INDEX;\n"
                "    closeable_tab_restored;\n"
                "}\n"
                "fn tabs_context_menu_moves_to_selected_existing_group_not_fixed_default() {\n"
                "    move-to-group:docs;\n"
                "    \"Review\";\n"
                "    \"グループに追加\";\n"
                "}\n"
                "fn tabs_context_menu_without_existing_groups_uses_direct_new_group_action() {\n"
                "    \"新しいグループを作成\";\n"
                "}\n"
                "fn pinned_tab_menu_hides_group_commands() {\n"
                "    !pinned_labels.contains(&\"新しいグループを作成\");\n"
                "}\n"
                "fn tabs_context_menu_hides_group_commands_for_ungroupable_tab() {\n"
                "    scratch.groupable = false;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_tabs_context_group_tests.rs",
                "fn tabs_group_header_context_menu_toggles_collapse_through_core_action() {\n"
                "    items[1];\n"
                "    id = \"rename\";\n"
                "    group_context_rename;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_context_tests.rs",
                "fn closeable_tab_strip_context_menu_click_uses_rendered_item_id() {\n"
                "    items[0].id = \"pin\";\n"
                "    closeable_tab_pin_changed;\n"
                "}\n"
                "fn closeable_tab_strip_context_menu_restores_last_closed_tab() {\n"
                "    closeable_tab_restored;\n"
                "}\n"
                "fn closeable_tab_strip_context_menu_moves_to_selected_existing_group() {\n"
                "    \"Review\";\n"
                "    \"グループに追加\";\n"
                "}\n"
                "fn closeable_pinned_tab_menu_hides_group_commands() {\n"
                "    !pinned_labels.contains(&\"新しいグループを作成\");\n"
                "}\n"
                "fn closeable_tab_strip_context_menu_hides_group_commands_for_ungroupable_tab() {\n"
                "    scratch.groupable = false;\n"
                "}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_context_no_group_tests.rs",
                "fn closeable_tab_strip_context_menu_without_existing_groups_uses_direct_new_group_action() {\n"
                "    \"新しいグループを作成\";\n"
                "    !labels.contains(&\"グループに追加\");\n"
                "    tab_context_new_group;\n"
                "    closeable_tab_group_changed;\n"
                "    context-group;\n"
                "}\n",
            )

            failures = WorkspaceTabGuardrails(root).failures()

            self.assertEqual([], failures)

    def test_detects_storybook_box_leak(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            page = root / "storybook/src/pages/sample.rs"
            page.parent.mkdir(parents=True)
            page.write_text("fn page() { let _ = Box::leak(Box::new(\"x\")); }\n", encoding="utf-8")

            failures = KucGuardrails(root).storybook_leak_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("Box::leak", failures[0])

    def test_detects_missing_openspec_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            task = root / "openspec/changes/ui-core-root-plan/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text("- [x] 1.1 `storybook/src/pages/sample.rs` を追加\n", encoding="utf-8")

            failures = KucGuardrails(root).openspec_evidence_failures()

            self.assertEqual(2, len(failures))

    def test_openspec_evidence_scans_all_active_changes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            task = root / "openspec/changes/storybook-page-button/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text("- [x] 1.1 `missing/path.rs` を追加\n", encoding="utf-8")

            failures = KucGuardrails(root).openspec_evidence_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("storybook-page-button/tasks.md", failures[0])

    def test_openspec_evidence_scans_completed_feedback_rows(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            task = root / "openspec/changes/storybook-page-tabs/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text("- [/] 1.1 `missing/context_menu.rs` を更新\n", encoding="utf-8")

            failures = KucGuardrails(root).openspec_evidence_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("storybook-page-tabs/tasks.md", failures[0])

    def test_openspec_evidence_resolves_current_repo_shorthand_paths(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "openspec/changes/storybook-page-button/tasks.md",
                "- [x] 1.1 `catalog/preset_labels.rs` を更新\n"
                "- [x] 1.2 `visual/storybook_ui_option_contract.rs` を更新\n"
                "- [x] 1.3 `interaction_contract/callback_action_contract.rs` を更新\n"
                "- [x] 1.4 `visual/legacy_01_24_contract*.rs` を更新\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
                "",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs",
                "",
            )
            write_text(
                root
                / "crates/katana-ui-core/tests/interaction_contract/callback_action_contract.rs",
                "",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract_tests.rs",
                "",
            )

            failures = KucGuardrails(root).openspec_evidence_failures()

            self.assertEqual([], failures)

    def test_openspec_evidence_resolves_core_nested_and_mod_paths(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "openspec/changes/02-add-drag-drop-primitive/tasks.md",
                "- [x] 1.1 `interaction/drop_target.rs` を追加\n"
                "- [x] 1.2 `molecule/structured/collapsible_panel.rs` を追加\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/interaction/drag_and_drop/drop_target.rs",
                "",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/collapsible_panel/mod.rs",
                "",
            )

            failures = KucGuardrails(root).openspec_evidence_failures()

            self.assertEqual([], failures)

    def test_detects_runtime_api_gated_by_test_cfg(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            ops = root / "crates/katana-ui-core/src/layout/split/ops.rs"
            ops.parent.mkdir(parents=True)
            ops.write_text(
                "#[cfg(test)]\npub(super) fn drag_ratio() -> f32 { 1.0 }\n",
                encoding="utf-8",
            )

            failures = KucGuardrails(root).runtime_api_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("drag_ratio", failures[0])

    def test_detects_missing_interactive_callback(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            types = root / "crates/katana-ui-core/src/composite/selector/toggle/types.rs"
            types.parent.mkdir(parents=True)
            types.write_text("pub struct ToggleProps { pub value: bool }\n", encoding="utf-8")

            failures = KucGuardrails(root).callback_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("on_change", failures[0])

    def test_detects_file_length_without_review_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = root / "crates/katana-ui-core/src/layout/card/types.rs"
            source.parent.mkdir(parents=True)
            source.write_text("pub struct X;\n" * 260, encoding="utf-8")
            task = root / "openspec/changes/ui-core-root-plan/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text(
                "- [x] 1.1 file-length 対応で `crates/katana-ui-core/src/layout/card/types.rs` を追加\n",
                encoding="utf-8",
            )

            failures = KucGuardrails(root).file_length_review_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("ops.rs", failures[0])

    def test_requires_repo_local_guardrail_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).repo_local_guardrail_policy_failures()

            self.assertEqual(3, len(failures))

    def test_accepts_repo_local_guardrail_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_repo_policy(root)

            failures = KucGuardrails(root).repo_local_guardrail_policy_failures()

            self.assertEqual([], failures)

    def test_rejects_katana_specific_svg_boundary_in_core(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_generic_ui_contract(root)
            write_text(
                root / "crates/katana-ui-core/src/render_model/typed_icon.rs",
                "pub struct UiIconProps { pub svg_source: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "fn leading_svg_icon_slot() {}\n"
                "fn trailing_svg_icon_button() {}\n"
                "fn icon(svg_source: String) { UiIconProps::new(svg_source); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs",
                "pub icon: Option<UiIconProps>\n"
                "pub fn svg_icon(mut self, value: UiIconProps) -> Self { self }\n"
                "fn icon(value: String) { UiIconProps::new(value); }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/bad.rs",
                "pub enum KatanaSvgIcon { Search }\n",
            )

            failures = KucGuardrails(root).generic_rust_ui_boundary_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("KUC core must stay generic", failures[0])

    def test_accepts_external_svg_props_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_generic_ui_contract(root)
            write_text(
                root / "crates/katana-ui-core/src/render_model/typed_icon.rs",
                "pub struct UiIconProps { pub svg_source: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "fn leading_svg_icon_slot() {}\n"
                "fn trailing_svg_icon_button() {}\n"
                "fn icon(svg_source: String) { UiIconProps::new(svg_source); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs",
                "pub icon: Option<UiIconProps>\n"
                "pub fn svg_icon(mut self, value: UiIconProps) -> Self { self }\n"
                "fn icon(value: String) { UiIconProps::new(value); }\n",
            )

            failures = KucGuardrails(root).generic_rust_ui_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_consumer_app_without_public_molecule_usage(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_generic_ui_contract(root)
            write_text(
                root / "crates/katana-ui-core/src/render_model/typed_icon.rs",
                "pub struct UiIconProps { pub svg_source: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "fn leading_svg_icon_slot() {}\n"
                "fn trailing_svg_icon_button() {}\n"
                "fn icon(svg_source: String) { UiIconProps::new(svg_source); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs",
                "pub icon: Option<UiIconProps>\n"
                "pub fn svg_icon(mut self, value: UiIconProps) -> Self { self }\n"
                "fn icon(value: String) { UiIconProps::new(value); }\n",
            )
            write_text(
                root / "examples/kuc-consumer-app/src/lib.rs",
                "quick_search: SearchBox\n",
            )

            failures = KucGuardrails(root).generic_rust_ui_boundary_failures()

            self.assertTrue(
                any("workspace_select: SelectBox" in failure for failure in failures),
                failures,
            )

    def test_rejects_missing_generic_app_layout_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_generic_ui_contract(root)
            write_text(
                root / "crates/katana-ui-core/src/render_model/typed_icon.rs",
                "pub struct UiIconProps { pub svg_source: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "fn leading_svg_icon_slot() {}\n"
                "fn trailing_svg_icon_button() {}\n"
                "fn icon(svg_source: String) { UiIconProps::new(svg_source); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs",
                "pub icon: Option<UiIconProps>\n"
                "pub fn svg_icon(mut self, value: UiIconProps) -> Self { self }\n"
                "fn icon(value: String) { UiIconProps::new(value); }\n",
            )
            (root / "crates/katana-ui-core/tests/generic_rust_app_layout_contract.rs").unlink()

            failures = KucGuardrails(root).generic_rust_ui_boundary_failures()

            self.assertIn(
                "generic Rust UI boundary file is missing",
                failures[0],
            )

    def test_rejects_missing_generic_app_action_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_generic_ui_contract(root)
            write_text(
                root / "crates/katana-ui-core/src/render_model/typed_icon.rs",
                "pub struct UiIconProps { pub svg_source: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "fn leading_svg_icon_slot() {}\n"
                "fn trailing_svg_icon_button() {}\n"
                "fn icon(svg_source: String) { UiIconProps::new(svg_source); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs",
                "pub icon: Option<UiIconProps>\n"
                "pub fn svg_icon(mut self, value: UiIconProps) -> Self { self }\n"
                "fn icon(value: String) { UiIconProps::new(value); }\n",
            )
            (root / "crates/katana-ui-core/tests/generic_rust_app_action_contract.rs").unlink()

            failures = KucGuardrails(root).generic_rust_ui_boundary_failures()

            self.assertIn(
                "generic Rust UI boundary file is missing",
                failures[0],
            )

    def test_rejects_storybook_katana_icon_pack_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/bad.rs",
                "mod katana_svg_icons; enum KatanaSvgIcon { Search }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/assets/katana-icons/ui/search.svg",
                "<svg />\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual(3, len(failures))

    def test_accepts_storybook_live_component_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/page.rs",
                "fn page() { StoryCatalog::interactive_story(); }\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_single_crate_distribution_requires_feature_gated_public_modules(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/Cargo.toml",
                "[features]\n"
                "default = []\n"
                'text-raster = ["dep:cosmic-text"]\n'
                'svg-raster = ["dep:resvg"]\n'
                'egui = ["text-raster", "svg-raster"]\n',
            )
            write_text(
                root / "crates/katana-ui-core/src/lib.rs",
                '#[cfg(feature = "egui")]\npub mod egui;\n'
                '#[cfg(feature = "text-raster")]\npub mod text_raster;\n'
                '#[cfg(feature = "svg-raster")]\npub mod svg_raster;\n',
            )
            for module in ("egui", "text_raster", "svg_raster"):
                write_text(root / f"crates/katana-ui-core/src/{module}/mod.rs", "")
            write_text(
                root / "crates/katana-ui-core/tests/egui_contract.rs",
                '#![cfg(feature = "egui")]\n',
            )

            failures = KucGuardrails(root).single_crate_distribution_failures()

            self.assertEqual([], failures)

    def test_single_crate_distribution_rejects_ungated_egui_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/Cargo.toml",
                "[features]\ndefault = []\n"
                'text-raster = []\nsvg-raster = []\n'
                'egui = ["text-raster", "svg-raster"]\n',
            )
            write_text(
                root / "crates/katana-ui-core/src/lib.rs",
                '#[cfg(feature = "egui")]\npub mod egui;\n'
                '#[cfg(feature = "text-raster")]\npub mod text_raster;\n'
                '#[cfg(feature = "svg-raster")]\npub mod svg_raster;\n',
            )
            for module in ("egui", "text_raster", "svg_raster"):
                write_text(root / f"crates/katana-ui-core/src/{module}/mod.rs", "")
            write_text(
                root / "crates/katana-ui-core/tests/egui_contract.rs",
                "use katana_ui_core::egui;\n",
            )

            failures = KucGuardrails(root).single_crate_distribution_failures()

            self.assertTrue(
                any("egui integration contract must be feature-gated" in item for item in failures),
                failures,
            )

    def test_single_crate_distribution_rejects_retired_package_reintroduction(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(root / "crates/katana-ui-core/Cargo.toml", "[features]\ndefault = []\n")
            write_text(root / "crates/katana-ui-core/src/lib.rs", "")
            write_text(
                root / "crates/katana-ui-core-egui-adapter/Cargo.toml",
                "[package]\nname = \"katana-ui-core-egui-adapter\"\n",
            )

            failures = KucGuardrails(root).single_crate_distribution_failures()

            self.assertTrue(
                any("retired capability must remain inside katana-ui-core" in item for item in failures),
                failures,
            )

    def test_rejects_storybook_private_svg_raster_dependencies(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core-storybook/Cargo.toml",
                "[dependencies]\nkatana-ui-core = { workspace = true }\nresvg.workspace = true\ntiny-skia.workspace = true\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_svg_icon.rs",
                "use resvg::usvg;\nuse tiny_skia::Pixmap;\nfn rasterize_svg() {\n"
                "    let _ = usvg::Tree::from_str(\"<svg/>\", &usvg::Options::default());\n"
                "}\n",
            )

            failures = KucGuardrails(root).storybook_svg_runtime_boundary_failures()

            self.assertTrue(
                any("must depend on katana-ui-core with the storybook-artifacts feature" in failure for failure in failures),
                failures,
            )
            self.assertTrue(
                any("private SVG raster dependency `resvg`" in failure for failure in failures), failures)
            self.assertTrue(
                any("private SVG raster path `resvg::`" in failure for failure in failures), failures)

    def test_accepts_storybook_public_svg_raster_runtime_adapter(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core-storybook/Cargo.toml",
                "[dependencies]\nkatana-ui-core = { workspace = true, features = [\"storybook-artifacts\"] }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_svg_icon.rs",
                "use katana_ui_core::svg_raster::{UiSvgRasterRequest, UiSvgRasterizer};\n"
                "fn draw(request: UiSvgRasterRequest, rasterizer: &mut UiSvgRasterizer) {\n"
                "    let _ = rasterizer.rasterize(&request);\n"
                "}\n",
            )

            failures = KucGuardrails(root).storybook_svg_runtime_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_command_chrome_host_fixed_text_and_glyph_fallback(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = root / "crates/katana-ui-core/src/molecule/command_chrome/model.rs"
            write_text(
                source,
                "use katana_language_editor::Editor;\n"
                "fn draw() {\n"
                "    let _ = egui::TextEdit::singleline(&mut String::new());\n"
                "    let _ = UiIconProps::new(\"⭐️\");\n"
                "    let _ = \"Search controls\";\n"
                "}\n",
            )

            failures = KucGuardrails(root).command_chrome_boundary_failures()

            self.assertTrue(any("katana_language_editor" in failure for failure in failures), failures)
            self.assertTrue(any("egui::" in failure for failure in failures), failures)
            self.assertTrue(any("UiIconProps::new(" in failure for failure in failures), failures)
            self.assertTrue(any("Search controls" in failure for failure in failures), failures)

    def test_accepts_generic_injected_command_chrome_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/command_chrome/model.rs",
                "pub struct CommandChromeAction { icon: Option<UiIconProps> }\n"
                "pub struct SearchControlStrings { label: String }\n",
            )

            failures = KucGuardrails(root).command_chrome_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_missing_retained_context_presentation_and_root_style_literals(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            adapter = root / "crates/katana-ui-core/src/egui"
            write_text(
                adapter / "text_command_surface/types.rs",
                "pub struct EguiTextCommandSurfacePresentation {}\n"
                "impl TextCommandSurfaceStyle { fn context_menu_raster_style() {} fn context_menu_paint_style() {} }\n",
            )
            write_text(
                adapter / "text_command_surface/synchronization.rs",
                "fn synchronize_context_menu() {}\n",
            )
            write_text(
                adapter / "text_command_surface/context_menu.rs",
                "fn show() { let color = [1, 2, 3, 4]; }\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/text_command_surface/context_menu.rs",
                "context_menu: Some(context_menu)\n"
                "ContextMenuEvent::TypeAheadMatched\n"
                "fn assert_focus_restored() {}\n"
                "AccessKitActionRequest\n"
                "assert_menu_closed(&outside_restored);\n"
                "assert_context_menu_opened(&accesskit_open)\n",
            )

            failures = KucGuardrails(root).text_command_surface_context_menu_root_contract_failures()

            self.assertTrue(any("presentation contract missing" in item for item in failures), failures)
            self.assertTrue(any("in-module color literal" in item for item in failures), failures)

    def test_rejects_prospective_consumer_context_menu_composition(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests/harness.rs",
                "use adapter::{EguiTextCommandSurfaceAdapter, EguiContextMenuAdapter};\n"
                "fn show() { request_open(anchor); plans.push(ArtifactPaintPlanRef::ContextMenu(plan)); }\n"
                "fn reorder() { output.artifact_order.insert(0, layer); output.artifact_order.remove(0); output.artifact_order.sort(); output.artifact_order.reverse(); output.artifact_order.extend(layers); output.artifact_order.splice(.., layers); }\n",
            )

            failures = KucGuardrails(root).text_command_surface_context_menu_consumer_failures()

            self.assertTrue(any("sequential EguiContextMenuAdapter" in item for item in failures), failures)
            self.assertTrue(any("request_open(" in item for item in failures), failures)
            self.assertTrue(any("ArtifactPaintPlanRef::ContextMenu" in item for item in failures), failures)
            for token in (".artifact_order.insert(", ".artifact_order.remove(", ".artifact_order.sort(", ".artifact_order.reverse(", ".artifact_order.extend(", ".artifact_order.splice("):
                self.assertTrue(any(token in item for item in failures), failures)

    def test_rejects_public_text_command_surface_artifact_order_storage(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/egui/text_command_surface/types.rs",
                "pub struct EguiTextCommandSurfaceOutput {\n"
                "    pub artifact_order: Vec<EguiTextCommandSurfaceChild>,\n"
                "}\n",
            )

            failures = KucGuardrails(
                root
            ).text_command_surface_artifact_order_ownership_failures()

            self.assertTrue(any("mutable public artifact_order storage" in item for item in failures), failures)
            self.assertTrue(any("read-only" in item for item in failures), failures)

    def test_rejects_controlled_presentation_geometry_and_legacy_gutter_escape_hatches(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/text_surface/gutter_types.rs",
                "pub struct TextSurfaceGutterRowId;\n"
                "pub struct TextSurfaceGutterRow;\n"
                "pub struct TextSurfaceAutomaticGutterOverride;\n"
                "pub struct TextSurfaceAutomaticGutterPresentation {\n"
                "    pub width: u32,\n"
                "    pub display_label: String,\n"
                "    pub logical_row: usize,\n"
                "    pub bounds: UiRect,\n"
                "    pub overrides: Vec<(TextSurfaceGutterRowId, TextSurfaceAutomaticGutterOverride)>,\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/text_surface/gutter.rs",
                "impl TextSurfaceAutomaticGutterPresentation {\n"
                "    pub const fn new(width: u32) -> Self { todo!() }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/text_surface/props.rs",
                "pub struct TextSurfacePresentation {\n"
                "    pub gutter: TextSurfaceGutter,\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/text_surface/surface_controlled.rs",
                "fn synchronize() { TextSurfaceGutter::new(32); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/command_chrome/floating_model.rs",
                "pub struct FloatingCommandToolbarPresentation {\n"
                "    pub anchor: Rect,\n"
                "    pub viewport: Rect,\n"
                "    pub panel_size: Size,\n"
                "}\n",
            )

            failures = KucGuardrails(root).controlled_presentation_boundary_failures()

            self.assertTrue(any("automatic gutter DTO must not accept `width`" in item for item in failures), failures)
            self.assertTrue(any("automatic gutter DTO must not accept `display_label`" in item for item in failures), failures)
            self.assertTrue(any("automatic gutter DTO must not accept `logical_row`" in item for item in failures), failures)
            self.assertTrue(any("automatic gutter constructor must not accept consumer geometry" in item for item in failures), failures)
            self.assertTrue(any("must not expose legacy gutter props" in item for item in failures), failures)
            self.assertTrue(any("must not require consumer gutter geometry" in item for item in failures), failures)
            self.assertTrue(any("floating toolbar DTO must not accept `panel_size`" in item for item in failures), failures)

    def test_accepts_controlled_presentation_without_consumer_geometry(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/text_surface/gutter_types.rs",
                "pub struct TextSurfaceGutterRowId;\n"
                "pub struct TextSurfaceAutomaticGutterOverride;\n"
                "pub struct TextSurfaceAutomaticGutterPresentation {\n"
                "    pub overrides: Vec<(TextSurfaceGutterRowId, TextSurfaceAutomaticGutterOverride)>,\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/text_surface/gutter.rs",
                "impl TextSurfaceAutomaticGutterPresentation {\n"
                "    pub const fn new() -> Self { todo!() }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/text_surface/props.rs",
                "pub struct TextSurfacePresentation {\n"
                "    pub automatic_gutter: Option<TextSurfaceAutomaticGutterPresentation>,\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/text_surface/surface_controlled.rs",
                "fn synchronize() { TextSurfaceGutter::from_controlled_automatic(value); }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/command_chrome/floating_model.rs",
                "pub struct FloatingCommandToolbarPresentation {\n"
                "    pub anchor: Rect,\n"
                "    pub viewport: Rect,\n"
                "    pub visibility: FloatingCommandToolbarVisibility,\n"
                "}\n"
                "impl FloatingCommandToolbarPresentation {\n"
                "    pub const fn new(anchor: Rect, viewport: Rect, visibility: FloatingCommandToolbarVisibility) -> Self { todo!() }\n"
                "}\n",
            )

            failures = KucGuardrails(root).controlled_presentation_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_private_text_surface_adapter_paths_and_emoji_substitution(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/Cargo.toml",
                "[dependencies]\n"
                "cosmic-text.workspace = true\n"
                "katana-language-editor.workspace = true\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/egui/text_surface/adapter.rs",
                "fn draw() {\n"
                "    let _ = egui::TextEdit::singleline(&mut String::new());\n"
                "    let _ = FontDefinitions::default();\n"
                "    let _ = input.replace(\"⭐️\", \"☆\");\n"
                "    ui.painter().text(Default::default(), Default::default(), \"x\", Default::default(), Default::default());\n"
                "}\n",
            )

            failures = KucGuardrails(root).egui_text_surface_adapter_boundary_failures()

            self.assertTrue(any("katana-language-editor" in failure for failure in failures), failures)
            self.assertTrue(any("egui::TextEdit" in failure for failure in failures), failures)
            self.assertTrue(any("FontDefinitions" in failure for failure in failures), failures)
            self.assertTrue(any('replace(\"⭐' in failure for failure in failures), failures)
            self.assertTrue(any("painter().text(" in failure for failure in failures), failures)

    def test_accepts_shared_text_surface_adapter_boundary(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/Cargo.toml", "[dependencies]\negui.workspace = true\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/egui/text_surface/adapter.rs",
                "use crate::text_raster::PlatformTextRasterizer;\n"
                "fn draw(_rasterizer: &mut PlatformTextRasterizer) {}\n",
            )

            failures = KucGuardrails(root).egui_text_surface_adapter_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_command_chrome_adapter_private_text_paths(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/egui/command_chrome_future.rs",
                "fn draw() {\n"
                "    let _ = egui::TextEdit::singleline(&mut String::new());\n"
                "    let _ = egui::Popup::from_response;\n"
                "    let _ = ui.button(\"fallback\");\n"
                "    ui.painter().text(Default::default(), Default::default(), \"x\", Default::default(), Default::default());\n"
                "}\n",
            )

            failures = KucGuardrails(root).egui_command_chrome_adapter_boundary_failures()

            self.assertTrue(any("egui::TextEdit" in failure for failure in failures), failures)
            self.assertTrue(any("egui::Popup" in failure for failure in failures), failures)
            self.assertTrue(any("ui.button(" in failure for failure in failures), failures)
            self.assertTrue(any("painter().text(" in failure for failure in failures), failures)

    def test_rejects_ambiguous_storybook_next_change_completion_scope(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "scripts/next-storybook-page-change.py",
                "print({'complete': True})\n",
            )
            write_text(
                root / "scripts/test_next_storybook_page_change.py",
                "def test_returns_none_when_all_complete(): pass\n",
            )

            failures = KucGuardrails(root).storybook_next_change_scope_failures()

            self.assertGreaterEqual(len(failures), 4)
            self.assertTrue(any("completion_scope" in it for it in failures), failures)
            self.assertTrue(any("kuc_dod_complete" in it for it in failures), failures)

    def test_accepts_scoped_storybook_next_change_completion_scope(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "scripts/next-storybook-page-change.py",
                '"completion_scope": "storybook_page_leaf_changes"\n'
                '"complete": kuc_dod_complete\n'
                '"kuc_dod_complete": kuc_dod_complete\n'
                "remaining_handoff_items = self.remaining_handoff_items()\n"
                "kuc_dod_complete = not remaining_handoff_items\n"
                "manual_acceptance_queue(manifest)\n"
                '"pending_reason": "manual_acceptance_pending"\n'
                '"next_manual_acceptance_page": next_page\n'
                '"pending_manual_acceptance_pages": pending_pages\n'
                "audit remaining P0/P1 handoff items\n",
            )
            write_text(
                root / "scripts/test_next_storybook_page_change.py",
                "def test_complete_payload_is_false_when_leaf_queue_is_done_but_kuc_dod_has_handoff_items():\n"
                "    self.assertEqual(\"storybook_page_leaf_changes\", payload[\"completion_scope\"])\n"
                "    self.assertFalse(payload[\"kuc_dod_complete\"])\n"
                "    self.assertFalse(payload[\"complete\"])\n"
                "def test_complete_payload_is_true_only_when_leaf_queue_and_kuc_dod_are_done():\n"
                "    self.assertTrue(payload[\"kuc_dod_complete\"])\n"
                "def test_payload_names_next_manual_acceptance_page_when_leaf_queue_is_done():\n"
                "    payload[\"next_manual_acceptance_page\"]\n"
                "    payload[\"next_command\"]\n"
                "    remaining_handoff_items\n",
            )

            failures = KucGuardrails(root).storybook_next_change_scope_failures()

            self.assertEqual([], failures)

    def test_rejects_missing_kuc_remaining_work_handoff(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).storybook_remaining_handoff_failures()

            self.assertEqual(
                [
                    "docs/reviews/*kuc-remaining-work-handoff.md: KUC remaining work handoff is missing"
                ],
                failures,
            )

    def test_accepts_kuc_remaining_work_handoff_with_manual_pending_items(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "docs/reviews/2026-06-14-kuc-remaining-work-handoff.md",
                "## P0\n"
                "- [ ] P0-1 text manual acceptance: manual_acceptance_pending "
                "text_drag_selection text_keyboard_copy text_zero_distance_drag_no_selection "
                "until audit_status=verified is allowed by the user.\n"
                "- [ ] P0-2 progress-bar manual acceptance: manual_acceptance_pending "
                "with progress_timed_tick progress_timed_cycle progress_indeterminate_segment_motion evidence.\n"
                "## P1\n"
                "- [ ] P1-1 final storybook-interaction-smoke\n",
            )

            failures = KucGuardrails(root).storybook_remaining_handoff_failures()

            self.assertEqual([], failures)

    def test_rejects_handoff_missing_manifest_manual_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "docs/storybook-77ui-interaction-manifest.json",
                "{"
                '"ui":['
                '{"page":"text","gaps":["manual_acceptance_pending: user confirmation"]},'
                '{"page":"progress-bar","gaps":["manual_acceptance_pending: user confirmation"]}'
                "]"
                "}\n",
            )
            write_text(
                root / "docs/reviews/2026-06-14-kuc-remaining-work-handoff.md",
                "## P0\n"
                "- [ ] P0-1 progress-bar manual acceptance: manual_acceptance_pending "
                "with progress_indeterminate_segment_motion evidence until audit_status=verified.\n"
                "## P1\n"
                "- [ ] P1-1 final storybook-interaction-smoke\n",
            )

            failures = KucGuardrails(root).storybook_remaining_handoff_failures()

            self.assertIn(
                "docs/reviews/2026-06-14-kuc-remaining-work-handoff.md: manual pending page `text` missing from remaining work handoff",
                failures,
            )

    def test_accepts_handoff_matching_manifest_manual_pending_pages(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "docs/storybook-77ui-interaction-manifest.json",
                "{"
                '"ui":['
                '{"page":"text","gaps":["manual_acceptance_pending: user confirmation"]},'
                '{"page":"progress-bar","gaps":["manual_acceptance_pending: user confirmation"]}'
                "]"
                "}\n",
            )
            write_text(
                root / "docs/reviews/2026-06-14-kuc-remaining-work-handoff.md",
                "## P0\n"
                "- [ ] P0-1 text manual acceptance: manual_acceptance_pending "
                "text_drag_selection text_keyboard_copy text_zero_distance_drag_no_selection "
                "until audit_status=verified is allowed by the user.\n"
                "- [ ] P0-2 progress-bar manual acceptance: manual_acceptance_pending "
                "with progress_timed_tick progress_timed_cycle progress_indeterminate_segment_motion evidence.\n"
                "## P1\n"
                "- [ ] P1-1 final storybook-interaction-smoke\n",
            )

            failures = KucGuardrails(root).storybook_remaining_handoff_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tabs_without_core_action_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs",
                "struct TabsScreenState;\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("CloseableTabStrip actions", failures[0])

    def test_accepts_storybook_tabs_core_action_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs",
                "struct TabsScreenState;\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_core.rs",
                "CloseableTabStripAction CloseableTabStripEvent "
                "apply_core_tab_action apply_core_tab_action_confirming_dirty "
                "CloseableTabStripEvent::name\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tabs_without_direct_pin_icon_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs",
                "struct TabsScreenState;\n"
                "CloseableTabStripAction::UnpinTab\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_core.rs",
                "CloseableTabStripAction CloseableTabStripEvent "
                "apply_core_tab_action apply_core_tab_action_confirming_dirty "
                "CloseableTabStripEvent::name\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs.rs",
                "fn tabs() {}\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertTrue(
                any("direct pin icon contract" in failure for failure in failures),
                failures,
            )

    def test_accepts_storybook_tabs_direct_pin_icon_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs",
                "fn unpin_tab_by_icon() { CloseableTabStripAction::UnpinTab; "
                "tab_pin_icon_unpin direct-icon }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_core.rs",
                "CloseableTabStripAction CloseableTabStripEvent "
                "apply_core_tab_action apply_core_tab_action_confirming_dirty "
                "CloseableTabStripEvent::name\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs.rs",
                "fn pin_icon_hit_at() {}\nfn pin_icon_rect_for_test() {}\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs_layout.rs",
                "fn pin_icon_hit_at() {}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_bridge.rs",
                "fn register_tabs_pin_icon_unpin() {}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/button_operation.rs",
                "enum StorybookButtonOperation { TabsPinIcon }\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_tests.rs",
                "fn tabs_pinned_icon_click_directly_unpins_tab() {}\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tabs_pinned_after_group_layout_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs_layout.rs",
                "fn layout_items() {\n"
                "    push_grouped_tabs(&mut items);\n"
                "    push_pinned_tabs(&mut items);\n"
                "}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/bar.rs",
                "fn append_workspace_tab_children() {\n"
                "    for group in &options.groups {}\n"
                "    for tab in options.tabs.iter().filter(|tab| tab.pinned) {}\n"
                "}\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertTrue(
                any("pinned tabs before group blocks" in failure for failure in failures),
                failures,
            )

    def test_accepts_storybook_tabs_pinned_before_group_layout_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs_layout.rs",
                "fn layout_items() {\n"
                "    push_pinned_tabs(&mut items);\n"
                "    push_grouped_tabs(&mut items);\n"
                "}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/bar.rs",
                "fn append_workspace_tab_children() {\n"
                "    for tab in options.tabs.iter().filter(|tab| tab.pinned) {}\n"
                "    for group in &options.groups {}\n"
                "}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_tests.rs",
                "fn tabs_pinned_tabs_render_before_group_block() {}\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/closeable_tab_strip_rendering_contract.rs",
                "fn closeable_tab_strip_renders_pinned_tabs_before_group_blocks() {}\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_closeable_tab_strip_without_live_core_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/dedicated_closeable_tab_strip.rs",
                "fn closeable_tab_strip() {}\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertTrue(
                any("closeable-tab-strip live core bridge" in failure for failure in failures),
                failures,
            )

    def test_accepts_closeable_tab_strip_live_core_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            base = root / "crates/katana-ui-core-storybook/src/visual"
            write_text(
                base / "dedicated_closeable_tab_strip.rs",
                "fn context_menu_command_at() {}\n",
            )
            write_text(
                base / "screen_state_tabs_bridge.rs",
                "fn register_closeable_tab_strip_select() {\n"
                "    CloseableTabStripAction::SelectTab;\n"
                "}\n",
            )
            write_text(
                base / "window_interaction/button_operation.rs",
                "enum StorybookButtonOperation { CloseableTabStripSelect }\n",
            )
            write_text(
                base / "window_interaction/button_operation/tabs_operation.rs",
                "fn click() { dedicated_closeable_tab_strip::tab_hit_at(); }\n",
            )
            write_text(
                base / "window_interaction/context_click.rs",
                "fn closeable_tab_strip_context_target() {}\n"
                "fn menu() { context_menu_command_at(); }\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_tests.rs",
                "fn closeable_tab_strip_component_click_selects_real_core_tab() {}\n"
                "fn closeable_tab_strip_context_menu_uses_real_core_commands() {}\n",
            )
            write_text(
                base / "visual_interaction_closeable_tab_strip_context_tests.rs",
                "fn closeable_tab_strip_tab_context_menu_applies_workspace_tab_commands() {\n"
                "    CLOSE_OTHERS_INDEX;\n"
                "    CLOSE_RIGHT_INDEX;\n"
                "    MOVE_TO_GROUP_INDEX;\n"
                "}\n"
                "fn closeable_tab_strip_context_menu_keeps_pinned_tabs_fixed_until_unpinned() {}\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_tabs_context_menu_without_core_context_menu_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs",
                "struct TabsScreenState;\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_core.rs",
                "CloseableTabStripAction CloseableTabStripEvent "
                "apply_core_tab_action apply_core_tab_action_confirming_dirty "
                "CloseableTabStripEvent::name\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_context.rs",
                "struct ContextMenu;\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual(8, len(failures))
            self.assertIn("context menu bridge", failures[0])

    def test_accepts_storybook_tabs_context_menu_core_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs",
                "struct TabsScreenState;\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_core.rs",
                "CloseableTabStripAction CloseableTabStripEvent "
                "apply_core_tab_action apply_core_tab_action_confirming_dirty "
                "CloseableTabStripEvent::name\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_context.rs",
                "CloseableTabContextMenu::menu ContextMenuAnchor::Pointer "
                "context_node.props().context_menu.items "
                "CloseableTabContextCommand::from_id "
                "CloseableTabGroupContextCommand::from_id "
                "from_item_id "
                "TabsContextMenuCommand::for_group open_context_menu_for_group\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_text_area_without_core_action_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_text_area.rs",
                "struct TextAreaRuntime;\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("core TextAreaAction", failures[0])

    def test_accepts_storybook_input_core_action_bridges(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_text_input.rs",
                "Input::new UiAction::input_value ComponentAction "
                "apply_core_text_input_value\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_text_area.rs",
                "struct TextAreaRuntime;\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/screen_state_text_area_core.rs",
                "TextArea::new TextAreaAction TextAreaActionOutcome "
                "apply_text_area_action\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_search_box_without_core_action_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/search_box_screen_state.rs",
                "struct SearchBoxRuntime;\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual(5, len(failures))
            self.assertIn("search-box core bridge", failures[0])

    def test_rejects_storybook_selection_without_core_action_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/selection_screen_state.rs",
                "struct SelectionScreenState;\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("selection controls", failures[0])

    def test_accepts_storybook_molecule_core_action_bridges(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_storybook_live_component_contract(root)
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/search_box_screen_state.rs",
                "SearchBox::new UiAction::input_value UiAction::search_submitted "
                "UiAction::clear_value ComponentAction\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/selection_screen_state.rs",
                "struct SelectionScreenState;\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/selection_screen_state_core.rs",
                "SelectBox::new ComboBox::new SelectionList::new "
                "UiAction::select_box_selected UiAction::set_selected_index "
                "ComponentAction\n",
            )

            failures = KucGuardrails(root).storybook_live_component_contract_failures()

            self.assertEqual([], failures)

    def test_rejects_combo_only_selection_api_in_shared_choice_macro(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/choice.rs",
                "macro_rules! choice_molecule {\n"
                "    () => { pub fn input_value(mut self) -> Self { self } };\n"
                "}\n"
                "choice_molecule!(SelectBox);\n"
                "impl ComboBox { pub fn input_value(mut self) -> Self { self } }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/accessors.rs",
                "macro_rules! selection_accessors {\n"
                "    () => { pub fn input_model(&self) -> &str { \"\" } };\n"
                "}\n"
                "selection_accessors!(Breadcrumb);\n"
                "impl ComboBox { pub fn input_model(&self) -> &str { \"\" } }\n",
            )

            failures = KucGuardrails(root).choice_api_boundary_failures()

            self.assertTrue(
                any("combo-only builder `pub fn input_value`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("combo-only accessor `pub fn input_model`" in it for it in failures),
                failures,
            )

    def test_accepts_combo_only_selection_api_on_combo_box_impl(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/choice.rs",
                "macro_rules! choice_molecule { () => { pub fn item(mut self) -> Self { self } }; }\n"
                "choice_molecule!(SelectBox);\n"
                "impl ComboBox {\n"
                "    pub fn input_value(mut self) -> Self { self }\n"
                "    pub fn filter_result(mut self) -> Self { self }\n"
                "    pub fn free_input(mut self) -> Self { self }\n"
                "}\n"
                "impl Breadcrumb { pub fn crumb_action(mut self) -> Self { self } }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/accessors.rs",
                "macro_rules! selection_accessors { () => { pub fn selected_option(&self) {} }; }\n"
                "selection_accessors!(Breadcrumb);\n"
                "impl ComboBox {\n"
                "    pub fn input_model(&self) -> &str { \"\" }\n"
                "    pub fn filter_results(&self) -> &[ChoiceItem] { &[] }\n"
                "    pub fn allows_free_input(&self) -> bool { false }\n"
                "}\n"
                "impl Breadcrumb { pub fn crumb_action_model(&self) {} }\n"
                "impl Tabs { pub fn icon_action_model(&self) {} }\n"
                "impl SideMenu { pub fn hover_expansion_model(&self) {} }\n"
                "impl SelectionList {\n"
                "    pub fn section_model(&self) {}\n"
                "    pub fn marker_model(&self) {}\n"
                "    pub fn has_more_row(&self) {}\n"
                "}\n",
            )

            failures = KucGuardrails(root).choice_api_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_specialized_selection_api_in_shared_macros(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/choice.rs",
                "macro_rules! choice_molecule {\n"
                "    () => { pub fn crumb_action(mut self) -> Self { self } };\n"
                "}\n"
                "choice_molecule!(SelectBox);\n"
                "impl Breadcrumb { pub fn crumb_action(mut self) -> Self { self } }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/options.rs",
                "macro_rules! selection_options {\n"
                "    () => {\n"
                "        pub fn icon_action(mut self) -> Self { self }\n"
                "        pub fn hover_expansion(mut self) -> Self { self }\n"
                "        pub fn section(mut self) -> Self { self }\n"
                "        pub fn marker(mut self) -> Self { self }\n"
                "        pub fn more_row(mut self) -> Self { self }\n"
                "    };\n"
                "}\n"
                "selection_options!(Breadcrumb);\n"
                "impl Tabs { pub fn icon_action(mut self) -> Self { self } }\n"
                "impl SideMenu { pub fn hover_expansion(mut self) -> Self { self } }\n"
                "impl SelectionList {\n"
                "    pub fn section(mut self) -> Self { self }\n"
                "    pub fn marker(mut self) -> Self { self }\n"
                "    pub fn more_row(mut self) -> Self { self }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/accessors.rs",
                "macro_rules! selection_accessors {\n"
                "    () => {\n"
                "        pub fn crumb_action_model(&self) {}\n"
                "        pub fn icon_action_model(&self) {}\n"
                "        pub fn hover_expansion_model(&self) {}\n"
                "        pub fn section_model(&self) {}\n"
                "        pub fn marker_model(&self) {}\n"
                "        pub fn has_more_row(&self) {}\n"
                "    };\n"
                "}\n"
                "selection_accessors!(Breadcrumb);\n"
                "impl Breadcrumb { pub fn crumb_action_model(&self) {} }\n"
                "impl Tabs { pub fn icon_action_model(&self) {} }\n"
                "impl SideMenu { pub fn hover_expansion_model(&self) {} }\n"
                "impl SelectionList {\n"
                "    pub fn section_model(&self) {}\n"
                "    pub fn marker_model(&self) {}\n"
                "    pub fn has_more_row(&self) {}\n"
                "}\n",
            )

            failures = KucGuardrails(root).choice_api_boundary_failures()

            self.assertTrue(
                any("breadcrumb-only builder `pub fn crumb_action`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("tabs-only builder `pub fn icon_action`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("side-menu-only builder `pub fn hover_expansion`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("selection-list-only builder `pub fn section`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("breadcrumb-only accessor `pub fn crumb_action_model`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("tabs-only accessor `pub fn icon_action_model`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("side-menu-only accessor `pub fn hover_expansion_model`" in it for it in failures),
                failures,
            )
            self.assertTrue(
                any("selection-list-only accessor `pub fn section_model`" in it for it in failures),
                failures,
            )

    def test_accepts_specialized_selection_api_on_target_impls(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/choice.rs",
                "macro_rules! choice_molecule { () => { pub fn item(mut self) -> Self { self } }; }\n"
                "choice_molecule!(SelectBox);\n"
                "impl ComboBox {\n"
                "    pub fn input_value(mut self) -> Self { self }\n"
                "    pub fn filter_result(mut self) -> Self { self }\n"
                "    pub fn free_input(mut self) -> Self { self }\n"
                "}\n"
                "impl Breadcrumb { pub fn crumb_action(mut self) -> Self { self } }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/options.rs",
                "macro_rules! selection_options { () => { pub fn items(&self) {} }; }\n"
                "selection_options!(Breadcrumb);\n"
                "impl Tabs { pub fn icon_action(mut self) -> Self { self } }\n"
                "impl SideMenu { pub fn hover_expansion(mut self) -> Self { self } }\n"
                "impl SelectionList {\n"
                "    pub fn section(mut self) -> Self { self }\n"
                "    pub fn marker(mut self) -> Self { self }\n"
                "    pub fn more_row(mut self) -> Self { self }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/accessors.rs",
                "macro_rules! selection_accessors { () => { pub fn selected_option(&self) {} }; }\n"
                "selection_accessors!(Breadcrumb);\n"
                "impl ComboBox {\n"
                "    pub fn input_model(&self) -> &str { \"\" }\n"
                "    pub fn filter_results(&self) -> &[ChoiceItem] { &[] }\n"
                "    pub fn allows_free_input(&self) -> bool { false }\n"
                "}\n"
                "impl Breadcrumb { pub fn crumb_action_model(&self) {} }\n"
                "impl Tabs { pub fn icon_action_model(&self) {} }\n"
                "impl SideMenu { pub fn hover_expansion_model(&self) {} }\n"
                "impl SelectionList {\n"
                "    pub fn section_model(&self) {}\n"
                "    pub fn marker_model(&self) {}\n"
                "    pub fn has_more_row(&self) {}\n"
                "}\n",
            )

            failures = KucGuardrails(root).choice_api_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_missing_svg_render_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/render_model/svg_icon_render_plan.rs",
                "pub struct UiSvgIconRenderPlan { pub svg_source: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/mod.rs",
                "mod svg_icon_render_plan;\n",
            )

            failures = KucGuardrails(root).adapter_svg_render_plan_failures()

            self.assertGreaterEqual(len(failures), 3)
            self.assertTrue(
                any("core/tests/svg_icon_render_plan_contract.rs" in it for it in failures),
                failures,
            )

    def test_accepts_svg_render_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/render_model/svg_icon_render_plan.rs",
                "pub struct UiSvgIconRenderPlan {\n"
                "    pub svg_source: String,\n"
                "    pub view_box: String,\n"
                "    pub path_summary: String,\n"
                "    pub paint_policy: super::UiSvgPaintPolicy,\n"
                "    pub theme_token: String,\n"
                "    pub callback: String,\n"
                "}\n"
                "impl UiSvgIconRenderPlan {\n"
                "    pub fn collect_from_tree() { leading_slot; trailing_icon_buttons; }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/svg_icon_pixel_plan.rs",
                "pub struct UiSvgIconViewBox;\n"
                "pub struct UiSvgIconPixelPlan {\n"
                "    pub viewport: UiRect,\n"
                "    pub scale_x_milli: u32,\n"
                "    pub scale_y_milli: u32,\n"
                "    pub pixel_ready: bool,\n"
                "}\n"
                "const DEFAULT_SVG_ICON_BOX_PX: u32 = 16;\n"
                "fn collect() { UiSvgIconRenderPlan::collect_from_tree(); }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/mod.rs",
                "pub use svg_icon_pixel_plan::{UiSvgIconPixelPlan, UiSvgIconViewBox};\n"
                "pub use svg_icon_render_plan::UiSvgIconRenderPlan;\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/svg_icon_render_plan_contract.rs",
                "CALLER_SEARCH_SVG\n"
                "CALLER_CLEAR_SVG\n"
                "UiSvgIconRenderPlan::collect_from_tree\n"
                "UiSvgIconPixelPlan::collect_from_tree\n"
                "svg_icon_pixel_plan_preserves_viewbox_scale_and_paint_contract\n"
                "semantic_fingerprint_changes_when_text_entry_svg_or_callback_changes\n"
                "svg_icon_render_plan_preserves_external_svg_metadata_for_adapters\n"
                "UiSvgPaintPolicy::StrokeOnly\n",
            )

            failures = KucGuardrails(root).adapter_svg_render_plan_failures()

            self.assertEqual([], failures)

    def test_rejects_missing_host_action_render_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/render_model/host_action_types.rs",
                "pub struct UiHostActionPlan { pub action_id: String }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/mod.rs",
                "mod host_action_plan;\n"
                "mod host_action_types;\n",
            )

            failures = KucGuardrails(root).host_action_render_plan_failures()

            self.assertGreaterEqual(len(failures), 3)
            self.assertTrue(
                any("host_action_plan_contract.rs" in it for it in failures),
                failures,
            )

    def test_accepts_host_action_render_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/render_model/host_action_types.rs",
                "pub struct UiHostActionPlan {\n"
                "    pub action_id: String,\n"
                "    pub enabled: bool,\n"
                "}\n"
                "ui.link.open\n"
                "ui.disclosure.\n"
                "ui.image.highlight\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/host_action_plan.rs",
                "impl UiHostActionPlan {\n"
                "    pub fn collect_from_tree() {}\n"
                "}\n"
                "push_context_menu_item_plans\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/common.rs",
                "pub host_actions: Vec<UiHostActionSpec>\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/mod.rs",
                "pub use host_action_types::{UiHostActionPlan, UiHostActionSpec};\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/host_action_plan_contract.rs",
                "generic_host_action_plan_collects_action_ids_and_enabled_state\n"
                "app.toolbar.\n"
                "ui.surface.\n"
                "UI_IMAGE_HIGHLIGHT_ACTION_ID\n",
            )

            failures = KucGuardrails(root).host_action_render_plan_failures()

            self.assertEqual([], failures)

    def test_rejects_missing_adapter_coverage_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/render_model/adapter_coverage_plan.rs",
                "pub struct UiAdapterCoveragePlan { pub input_count: usize }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/mod.rs",
                "mod adapter_coverage_plan;\n",
            )

            failures = KucGuardrails(root).adapter_coverage_plan_failures()

            self.assertGreaterEqual(len(failures), 3)
            self.assertTrue(
                any("adapter_coverage_plan_contract.rs" in it for it in failures),
                failures,
            )

    def test_accepts_adapter_coverage_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/render_model/adapter_coverage_plan.rs",
                "pub struct UiAdapterCoveragePlan {\n"
                "    pub input_count: usize,\n"
                "    pub text_area_count: usize,\n"
                "    pub tab_container_count: usize,\n"
                "    pub selection_count: usize,\n"
                "    pub split_pane_count: usize,\n"
                "    pub scroll_area_count: usize,\n"
                "    pub modal_count: usize,\n"
                "    pub required_consumer_node_kind_count: usize,\n"
                "    pub missing_required_consumer_node_kinds: Vec<UiNodeKind>,\n"
                "    pub unsupported_node_count: usize,\n"
                "}\n"
                "impl UiAdapterCoveragePlan {\n"
                "    pub fn collect_from_tree() {}\n"
                "    pub fn consumer_shell_ready(&self) -> bool { true }\n"
                "}\n"
                "fn required() { UiNodeKind::ImageSurface; }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/mod.rs",
                "pub use adapter_coverage_plan::UiAdapterCoveragePlan;\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/adapter_contract/action_bridge.rs",
                "pub struct AdapterActionBridge;\n"
                "impl AdapterActionBridge {\n"
                "    pub fn dispatch<ComponentAction>(\n"
                "        component: &mut ComponentAction,\n"
                "        action: &UiAction,\n"
                "    ) -> UiActionResult {\n"
                "        component.apply_action(action)\n"
                "    }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/adapter_contract/host_action_bridge.rs",
                "pub struct AdapterHostActionBridge;\n"
                "fn trigger() {\n"
                "    UiHostActionPlan::collect_from_root();\n"
                "    action.enabled;\n"
                "    action.action_id == action_id;\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/adapter_contract/mod.rs",
                "pub use action_bridge::AdapterActionBridge;\n"
                "pub use host_action_bridge::AdapterHostActionBridge;\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/adapter_coverage_plan_contract.rs",
                "adapter_coverage_plan_reports_consumer_shell_surfaces\n"
                "adapter_coverage_plan_blocks_consumer_ready_when_unsupported_nodes_exist\n"
                "adapter_coverage_plan_requires_image_surface_for_native_raster_parity\n"
                "ImageSurface::from_rgba\n"
                "modal_count\n"
                "consumer_shell_ready\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/adapter_host_action_bridge_contract.rs",
                "adapter_host_action_bridge_triggers_enabled_button_command\n"
                "adapter_host_action_bridge_triggers_text_entry_icon_callback\n"
                "adapter_host_action_bridge_triggers_text_area_icon_callback\n"
                "adapter_host_action_bridge_rejects_disabled_action\n",
            )
            write_text(
                root / "docs/dependency-policy.md",
                "UiAdapterCoveragePlan AdapterActionBridge AdapterHostActionBridge "
                "core crate outside core\n",
            )

            failures = KucGuardrails(root).adapter_coverage_plan_failures()

            self.assertEqual([], failures)

    def test_rejects_kal_side_guardrail_dependency(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_repo_policy(root, "../kal\n")

            failures = KucGuardrails(root).repo_local_guardrail_policy_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("kal-side edits", failures[0])

    def test_requires_agent_stop_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).agent_stop_policy_failures()

            self.assertEqual(1, len(failures))

    def test_accepts_agent_stop_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "AGENTS.md",
                "## runner 停止条件\n"
                "v0.1.0 release readiness が未達\n"
                "ローカル保存（commit）\n"
                "停止理由にしない\n"
                "push confirmation required\n"
                "release confirmation required\n"
                "destructive operation confirmation required\n"
                "次の未完了タスク\n",
            )

            failures = KucGuardrails(root).agent_stop_policy_failures()

            self.assertEqual([], failures)

    def test_requires_agent_stop_hook_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).agent_hook_policy_failures()

            self.assertEqual(4, len(failures))

    def test_accepts_agent_stop_hook_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / ".githooks/pre-commit",
                "just kuc-guardrails\n"
                "fix-and-continue\n"
                "push confirmation required\n"
                "release confirmation required\n"
                "destructive operation confirmation required\n"
                "ユーザー確認で止まらず\n",
            )
            write_text(
                root / ".githooks/pre-push",
                "KUC_PUSH_CONFIRMED\n"
                "push confirmation required\n"
                "release confirmation required\n",
            )
            write_text(
                root / "scripts/install-git-hooks.sh",
                "git config core.hooksPath .githooks\n",
            )
            write_text(root / "AGENTS.md", "repository hook\n")

            failures = KucGuardrails(root).agent_hook_policy_failures()

            self.assertEqual([], failures)

    def test_requires_kuc_guardrails_to_run_release_readiness(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "kuc-guardrails: consumer-app-contract\n"
                "    python3 scripts/test_kuc_guardrails.py\n"
                "    python3 scripts/assert-kuc-guardrails.py\n",
            )

            failures = KucGuardrails(root).release_readiness_recipe_failures()

            self.assertEqual(11, len(failures))

    def test_requires_release_readiness_runtime_check_not_only_self_test(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "consumer-app-contract:\n"
                "    cargo test -p kuc-consumer-app --locked\n"
                "    cargo test -p katana-ui-core --test generic_rust_app_contract --locked\n"
                "    cargo test -p katana-ui-core --test generic_rust_app_layout_contract --locked\n"
                "    cargo test -p katana-ui-core --test generic_rust_app_action_contract --locked\n"
                "integration-test: consumer-app-contract\n"
                "e2e-test:\n"
                "    bash scripts/storybook-requirement-gate.sh\n"
                "smoke-test: storybook-smoke storybook-interaction-smoke\n"
                "kuc-guardrails: consumer-app-contract\n"
                "    python3 scripts/test_kuc_guardrails.py\n"
                "    python3 scripts/assert-kuc-release-readiness.py --self-test\n"
                "    python3 scripts/assert-kuc-guardrails.py\n"
                "release-readiness-check: integration-test e2e-test smoke-test\n",
            )

            failures = KucGuardrails(root).release_readiness_recipe_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("runtime check", failures[0])

    def test_accepts_kuc_guardrails_release_readiness_recipe(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "consumer-app-contract:\n"
                "    cargo test -p kuc-consumer-app --locked\n"
                "    cargo test -p katana-ui-core --test generic_rust_app_contract --locked\n"
                "    cargo test -p katana-ui-core --test generic_rust_app_layout_contract --locked\n"
                "    cargo test -p katana-ui-core --test generic_rust_app_action_contract --locked\n"
                "integration-test: consumer-app-contract\n"
                "e2e-test:\n"
                "    bash scripts/storybook-requirement-gate.sh\n"
                "smoke-test: storybook-smoke storybook-interaction-smoke\n"
                "kuc-guardrails: consumer-app-contract\n"
                "    python3 scripts/test_kuc_guardrails.py\n"
                "    python3 scripts/assert-kuc-release-readiness.py --self-test\n"
                "    python3 scripts/assert-kuc-release-readiness.py\n"
                "    python3 scripts/assert-kuc-guardrails.py\n"
                "release-readiness-check: integration-test e2e-test smoke-test\n",
            )

            failures = KucGuardrails(root).release_readiness_recipe_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_regression_without_manual_acceptance_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "storybook-manual-acceptance-smoke:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_smoke.py\n"
                "storybook-regression: cargo-test storybook-check storybook-smoke storybook-interaction-smoke storybook-requirement-gate\n",
            )

            failures = KucGuardrails(root).storybook_regression_recipe_failures()

            self.assertEqual(
                [
                    "Justfile: storybook-manual-acceptance-approval-template recipe is missing",
                    "Justfile: storybook-manual-acceptance-approval-template must run the approval template script",
                    "Justfile: storybook-manual-acceptance-next recipe is missing",
                    "Justfile: storybook-manual-acceptance-next must run the next script",
                    "Justfile: storybook-manual-acceptance-status recipe is missing",
                    "Justfile: storybook-manual-acceptance-status must run the status script",
                    "Justfile: storybook-manual-acceptance-complete-next recipe is missing",
                    "Justfile: storybook-manual-acceptance-complete-next must run the complete-next script",
                    "Justfile: storybook-manual-acceptance-mark-approved recipe is missing",
                    "Justfile: storybook-manual-acceptance-mark-approved must run the mark-approved script",
                    "Justfile: storybook-manual-acceptance-approve recipe is missing",
                    "Justfile: storybook-manual-acceptance-approve must run the approve script",
                    "Justfile: kuc-guardrails must run python3 scripts/test_next_storybook_page_change.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_queue.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_review.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_status.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_next.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_approval_template.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_complete_next.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_mark_approved.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_approve.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_smoke.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_manual_acceptance_final_gate.py",
                    "Justfile: kuc-guardrails must run python3 scripts/test_storybook_interaction_pending_only.py",
                    "Justfile: storybook-manual-acceptance-final-gate recipe is missing",
                    "Justfile: storybook-manual-acceptance-final-gate must run the final gate",
                    "Justfile: storybook-kuc-dod-final recipe is missing",
                    "Justfile: storybook-kuc-dod-final must require final gate and interaction smoke",
                    "Justfile: storybook-interaction-pending-only recipe is missing",
                    "Justfile: storybook-interaction-pending-only must run the pending-only verifier",
                    "scripts/storybook_interaction_pending_only.py: pending-only verifier is missing",
                    "scripts/test_storybook_interaction_pending_only.py: pending-only verifier test is missing",
                    "scripts/storybook_manual_acceptance_final_gate.py: manual acceptance final gate is missing",
                    "scripts/test_storybook_manual_acceptance_final_gate.py: manual acceptance final gate test is missing",
                    "scripts/storybook_manual_acceptance_metadata.py: manual acceptance metadata validator is missing",
                    "Justfile: storybook-manual-acceptance-smoke must regenerate live interaction audit",
                    "Justfile: storybook-regression must include storybook-manual-acceptance-smoke"
                ],
                failures,
            )

    def test_rejects_manual_acceptance_smoke_without_fresh_live_audit(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "storybook-manual-acceptance-smoke:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_smoke.py\n"
                "storybook-regression: cargo-test storybook-check storybook-smoke storybook-manual-acceptance-smoke storybook-interaction-smoke storybook-requirement-gate\n",
            )

            failures = KucGuardrails(root).storybook_regression_recipe_failures()

            self.assertIn(
                "Justfile: storybook-manual-acceptance-smoke must regenerate live interaction audit",
                failures,
            )

    def test_accepts_storybook_regression_with_manual_acceptance_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "storybook-manual-acceptance-smoke:\n"
                "    cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --headless-interaction-audit\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_smoke.py\n"
                "storybook-interaction-pending-only:\n"
                "    cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --headless-interaction-audit\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_interaction_pending_only.py\n"
                "storybook-manual-acceptance-approval-template:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_approval_template.py\n"
                "storybook-manual-acceptance-next:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_next.py\n"
                "storybook-manual-acceptance-status:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_status.py\n"
                "storybook-manual-acceptance-complete-next approved_by approved_at:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_complete_next.py --approved-by \"{{approved_by}}\" --approved-at \"{{approved_at}}\"\n"
                "storybook-manual-acceptance-mark-approved page approved_by approved_at:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_mark_approved.py --page \"{{page}}\" --approved-by \"{{approved_by}}\" --approved-at \"{{approved_at}}\"\n"
                "storybook-manual-acceptance-approve page:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_approve.py --page \"{{page}}\"\n"
                "kuc-guardrails:\n"
                "    python3 scripts/test_next_storybook_page_change.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_queue.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_review.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_status.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_next.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_approval_template.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_complete_next.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_mark_approved.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_approve.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_smoke.py\n"
                "    python3 scripts/test_storybook_manual_acceptance_final_gate.py\n"
                "    python3 scripts/test_storybook_interaction_pending_only.py\n"
                "storybook-manual-acceptance-final-gate:\n"
                "    PYTHONPATH=scripts python3 scripts/storybook_manual_acceptance_final_gate.py\n"
                "storybook-kuc-dod-final: storybook-manual-acceptance-final-gate storybook-interaction-smoke\n"
                "storybook-regression: cargo-test storybook-check storybook-smoke storybook-manual-acceptance-smoke storybook-interaction-pending-only storybook-interaction-smoke storybook-requirement-gate\n",
            )
            write_text(
                root / "scripts/storybook_interaction_pending_only.py",
                "# fixture\n",
            )
            write_text(
                root / "scripts/test_storybook_interaction_pending_only.py",
                "# fixture\n",
            )
            write_text(
                root / "scripts/storybook_manual_acceptance_final_gate.py",
                "# fixture\n",
            )
            write_text(
                root / "scripts/test_storybook_manual_acceptance_final_gate.py",
                "# fixture\n",
            )
            write_text(
                root / "scripts/storybook_manual_acceptance_metadata.py",
                "# fixture\n",
            )

            failures = KucGuardrails(root).storybook_regression_recipe_failures()

            self.assertEqual([], failures)

    def test_rejects_missing_storybook_live_harness_dor(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).storybook_live_harness_dor_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("Storybook live harness DoR is missing", failures[0])

    def test_accepts_storybook_live_harness_dor(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "docs/storybook-live-harness-dor.md",
                "解析レーン\n"
                "実作業レーン\n"
                "`storybook-interaction-smoke`\n"
                "interaction smoke として未成立\n"
                "checkbox / radio\n"
                "native window 経路\n"
                "screenshot を完了根拠にする\n",
            )
            write_text(
                root / "scripts/storybook-interaction-smoke.sh",
                "--headless-interaction-audit\n"
                "storybook-live-interaction-audit.json\n"
                "checkbox_changed=true\n"
                "radio_changed=true\n"
                "body_pixel_diff\n",
            )

            failures = KucGuardrails(root).storybook_live_harness_dor_failures()

            self.assertEqual([], failures)

    def test_rejects_commit_confirmation_as_stop_reason(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / ".githooks/pre-commit",
                "just kuc-guardrails\n"
                "fix-and-continue\n"
                "push confirmation required\n"
                "release confirmation required\n"
                "destructive operation confirmation required\n"
                "ユーザー確認で止まらず\n",
            )
            write_text(
                root / ".githooks/pre-push",
                "KUC_PUSH_CONFIRMED\n"
                "push confirmation required\n"
                "release confirmation required\n",
            )
            write_text(
                root / "scripts/install-git-hooks.sh",
                "git config core.hooksPath .githooks\n",
            )
            write_text(root / "AGENTS.md", "commit confirmation required\n")

            failures = KucGuardrails(root).agent_hook_policy_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("local commit must not be a stop reason", failures[0])

    def test_checks_storybook_panel_evidence_markers(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).storybook_panel_evidence_failures()

            self.assertEqual(7, len(failures))
            docs = root / "docs/architecture/ui-separation/ui-core-parity-gap.md"
            write_text(
                docs,
                "storybook-panel-interaction-report.json story_selection theme_switch "
                "operation_sequence callback log target state id before / after summary\n",
            )

            failures = KucGuardrails(root).storybook_panel_evidence_failures()

            self.assertEqual([], failures)

    def test_checks_visual_fallback_policy_markers(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).visual_fallback_policy_failures()

            self.assertEqual(3, len(failures))
            docs = root / "docs/architecture/ui-separation/ui-core-parity-gap.md"
            write_text(
                docs,
                "required_ui_fallbacks=0 generic `node` fallback は完了根拠にしない\n",
            )

            failures = KucGuardrails(root).visual_fallback_policy_failures()

            self.assertEqual([], failures)

    def test_checks_storybook_reflection_audit_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).storybook_reflection_audit_policy_failures()

            self.assertEqual(1, len(failures))
            write_text(
                root / "Justfile",
                "kuc-guardrails:\n"
                "    python3 scripts/test_storybook_reflection_audit.py\n"
                "storybook-reflection-audit:\n"
                "    python3 scripts/assert-storybook-reflection-audit.py --strict\n",
            )
            write_text(
                root / "docs/architecture/ui-separation/ui-core-parity-gap.md",
                "just storybook-reflection-audit missing-* page 固有 surface\n",
            )

            failures = KucGuardrails(root).storybook_reflection_audit_policy_failures()

            self.assertEqual([], failures)

    def test_requires_typed_action_model(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).typed_action_model_failures()

            self.assertEqual(4, len(failures))

    def test_accepts_typed_action_model_without_external_store(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            interaction = root / "crates/katana-ui-core/src/interaction/mod.rs"
            component = root / "crates/katana-ui-core/src/component.rs"
            contract = root / "crates/katana-ui-core/tests/interaction_contract.rs"
            callback_contract = (
                root
                / "crates/katana-ui-core/tests/interaction_contract/callback_action_contract.rs"
            )
            write_text(
                interaction,
                "pub enum UiAction {}\npub struct UiActionResult {}\npub struct UiCallbackLog {}\n",
            )
            write_text(
                component,
                "pub trait ComponentAction { fn apply_action(&mut self); }\n",
            )
            write_text(
                contract,
                "fn action_targets_only_the_matching_component_state() {}\n"
                "fn action_result_is_serializable_snapshot() {}\n",
            )
            write_text(
                callback_contract,
                "fn callback_action_invokes_named_callback_without_mutating_value() {}\n",
            )

            failures = KucGuardrails(root).typed_action_model_failures()

            self.assertEqual([], failures)

    def test_requires_component_state_ownership_handle_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).component_state_ownership_failures()

            self.assertGreaterEqual(len(failures), 8)

    def test_accepts_component_state_ownership_handle_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/state.rs",
                "pub struct UiStateHandle<T>(T);\n"
                "pub struct UiComponentState;\n"
                "impl<T> UiStateHandle<T> { pub fn update(&self) {} }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/component.rs",
                "pub trait ComponentStateBinding {}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/mod.rs",
                "pub fn state_snapshot() {}\npub fn sync_state() {}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store.rs",
                "struct Key { component_id: &'static str }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/window_interaction.rs",
                "selected_component_presets\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/navigation_tests.rs",
                "fn preset_tab_selection_is_owned_by_component() {}\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/interaction_contract.rs",
                "fn action_targets_only_the_matching_component_state() {}\n"
                "fn complex_ui_state_is_owned_by_the_component_model() {}\n"
                "fn app_global_state_updates_component_owned_state_via_handle() {}\n"
                "fn state_handle_supports_react_like_get_set_and_update_without_global_store() {}\n",
            )
            write_text(
                root
                / "openspec/changes/establish-kuc-atoms-molecules-catalog/core-foundation-contract.md",
                "UiStateHandle set/update global state component-owned state\n",
            )

            failures = KucGuardrails(root).component_state_ownership_failures()

            self.assertEqual([], failures)

    def test_rejects_page_owned_storybook_component_state(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(root / "crates/katana-ui-core/src/state.rs", "UiStateHandle UiComponentState\n")
            write_text(root / "crates/katana-ui-core/src/component.rs", "ComponentStateBinding\n")
            write_text(root / "crates/katana-ui-core/src/atom/mod.rs", "state_snapshot sync_state\n")
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store.rs",
                "struct Key { page: &'static str, component_id: &'static str }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/window_interaction.rs",
                "selected_component_presets selected_presets\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/navigation_tests.rs",
                "preset_tab_selection_is_owned_by_component\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/interaction_contract.rs",
                "action_targets_only_the_matching_component_state\n"
                "complex_ui_state_is_owned_by_the_component_model\n"
                "app_global_state_updates_component_owned_state_via_handle\n"
                "state_handle_supports_react_like_get_set_and_update_without_global_store\n",
            )
            write_text(
                root
                / "openspec/changes/establish-kuc-atoms-molecules-catalog/core-foundation-contract.md",
                "set/update\n",
            )

            failures = KucGuardrails(root).component_state_ownership_failures()

            self.assertIn("storybook state key must not be page-owned", failures)
            self.assertIn("storybook preset state must be component-owned", failures)

    def test_rejects_public_app_shell_api(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/mod.rs",
                "pub use app_primitives::{AppShell, CollapsiblePanel};\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/widget/molecules.rs",
                "pub use crate::molecule::{AppShellSlot, CollapsiblePanel};\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
                '"app-shell" => &["shell"],\n',
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/kind.rs",
                "pub enum UiNodeKind { AppShell, CollapsiblePanel }\n",
            )

            failures = KucGuardrails(root).public_app_shell_failures()

            self.assertEqual(4, len(failures))

    def test_accepts_collapsible_panel_without_public_app_shell(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/mod.rs",
                "pub use structured::CollapsiblePanel;\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/widget/molecules.rs",
                "pub use crate::molecule::CollapsiblePanel;\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
                '"collapsible-panel" => &["Explorer panel"],\n',
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/kind.rs",
                "pub enum UiNodeKind { CollapsiblePanel }\n",
            )

            failures = KucGuardrails(root).public_app_shell_failures()

            self.assertEqual([], failures)

    def test_rejects_text_surface_storybook_canvas_and_direct_core_action(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            runtime = root / "crates/katana-ui-core-storybook/src/visual/text_surface_runtime.rs"
            artifact = root / "crates/katana-ui-core-storybook/src/visual/text_surface_artifact.rs"
            write_text(
                runtime,
                "EguiTextSurfaceAdapter\n"
                "adapter.show(ui, surface\n"
                "egui::RawInput\n"
                "fn run_scripted_sequence() {}\n"
                "TextSurfaceArtifactFrame\n"
                "TextSurfaceEvent\n"
                "fn actual_egui_script_is_deterministic_and_covers_editor_surface_events() {}\n"
                "fn scripted_artifact_writes_plan_only_png_gif_and_manifest() {}\n"
                "egui::Canvas\n"
                "surface.apply_action(\n",
            )
            write_text(
                artifact,
                "TextSurfacePaintOperationKind\n"
                "fn render_artifact_frame() {}\n"
                "fn write_png() {}\n"
                "fn write_gif() {}\n"
                "adapter-paint-plan-only\n"
                "actual-egui-raw-input\n"
                "color_emoji_texture_present\n"
                "star_variation_selector_present\n",
            )

            failures = KucGuardrails(root).text_surface_storybook_artifact_boundary_failures()

            self.assertTrue(
                any("must not contain `egui::Canvas`" in failure for failure in failures), failures
            )
            self.assertTrue(
                any("must not contain `surface.apply_action(`" in failure for failure in failures),
                failures,
            )

    def test_accepts_text_surface_storybook_actual_egui_paint_plan_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            runtime = root / "crates/katana-ui-core-storybook/src/visual/text_surface_runtime.rs"
            artifact = root / "crates/katana-ui-core-storybook/src/visual/text_surface_artifact.rs"
            write_text(
                runtime,
                "EguiTextSurfaceAdapter\n"
                "adapter.show(ui, surface\n"
                "egui::RawInput\n"
                "fn run_scripted_sequence() {}\n"
                "TextSurfaceArtifactFrame\n"
                "TextSurfaceEvent\n"
                "fn actual_egui_script_is_deterministic_and_covers_editor_surface_events() {}\n"
                "fn scripted_artifact_writes_plan_only_png_gif_and_manifest() {}\n",
            )
            write_text(
                artifact,
                "TextSurfacePaintOperationKind\n"
                "fn render_artifact_frame() {}\n"
                "fn write_png() {}\n"
                "fn write_gif() {}\n"
                "adapter-paint-plan-only\n"
                "actual-egui-raw-input\n"
                "color_emoji_texture_present\n"
                "star_variation_selector_present\n",
            )

            failures = KucGuardrails(root).text_surface_storybook_artifact_boundary_failures()

            self.assertEqual([], failures)

    def test_rejects_storybook_private_artifact_compositor(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/egui/artifact_compositor.rs",
                "mod artifact_compositor_types;\n"
                "mod artifact_compositor_paint;\n"
                "pub struct ArtifactCompositor;\n"
                "impl ArtifactCompositor { pub fn compose() {} }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/text_surface_artifact.rs",
                "fn blend_texture() {}\n",
            )

            failures = KucGuardrails(root).artifact_compositor_boundary_failures()

            self.assertTrue(
                any("ArtifactCompositor::compose" in failure for failure in failures), failures
            )
            self.assertTrue(
                any("blend_texture(" in failure for failure in failures), failures
            )

    def test_release_publish_script_publishes_only_katana_ui_core(self) -> None:
        root = Path(__file__).resolve().parents[1]
        source = (root / "scripts/release/publish-crates.sh").read_text(encoding="utf-8")
        self.assertIn("  katana-ui-core\n", source)
        self.assertNotIn("katana-ui-core-egui-adapter", source)
        self.assertNotIn("katana-ui-core-svg-raster", source)
        self.assertNotIn("katana-ui-core-text-raster", source)
        self.assertIn('wait_until_available "${package}"', source)
        self.assertIn('cargo publish -p "${package}" --locked', source)
        self.assertNotIn("--token", source)

    def test_release_workflows_use_short_lived_crates_io_oidc_tokens(self) -> None:
        root = Path(__file__).resolve().parents[1]
        workflow_paths = [
            root / ".github/workflows/release.yml",
            root / ".github/workflows/release-publish-retry.yml",
        ]
        auth_action = (
            "rust-lang/crates-io-auth-action@"
            "c6f97d42243bad5fab37ca0427f495c86d5b1a18"
        )

        for workflow_path in workflow_paths:
            with self.subTest(workflow=workflow_path.name):
                source = workflow_path.read_text(encoding="utf-8")

                self.assertIn("id-token: write", source)
                self.assertIn(auth_action, source)
                self.assertIn(
                    "CARGO_REGISTRY_TOKEN: ${{ steps.crates_io_auth.outputs.token }}",
                    source,
                )
                self.assertNotIn("secrets.CARGO_REGISTRY_TOKEN", source)

    def test_release_publish_retry_uses_the_immutable_tag_publish_script(self) -> None:
        root = Path(__file__).resolve().parents[1]
        source = (root / ".github/workflows/release-publish-retry.yml").read_text(
            encoding="utf-8"
        )

        self.assertIn("cd release-source", source)
        self.assertIn(
            'bash scripts/release/publish-crates.sh "${{ inputs.version }}"', source
        )
        self.assertNotIn(
            "../release-tools/scripts/release/publish-crates.sh", source
        )

    def test_release_scope_guard_lists_only_katana_ui_core_as_public(self) -> None:
        root = Path(__file__).resolve().parents[1]
        source = (root / "scripts/release/verify-core-release-scope.sh").read_text(
            encoding="utf-8"
        )
        expected = "expected_publishable=$'katana-ui-core'"

        self.assertIn(expected, source)
        self.assertIn('p["publish"] != []', source)


if __name__ == "__main__":
    unittest.main()
