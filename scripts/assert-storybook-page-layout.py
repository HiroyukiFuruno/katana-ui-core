#!/usr/bin/env python3
from pathlib import Path
import sys

STORYBOOK_FILES = (
    Path("storybook/Cargo.toml"),
    Path("storybook/src/main.rs"),
    Path("crates/katana-ui-core-storybook/Cargo.toml"),
    Path("crates/katana-ui-core-storybook/src/lib.rs"),
    Path("crates/katana-ui-core-storybook/src/main.rs"),
    Path("crates/katana-ui-core-storybook/src/snapshot_command.rs"),
    Path("crates/katana-ui-core-storybook/src/snapshot_output.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/atoms/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/atoms/atom_interactions.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/atoms/atom_motion_interactions.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_basic.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_heavy.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_interaction.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_platform_primitives.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_runtime_primitives.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/foundation_theme/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/layout/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_detail.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod_options.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod_specs.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod_specs_atoms.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod_specs_molecules.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/tests.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_operations.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_report.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/preset_labels.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/tests.rs"),
    Path("crates/katana-ui-core-storybook/src/panel.rs"),
    Path("crates/katana-ui-core-storybook/src/panel/panel_build.rs"),
    Path("crates/katana-ui-core-storybook/src/panel/panel_verify.rs"),
    Path("crates/katana-ui-core-storybook/src/requirements.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/coverage.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/coverage_markers.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_basic.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_common.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_complex.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_context_menu.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_context_menu_anchor.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_context_menu_labels.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_context_menu_metrics.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_context_menu_popup.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_atoms.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_common.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_atom_buttons.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_atom_motion.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_atom_primitives.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_form_inputs.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_form_overlays.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_forms.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_metrics.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_molecule_basic.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_molecule_color_diff.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_molecule_disclosure.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_molecule_surfaces.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_dod_molecules.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_foundation_panel.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_feedback.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/inspector.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/inspector_rows.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/layout_metrics.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/layout_metrics/tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/modal.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/navigation.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/navigation_icons.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/navigation_tree.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/palette.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/panel_layout.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/panel_scroll_contract_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/panel_scroll_interaction_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/panel_scroll_layout_contract_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preset_tabs.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/panel_scroll_state.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/panel_scrollbars.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_contract.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_contract_rows.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_contract_rows/tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_detail.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/render.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/render_context.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/runtime.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/scrollbar.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/shell.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/canvas.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/canvas_clip.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/canvas_model.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/canvas/tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/text.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/text_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/types.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/visual_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/content_position.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/panel_scroll_drag.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/scroll_operation.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_01_12.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract/legacy_13_24.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/legacy_01_24_contract_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/tests/button_operation_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/tests/navigation_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/tests/preview_action_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/tests/required_page_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction/tests/scroll_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_modal_plan.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_options.rs"),
    Path("crates/katana-ui-core-storybook/tests/storybook_panel_scroll_contract.rs"),
)

DOC_FILES = (
    Path("docs/architecture/ui-separation/ui-core-parity-gap.md"),
    Path("docs/architecture/ui-separation/owned-ui-task-map.md"),
    Path("tmp/reports/2026-05-17-overnight-residual-scope.md"),
)


def main() -> int:
    source = "\n".join(path.read_text(encoding="utf-8") for path in STORYBOOK_FILES)
    docs = "\n".join(path.read_text(encoding="utf-8") for path in DOC_FILES)
    required = (
        "StorybookPanel::verify_theme_variants",
        "ThemeSnapshot::light()",
        "ThemeSnapshot::dark()",
        "StorybookStyleSheet",
        "styled_story_roots",
        "StorybookVisual",
        "--open-modal-window",
        "--runtime-regression",
        "modal_window_opened={}",
        "state_reflected={}",
        "overlay_rendered={}",
        "modal_plan_same_display",
        "render_summary()",
        "katana-ui-core-storybook:",
    )
    current_panel_tokens = (
        "PanelRegion::Navigation",
        "PanelRegion::Preview",
        "PanelRegion::Details",
        "navigation_panel",
        "preview_panel",
        "details_panel",
    )
    source_evidence_tokens = (
        "StorybookPanelInteractionReport",
        "CallbackLogReport",
        "selector_operations",
        "overlay_dismissals",
        "color_picker_updates",
        "theme_control",
        "modal_required",
        "theme_difference_pixels",
        "operation_difference_pixels",
        "required_ui_fallbacks",
        "PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT",
        "KATANA_TAB_BORDER_WIDTH",
        "TreeExpansionState",
        "scrollbar_visible",
        "selected_detail_rect",
        "main_window_options",
        "ScaleMode::AspectRatioStretch",
        "operation_preset_changes_tab_and_canvas_pixels",
        "click_mapping_can_select_visible_story_and_change_rendered_scene",
        "clicking_preview_button_emits_action_event_and_changes_rendering",
        "clicking_button_setting_updates_props_and_rendering",
        "button_action_hit_rect",
        "component_action_hit_rect",
        "button_setting_hit_rect",
        "screen controls: active",
        "StorybookInteractionSpec",
        "clicking_selected_preview_emits_component_event_for_non_button_pages",
        "clicking_settings_row_mutates_selected_component_options",
        "storybook_panels_have_independent_vertical_scroll_contract",
        "assert_independent_scroll_states",
        "assert_panel_scroll",
        "vertical_scrollbar_visible",
        "content_height > scroll.viewport_height",
        "PanelScrollOffsets",
        "apply_scroll_delta_at",
        "scroll_delta_updates_only_the_panel_under_pointer",
        "panel_scrollbar_thumbs_move_only_for_scrolled_panel",
        "props_with_option",
        "resolved_after_value",
        "settings_state_uses_actual_option_after_value",
        "legacy_01_24_clicks_emit_expected_action_event_state_and_repaint_body",
        "legacy_01_24_settings_mutate_option_and_repaint_body",
        "legacy_01_24_state_is_isolated_by_page_and_preset",
        "legacy_01_24_catalog_model_contains_expected_core_node_kind",
        "PanelRegionLayout",
        "panel_scrollbar_tracks_stay_inside_own_panel_frames",
        "panel_content_viewports_reserve_scrollbar_gutters",
        "non_overflowing_preview_has_no_scroll_offset",
        "rendered_panel_content_does_not_paint_reserved_scrollbar_gutter",
        "get_unscaled_mouse_pos",
    )
    evidence_tokens = (
        "storybook-panel-interaction-report.json",
        "callback log",
        "required_ui_fallbacks=0",
        "generic `node` fallback",
    )
    missing = [token for token in required if token not in source]
    missing.extend(token for token in current_panel_tokens if token not in source)
    missing.extend(token for token in source_evidence_tokens if token not in source)
    missing.extend(token for token in evidence_tokens if token not in docs)
    forbidden = (
        "framework-native runtime",
        "katana_ui_core_adapter",
        "adapter",
        "adapter::",
        "Application::new()",
        "All components",
        "draw_preview_stories",
        "PREVIEW_VISIBLE_STORIES",
    )
    leaked = []
    for path in STORYBOOK_FILES:
        candidate = path.read_text(encoding="utf-8")
        for token in forbidden:
            if token in candidate:
                leaked.append(f"{path}:{token}")
    if missing:
        print("storybook core-only layout lint failed", file=sys.stderr)
        for token in missing:
            print(f"- missing token: {token}", file=sys.stderr)
        return 1
    if leaked:
        print("storybook must not render through Adapter", file=sys.stderr)
        for token in leaked:
            print(f"- forbidden token: {token}", file=sys.stderr)
        return 1
    mouse_position_leaks = []
    for path in (
        Path("crates/katana-ui-core-storybook/src/visual/window.rs"),
        Path("crates/katana-ui-core-storybook/src/visual/window_interaction.rs"),
    ):
        candidate = path.read_text(encoding="utf-8")
        if ".get_mouse_pos(" in candidate:
            mouse_position_leaks.append(str(path))
    if mouse_position_leaks:
        print("storybook mouse hit testing must use unscaled coordinates", file=sys.stderr)
        for path in mouse_position_leaks:
            print(f"- scaled mouse coordinate remains: {path}", file=sys.stderr)
        return 1
    legacy_setting_sources = (
        Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_detail.rs"),
        Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod.rs"),
        Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction/legacy_dod_options.rs"),
    )
    synthetic_settings = [
        str(path)
        for path in legacy_setting_sources
        if '"-settings"' in path.read_text(encoding="utf-8")
    ]
    if synthetic_settings:
        print("storybook settings mutation must update typed props", file=sys.stderr)
        for path in synthetic_settings:
            print(f"- synthetic settings suffix remains: {path}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
