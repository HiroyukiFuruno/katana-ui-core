mod button_options;
mod button_options_draw;
mod canvas;
mod canvas_clip;
mod canvas_color;
mod canvas_model;
mod canvas_png;
mod canvas_rendering;
mod canvas_round_rect;
mod canvas_scale;
mod canvas_viewport;
mod coverage;
mod coverage_legacy_preview;
mod coverage_markers;
mod dedicated;
mod dedicated_atoms;
mod dedicated_attachment_chip;
mod dedicated_attachment_chip_style;
mod dedicated_banner;
mod dedicated_banner_labels;
mod dedicated_banner_style;
mod dedicated_basic;
mod dedicated_breadcrumb;
mod dedicated_breadcrumb_style;
mod dedicated_card;
mod dedicated_card_style;
mod dedicated_chip;
mod dedicated_chip_group;
mod dedicated_chip_group_style;
mod dedicated_chip_style;
mod dedicated_closeable_tab_strip;
mod dedicated_closeable_tab_strip_style;
mod dedicated_collapsible_panel;
mod dedicated_collapsible_panel_style;
mod dedicated_command_palette;
mod dedicated_command_palette_style;
mod dedicated_common;
mod dedicated_complex;
mod dedicated_context_menu;
mod dedicated_context_menu_anchor;
mod dedicated_context_menu_labels;
mod dedicated_context_menu_metrics;
mod dedicated_context_menu_popup;
mod dedicated_diagnostics_list;
mod dedicated_diagnostics_list_style;
mod dedicated_dod_atom_button_live;
mod dedicated_dod_atom_button_live_status;
mod dedicated_dod_atom_button_live_surface;
mod dedicated_dod_atom_buttons;
mod dedicated_dod_atom_divider;
mod dedicated_dod_atom_loading_dots;
mod dedicated_dod_atom_motion;
mod dedicated_dod_atom_primitives;
mod dedicated_dod_atom_progress;
mod dedicated_dod_atom_skeleton;
mod dedicated_dod_atom_slide_control;
mod dedicated_dod_atom_spacer;
mod dedicated_dod_atom_swatch_live;
mod dedicated_dod_atoms;
mod dedicated_dod_common;
mod dedicated_dod_form_binary_choice_layout;
mod dedicated_dod_form_binary_choice_live;
mod dedicated_dod_form_choice_marks;
mod dedicated_dod_form_choice_status;
mod dedicated_dod_form_combo_layout;
mod dedicated_dod_form_combo_live;
mod dedicated_dod_form_field;
mod dedicated_dod_form_field_labels;
mod dedicated_dod_form_input_live;
mod dedicated_dod_form_input_live_layout;
mod dedicated_dod_form_input_live_values;
mod dedicated_dod_form_inputs;
mod dedicated_dod_form_overlays;
mod dedicated_dod_form_segmented_live;
mod dedicated_dod_form_select_live;
mod dedicated_dod_form_selection_list_layout;
mod dedicated_dod_form_selection_list_live;
mod dedicated_dod_forms;
mod dedicated_dod_layout_align_center;
mod dedicated_dod_layout_column;
mod dedicated_dod_layout_grid;
mod dedicated_dod_layout_scroll_area;
mod dedicated_dod_layout_stack;
mod dedicated_dod_layouts;
mod dedicated_dod_metrics;
mod dedicated_dod_molecule_badge;
mod dedicated_dod_molecule_basic;
mod dedicated_dod_molecule_code_diff;
mod dedicated_dod_molecule_color_diff;
mod dedicated_dod_molecule_disclosure;
mod dedicated_dod_molecule_key_cap;
mod dedicated_dod_molecule_menu;
mod dedicated_dod_molecule_split_pane;
mod dedicated_dod_molecule_surfaces;
mod dedicated_dod_molecule_tree;
mod dedicated_dod_molecule_tree_lines;
mod dedicated_dod_molecule_tree_parts;
mod dedicated_dod_molecules;
mod dedicated_dod_runtime_motion;
mod dedicated_dod_status;
mod dedicated_drag_and_drop;
mod dedicated_drag_and_drop_style;
mod dedicated_dynamic_array_editor;
mod dedicated_dynamic_array_editor_style;
mod dedicated_empty_state;
mod dedicated_empty_state_style;
mod dedicated_feedback;
mod dedicated_foundation_panel;
mod dedicated_hover_card;
mod dedicated_hover_card_labels;
mod dedicated_list;
mod dedicated_list_style;
mod dedicated_menu_button;
mod dedicated_modal;
mod dedicated_modal_labels;
mod dedicated_node_labels;
mod dedicated_notification_toast;
mod dedicated_notification_toast_labels;
mod dedicated_search_control_strip;
mod dedicated_search_control_strip_style;
mod dedicated_settings_list;
mod dedicated_settings_list_style;
mod dedicated_shortcut_cheatsheet;
mod dedicated_shortcut_cheatsheet_style;
mod dedicated_shortcut_combo;
mod dedicated_shortcut_combo_style;
mod dedicated_side_menu;
mod dedicated_side_menu_style;
mod dedicated_skeleton_cluster;
mod dedicated_skeleton_cluster_style;
mod dedicated_startup_state_panel;
mod dedicated_startup_state_panel_style;
mod dedicated_status_bar;
mod dedicated_status_bar_style;
mod dedicated_tabs;
mod dedicated_tabs_controls;
mod dedicated_tabs_metrics;
mod dedicated_tabs_strip;
mod dedicated_toast_stack_manager;
mod dedicated_toast_stack_manager_labels;
mod dedicated_toast_stack_manager_style;
mod dedicated_toolbar;
mod dedicated_toolbar_style;
mod dedicated_tooltip;
mod dedicated_virtualization;
mod dedicated_virtualization_style;
mod dedicated_window_control_button_group;
mod dedicated_window_control_button_group_style;
mod inspector;
mod inspector_row_text;
mod inspector_rows;
mod interaction_spec;
mod layout_metrics;
#[cfg(test)]
mod legacy_01_24_contract;
#[cfg(test)]
mod legacy_01_24_contract_tests;
mod modal;
mod navigation;
mod navigation_guides;
mod navigation_icons;
mod navigation_render_types;
mod navigation_tree;
mod palette;
#[cfg(test)]
mod panel_in_panel_behavior_tests;
#[cfg(test)]
mod panel_in_panel_state_tests;
mod panel_layout;
mod panel_options;
mod panel_screen_state;
#[cfg(test)]
mod panel_scroll_contract_tests;
#[cfg(test)]
mod panel_scroll_interaction_tests;
#[cfg(test)]
mod panel_scroll_layout_contract_tests;
#[cfg(test)]
mod panel_scroll_panel_contract_tests;
#[cfg(test)]
mod panel_scroll_panel_interaction_tests;
mod panel_scroll_state;
#[cfg(test)]
mod panel_scroll_state_tests;
mod panel_scrollbar_hit_test;
mod panel_scrollbar_metrics;
#[cfg(test)]
mod panel_scrollbar_metrics_tests;
#[cfg(test)]
mod panel_scrollbar_overflow_model_tests;
mod panel_scrollbars;
mod presentation;
mod preset_tab_scroll;
mod preset_tabs;
mod preview;
mod preview_contract;
mod preview_contract_rows;
mod preview_detail;
mod preview_effects;
mod render;
mod render_context;
mod runtime;
mod screen_state;
mod screen_state_action_bridge;
mod screen_state_context_menu;
mod screen_state_default;
mod screen_state_forms;
mod screen_state_settings;
mod screen_state_tabs;
mod screen_state_tabs_bridge;
mod screen_state_tabs_types;
mod screen_state_text_area;
mod screen_state_text_area_scroll;
mod screen_state_text_input;
mod scrollbar;
mod scrollbar_model;
mod search_box_screen_state;
mod selection_control_metrics;
mod selection_screen_state;
mod selection_screen_state_labels;
mod shell;
mod storybook_ui_option_contract;
mod storybook_ui_runtime_options;
mod storybook_ui_tabs_options;
mod switch_control;
mod text;
#[cfg(test)]
mod text_antialias_tests;
mod text_input_screen_state;
mod text_raster;
mod text_raster_request;
#[cfg(test)]
mod text_test_support;
#[cfg(test)]
mod text_tests;
mod types;
#[cfg(test)]
mod visual_interaction_accordion_tests;
#[cfg(test)]
mod visual_interaction_attachment_chip_tests;
#[cfg(test)]
mod visual_interaction_badge_tests;
#[cfg(test)]
mod visual_interaction_banner_tests;
#[cfg(test)]
mod visual_interaction_breadcrumb_tests;
#[cfg(test)]
mod visual_interaction_button_hover_tests;
#[cfg(test)]
mod visual_interaction_button_summary_tests;
#[cfg(test)]
mod visual_interaction_button_tests;
#[cfg(test)]
mod visual_interaction_card_tests;
#[cfg(test)]
mod visual_interaction_checkbox_tests;
#[cfg(test)]
mod visual_interaction_chip_group_tests;
#[cfg(test)]
mod visual_interaction_chip_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_tests;
#[cfg(test)]
mod visual_interaction_code_diff_tests;
#[cfg(test)]
mod visual_interaction_collapsible_panel_tests;
#[cfg(test)]
mod visual_interaction_color_picker_rgba_tests;
#[cfg(test)]
mod visual_interaction_color_swatch_tests;
#[cfg(test)]
mod visual_interaction_combo_box_tests;
#[cfg(test)]
mod visual_interaction_command_palette_tests;
#[cfg(test)]
mod visual_interaction_context_menu_tests;
#[cfg(test)]
mod visual_interaction_diagnostics_list_tests;
#[cfg(test)]
mod visual_interaction_drag_and_drop_tests;
#[cfg(test)]
mod visual_interaction_dynamic_array_editor_tests;
#[cfg(test)]
mod visual_interaction_empty_state_tests;
#[cfg(test)]
mod visual_interaction_form_field_tests;
#[cfg(test)]
mod visual_interaction_hover_card_tests;
#[cfg(test)]
mod visual_interaction_icon_text_button_tests;
#[cfg(test)]
mod visual_interaction_key_cap_tests;
#[cfg(test)]
mod visual_interaction_list_tests;
#[cfg(test)]
mod visual_interaction_loading_dots_tests;
#[cfg(test)]
mod visual_interaction_menu_button_tests;
#[cfg(test)]
mod visual_interaction_menu_tests;
#[cfg(test)]
mod visual_interaction_modal_overlay_tests;
#[cfg(test)]
mod visual_interaction_modal_tests;
#[cfg(test)]
mod visual_interaction_motion_tests;
#[cfg(test)]
mod visual_interaction_notification_toast_tests;
#[cfg(test)]
mod visual_interaction_popover_tests;
#[cfg(test)]
mod visual_interaction_progress_bar_tests;
#[cfg(test)]
mod visual_interaction_radio_tests;
#[cfg(test)]
mod visual_interaction_search_box_tests;
#[cfg(test)]
mod visual_interaction_search_control_strip_tests;
#[cfg(test)]
mod visual_interaction_segmented_toggle_tests;
#[cfg(test)]
mod visual_interaction_select_box_tests;
#[cfg(test)]
mod visual_interaction_selection_list_tests;
#[cfg(test)]
mod visual_interaction_settings_list_tests;
#[cfg(test)]
mod visual_interaction_shortcut_cheatsheet_tests;
#[cfg(test)]
mod visual_interaction_shortcut_combo_tests;
#[cfg(test)]
mod visual_interaction_side_menu_tests;
#[cfg(test)]
mod visual_interaction_skeleton_cluster_tests;
#[cfg(test)]
mod visual_interaction_skeleton_tests;
#[cfg(test)]
mod visual_interaction_slide_control_tests;
#[cfg(test)]
mod visual_interaction_spinner_tests;
#[cfg(test)]
mod visual_interaction_startup_state_panel_tests;
#[cfg(test)]
mod visual_interaction_status_bar_tests;
#[cfg(test)]
mod visual_interaction_tabs_tests;
#[cfg(test)]
mod visual_interaction_test_support;
#[cfg(test)]
mod visual_interaction_tests;
#[cfg(test)]
mod visual_interaction_text_area_keyboard_tests;
#[cfg(test)]
mod visual_interaction_text_area_scroll_tests;
#[cfg(test)]
mod visual_interaction_text_area_tests;
#[cfg(test)]
mod visual_interaction_text_input_event_tests;
#[cfg(test)]
mod visual_interaction_text_input_layout_tests;
#[cfg(test)]
mod visual_interaction_text_input_tests;
#[cfg(test)]
mod visual_interaction_toast_stack_manager_tests;
#[cfg(test)]
mod visual_interaction_toggle_tests;
#[cfg(test)]
mod visual_interaction_toolbar_tests;
#[cfg(test)]
mod visual_interaction_tooltip_tests;
#[cfg(test)]
mod visual_interaction_tree_view_tests;
#[cfg(test)]
mod visual_interaction_virtualization_tests;
#[cfg(test)]
mod visual_interaction_window_control_button_group_tests;
#[cfg(test)]
mod visual_layout_row_tests;
#[cfg(test)]
mod visual_menu_panel_tests;
#[cfg(test)]
mod visual_navigation_label_support;
#[cfg(test)]
mod visual_navigation_label_tests;
#[cfg(test)]
mod visual_navigation_line_tests;
#[cfg(test)]
mod visual_navigation_support;
#[cfg(test)]
mod visual_navigation_tree_line_continuity_tests;
#[cfg(test)]
mod visual_preset_marker_tests;
#[cfg(test)]
mod visual_preset_tab_scroll_tests;
#[cfg(test)]
mod visual_tests;
mod window;
mod window_coordinates;
mod window_cursor;
mod window_interaction;
mod window_keyboard;
mod window_modal_plan;
mod window_options;

pub use canvas::Canvas;
pub use coverage::StorybookVisualCoverageReport;
pub use runtime::{StorybookRuntimeReport, StorybookVisualError, StorybookWindowRun};
use std::path::Path;
pub use types::StorybookVisual;

impl StorybookVisual {
    #[must_use]
    pub fn render(self) -> Canvas {
        render::render_storybook_canvas()
    }

    pub fn save_png(self, path: &Path) -> image::ImageResult<()> {
        self.render().save_png(path)
    }

    #[must_use]
    pub fn render_scenario(self, theme_id: &str, selected_page: &str, operation: bool) -> Canvas {
        render::render_storybook_canvas_for(theme_id, selected_page, operation)
    }

    pub fn save_scenario_png(
        self,
        path: &Path,
        theme_id: &str,
        selected_page: &str,
        operation: bool,
    ) -> image::ImageResult<()> {
        self.render_scenario(theme_id, selected_page, operation)
            .save_png(path)
    }

    pub fn save_preset_png(
        self,
        path: &Path,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
    ) -> image::ImageResult<()> {
        self.render_preset(theme_id, selected_page, preset_index, 0)
            .save_png(path)
    }

    pub fn save_preset_scrolled_png(
        self,
        path: &Path,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
    ) -> image::ImageResult<()> {
        self.render_preset(theme_id, selected_page, preset_index, scroll_y)
            .save_png(path)
    }

    pub fn save_preset_scrolled_png_with_scrollbar(
        self,
        path: &Path,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
        scrollbar_visible: bool,
    ) -> image::ImageResult<()> {
        self.render_preset_with_scrollbar(
            theme_id,
            selected_page,
            preset_index,
            scroll_y,
            scrollbar_visible,
        )
        .save_png(path)
    }

    pub fn save_clicked_preset_scrolled_png_with_scrollbar(
        self,
        path: &Path,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
        scrollbar_visible: bool,
    ) -> image::ImageResult<()> {
        self.render_clicked_preset_with_scrollbar(
            theme_id,
            selected_page,
            preset_index,
            scroll_y,
            scrollbar_visible,
        )
        .save_png(path)
    }

    #[must_use]
    pub fn render_scrolled(
        self,
        theme_id: &str,
        selected_page: &str,
        operation: bool,
        scroll_y: usize,
    ) -> Canvas {
        render::render_storybook_canvas_scrolled(theme_id, selected_page, operation, scroll_y)
    }

    #[must_use]
    pub fn render_preset(
        self,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
    ) -> Canvas {
        render::render_storybook_canvas_for_preset(theme_id, selected_page, preset_index, scroll_y)
    }

    #[must_use]
    pub fn render_preset_with_scrollbar(
        self,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
        scrollbar_visible: bool,
    ) -> Canvas {
        render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
            theme_id,
            selected_page,
            preset_index,
            preset_tab_scroll_x: preset_tab_scroll::active_index_scroll_x(
                selected_page,
                preset_index,
            ),
            scroll_y,
            scrollbar_visible,
            panel_scroll: panel_scroll_state::PanelScrollOffsets::default(),
            tree_expansion: navigation_tree::TreeExpansionState::default(),
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
            screen_state: screen_state::StorybookScreenState::default(),
        })
    }

    #[must_use]
    pub fn render_clicked_preset_with_scrollbar(
        self,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
        scrollbar_visible: bool,
    ) -> Canvas {
        let mut screen_state = screen_state::StorybookScreenState::default();
        if button_options::is_button_page(selected_page) {
            screen_state.register_button_click(selected_page);
        } else {
            screen_state.register_preview_action(selected_page);
        }
        render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
            theme_id,
            selected_page,
            preset_index,
            preset_tab_scroll_x: preset_tab_scroll::active_index_scroll_x(
                selected_page,
                preset_index,
            ),
            scroll_y,
            scrollbar_visible,
            panel_scroll: panel_scroll_state::PanelScrollOffsets::default(),
            tree_expansion: navigation_tree::TreeExpansionState::default(),
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
            screen_state,
        })
    }

    pub fn save_modal_png(self, path: &Path) -> image::ImageResult<()> {
        modal::render_modal_canvas().save_png(path)
    }

    #[must_use]
    pub fn coverage_report(self) -> StorybookVisualCoverageReport {
        let examples = crate::StoryCatalog.examples();
        coverage::visual_coverage_report(&examples)
    }
}
