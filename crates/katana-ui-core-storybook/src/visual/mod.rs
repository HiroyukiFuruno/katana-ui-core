mod button_options;
mod button_options_draw;
mod canvas;
mod canvas_blit;
mod canvas_clip;
mod canvas_color;
mod canvas_model;
mod canvas_physical;
mod canvas_png;
mod canvas_rendering;
mod canvas_round_rect;
mod canvas_scale;
mod canvas_scroll;
mod canvas_text_selection;
mod canvas_viewport;
mod command_chrome_app;
mod command_chrome_artifact;
mod command_chrome_artifact_writer;
mod command_chrome_fixture;
mod command_chrome_runtime;
mod command_chrome_script;
#[cfg(test)]
mod command_chrome_script_tests;
mod command_chrome_script_types;
mod command_chrome_surface;
#[cfg(test)]
mod context_menu_surface;
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
mod dedicated_context_menu_submenu;
mod dedicated_diagnostics_list;
mod dedicated_diagnostics_list_style;
mod dedicated_dod_atom_button_live;
mod dedicated_dod_atom_button_live_status;
mod dedicated_dod_atom_button_live_surface;
mod dedicated_dod_atom_buttons;
mod dedicated_dod_atom_divider;
mod dedicated_dod_atom_icon_grid;
mod dedicated_dod_atom_loading_dots;
mod dedicated_dod_atom_motion;
mod dedicated_dod_atom_primitives;
mod dedicated_dod_atom_progress;
mod dedicated_dod_atom_progress_motion;
mod dedicated_dod_atom_progress_props;
mod dedicated_dod_atom_skeleton;
mod dedicated_dod_atom_slide_control;
mod dedicated_dod_atom_spacer;
mod dedicated_dod_atom_swatch_live;
mod dedicated_dod_atoms;
mod dedicated_dod_common;
mod dedicated_dod_common_blocks;
mod dedicated_dod_form_binary_choice_chrome;
mod dedicated_dod_form_binary_choice_layout;
mod dedicated_dod_form_binary_choice_live;
mod dedicated_dod_form_choice_marks;
mod dedicated_dod_form_choice_status;
mod dedicated_dod_form_combo_layout;
mod dedicated_dod_form_combo_live;
mod dedicated_dod_form_combo_model;
mod dedicated_dod_form_field;
mod dedicated_dod_form_field_labels;
mod dedicated_dod_form_input_live;
mod dedicated_dod_form_input_live_layout;
mod dedicated_dod_form_input_live_values;
mod dedicated_dod_form_inputs;
mod dedicated_dod_form_overlays;
mod dedicated_dod_form_segmented_live;
mod dedicated_dod_form_select_live;
mod dedicated_dod_form_select_live_layout;
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
mod dedicated_dod_molecule_split_pane_labels;
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
mod dedicated_fallback;
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
mod dedicated_tabs_context_menu;
mod dedicated_tabs_controls;
mod dedicated_tabs_icons;
mod dedicated_tabs_layout;
mod dedicated_tabs_metrics;
mod dedicated_tabs_scroll;
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
#[cfg(test)]
mod legacy_01_24_expected_kind;
mod list_screen_state;
mod live_interaction_audit;
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
mod presentation_frame;
mod presentation_frame_scale;
mod presentation_frame_scale_average;
mod preset_tab_label;
mod preset_tab_scroll;
mod preset_tabs;
mod preview;
mod preview_contract;
mod preview_contract_rows;
mod preview_detail;
mod preview_effects;
mod render;
mod render_cache;
mod render_context;
mod render_preset_defaults;
mod runtime;
mod runtime_dependency;
mod screen_state;
mod screen_state_accordion;
mod screen_state_action_bridge;
mod screen_state_breadcrumb;
mod screen_state_button;
mod screen_state_code_diff;
mod screen_state_context_menu;
mod screen_state_default;
mod screen_state_form_field;
mod screen_state_forms;
mod screen_state_forms_bridge;
mod screen_state_hover_card;
mod screen_state_live_component_bridge;
mod screen_state_modal;
mod screen_state_panel_bridge;
mod screen_state_popover;
mod screen_state_search_control;
mod screen_state_segmented_toggle;
mod screen_state_setting_semantics;
mod screen_state_setting_semantics_chip;
mod screen_state_setting_semantics_collection;
mod screen_state_setting_semantics_core;
mod screen_state_setting_semantics_foundation;
mod screen_state_setting_semantics_foundation_extra;
mod screen_state_setting_semantics_live;
mod screen_state_setting_semantics_overlay;
mod screen_state_setting_semantics_selection;
mod screen_state_setting_semantics_specialized;
mod screen_state_setting_semantics_surface;
mod screen_state_setting_semantics_text_entry;
mod screen_state_settings;
mod screen_state_side_menu;
mod screen_state_status_bar;
mod screen_state_tabs;
mod screen_state_tabs_bridge;
mod screen_state_tabs_context;
mod screen_state_tabs_context_close;
mod screen_state_tabs_context_menu_types;
mod screen_state_tabs_core;
mod screen_state_tabs_drag;
mod screen_state_tabs_group_context;
mod screen_state_tabs_keyboard;
mod screen_state_tabs_options;
mod screen_state_tabs_presets;
mod screen_state_tabs_types;
mod screen_state_text_area;
mod screen_state_text_area_clear;
mod screen_state_text_area_core;
mod screen_state_text_area_paste;
mod screen_state_text_area_queries;
mod screen_state_text_area_scroll;
mod screen_state_text_input;
mod screen_state_text_input_selection;
mod screen_state_theme_tokens_bridge;
mod screen_state_toast_stack;
mod screen_state_toolbar;
mod screen_state_tooltip;
mod scrollbar;
mod scrollbar_model;
mod search_box_screen_state;
mod selection_control_metrics;
mod selection_screen_state;
mod selection_screen_state_actions;
mod selection_screen_state_contract;
mod selection_screen_state_core;
mod selection_screen_state_labels;
mod shell;
mod storybook_ui_form_options;
mod storybook_ui_foundation_options;
mod storybook_ui_molecule_options;
mod storybook_ui_option_contract;
mod storybook_ui_runtime_options;
mod storybook_ui_surface_options;
mod storybook_ui_tabs_options;
mod switch_control;
mod text;
#[cfg(test)]
mod text_antialias_band_tests;
#[cfg(test)]
mod text_antialias_tests;
mod text_area_screen_state;
mod text_command_root_storybook;
#[cfg(test)]
mod text_command_surface_integration_tests;
#[cfg(test)]
mod text_emoji_color_tests;
mod text_input_screen_state;
mod text_selection;
mod text_selection_overlay;
mod text_selection_types;
mod text_surface_app;
mod text_surface_artifact;
mod text_surface_artifact_writer;
mod text_surface_fixture;
mod text_surface_runtime;
mod text_surface_script;
mod text_surface_script_steps;
mod text_surface_script_types;
#[cfg(test)]
mod text_test_support;
#[cfg(test)]
mod text_tests;
mod types;
mod ui_tree_canvas;
mod ui_tree_canvas_checkbox;
mod ui_tree_canvas_choice_control;
#[cfg(test)]
mod ui_tree_canvas_compact_heading_tests;
mod ui_tree_canvas_context_menu;
mod ui_tree_canvas_control;
mod ui_tree_canvas_entry;
mod ui_tree_canvas_extensions;
mod ui_tree_canvas_hit;
mod ui_tree_canvas_hit_api;
mod ui_tree_canvas_hit_metrics;
#[cfg(test)]
mod ui_tree_canvas_hit_tests;
mod ui_tree_canvas_hover;
mod ui_tree_canvas_image_blit;
mod ui_tree_canvas_image_cache;
mod ui_tree_canvas_image_cache_entry;
mod ui_tree_canvas_image_cache_key;
#[cfg(test)]
mod ui_tree_canvas_image_cache_tests;
#[cfg(test)]
mod ui_tree_canvas_image_clip_tests;
#[cfg(test)]
mod ui_tree_canvas_image_hover_tests;
mod ui_tree_canvas_image_metrics;
mod ui_tree_canvas_image_raster_cache;
#[cfg(test)]
mod ui_tree_canvas_image_transform_tests;
#[cfg(test)]
mod ui_tree_canvas_interactive_hover_tests;
mod ui_tree_canvas_layout;
mod ui_tree_canvas_loading;
mod ui_tree_canvas_palette;
mod ui_tree_canvas_renderer_image;
mod ui_tree_canvas_rgba;
mod ui_tree_canvas_row_layout;
mod ui_tree_canvas_row_layout_tests;
mod ui_tree_canvas_scroll;
mod ui_tree_canvas_scroll_height_cache;
#[cfg(test)]
mod ui_tree_canvas_scroll_image_support;
#[cfg(test)]
mod ui_tree_canvas_scroll_image_tests;
mod ui_tree_canvas_scroll_measure;
#[cfg(test)]
mod ui_tree_canvas_scroll_offset_tests;
mod ui_tree_canvas_scroll_partial;
#[cfg(test)]
mod ui_tree_canvas_scroll_static_tests;
mod ui_tree_canvas_separator;
mod ui_tree_canvas_settings;
mod ui_tree_canvas_svg_icon;
#[cfg(test)]
mod ui_tree_canvas_table_tests;
#[cfg(test)]
mod ui_tree_canvas_tests;
mod ui_tree_canvas_text;
mod ui_tree_canvas_text_line_width;
mod ui_tree_canvas_text_metrics;
mod ui_tree_canvas_text_role;
#[cfg(test)]
mod ui_tree_canvas_text_wrap_support;
#[cfg(test)]
mod ui_tree_canvas_text_wrap_tests;
mod ui_tree_canvas_tree;
#[cfg(test)]
mod ui_tree_canvas_tree_katana_tests;
mod ui_tree_canvas_tree_parts;
mod ui_tree_canvas_types;
mod ui_tree_interaction_surface;
mod ui_tree_storybook_host;
#[cfg(test)]
mod ui_tree_storybook_host_tests;
mod ui_tree_surface_host;
#[cfg(test)]
mod ui_tree_surface_host_tests;
#[cfg(test)]
mod visual_coverage_edge_tests;
#[cfg(test)]
mod visual_dedicated_fallback_tests;
#[cfg(test)]
mod visual_exhaustive_render_contract_tests;
#[cfg(test)]
mod visual_inspector_button_preset_tests;
#[cfg(test)]
mod visual_inspector_fallback_status_tests;
#[cfg(test)]
mod visual_inspector_option_contract_tests;
#[cfg(test)]
mod visual_inspector_preset_follow_tests;
#[cfg(test)]
mod visual_inspector_text_entry_preset_tests;
#[cfg(test)]
mod visual_interaction_accordion_tests;
#[cfg(test)]
mod visual_interaction_attachment_chip_tests;
#[cfg(test)]
mod visual_interaction_badge_options_tests;
#[cfg(test)]
mod visual_interaction_badge_tests;
#[cfg(test)]
mod visual_interaction_banner_options_tests;
#[cfg(test)]
mod visual_interaction_banner_tests;
#[cfg(test)]
mod visual_interaction_binary_choice_options_tests;
#[cfg(test)]
mod visual_interaction_binary_choice_state_tests;
#[cfg(test)]
mod visual_interaction_breadcrumb_state_tests;
#[cfg(test)]
mod visual_interaction_breadcrumb_tests;
#[cfg(test)]
mod visual_interaction_button_center_tests;
#[cfg(test)]
mod visual_interaction_button_hover_tests;
#[cfg(test)]
mod visual_interaction_button_instance_tests;
#[cfg(test)]
mod visual_interaction_button_summary_tests;
#[cfg(test)]
mod visual_interaction_button_tests;
#[cfg(test)]
mod visual_interaction_card_tests;
#[cfg(test)]
mod visual_interaction_checkbox_disabled_tests;
#[cfg(test)]
mod visual_interaction_checkbox_glyph_tests;
#[cfg(test)]
mod visual_interaction_checkbox_operation_tests;
#[cfg(test)]
mod visual_interaction_checkbox_readability_tests;
#[cfg(test)]
mod visual_interaction_checkbox_snapshot_tests;
#[cfg(test)]
mod visual_interaction_checkbox_state_read_tests;
#[cfg(test)]
mod visual_interaction_checkbox_tests;
#[cfg(test)]
mod visual_interaction_chip_family_options_tests;
#[cfg(test)]
mod visual_interaction_chip_group_tests;
#[cfg(test)]
mod visual_interaction_chip_options_tests;
#[cfg(test)]
mod visual_interaction_chip_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_context_no_group_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_context_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_group_context_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_keyboard_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_options_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_scroll_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_state_tests;
#[cfg(test)]
mod visual_interaction_closeable_tab_strip_tests;
#[cfg(test)]
mod visual_interaction_code_diff_tests;
#[cfg(test)]
mod visual_interaction_collapsible_panel_tests;
#[cfg(test)]
mod visual_interaction_collection_options_tests;
#[cfg(test)]
mod visual_interaction_color_picker_options_tests;
#[cfg(test)]
mod visual_interaction_color_picker_rgba_tests;
#[cfg(test)]
mod visual_interaction_color_swatch_tests;
#[cfg(test)]
mod visual_interaction_combo_box_tests;
#[cfg(test)]
mod visual_interaction_command_palette_options_tests;
#[cfg(test)]
mod visual_interaction_command_palette_tests;
#[cfg(test)]
mod visual_interaction_context_menu_operation_tests;
#[cfg(test)]
mod visual_interaction_context_menu_tests;
#[cfg(test)]
mod visual_interaction_counter_overlap_tests;
#[cfg(test)]
mod visual_interaction_diagnostics_list_options_tests;
#[cfg(test)]
mod visual_interaction_diagnostics_list_tests;
#[cfg(test)]
mod visual_interaction_divider_options_tests;
#[cfg(test)]
mod visual_interaction_divider_tests;
#[cfg(test)]
mod visual_interaction_drag_and_drop_state_tests;
#[cfg(test)]
mod visual_interaction_drag_and_drop_tests;
#[cfg(test)]
mod visual_interaction_dynamic_array_editor_state_tests;
#[cfg(test)]
mod visual_interaction_dynamic_array_editor_tests;
#[cfg(test)]
mod visual_interaction_empty_state_tests;
#[cfg(test)]
mod visual_interaction_feedback_options_tests;
#[cfg(test)]
mod visual_interaction_form_field_tests;
#[cfg(test)]
mod visual_interaction_foundation_extra_options_tests;
#[cfg(test)]
mod visual_interaction_foundation_options_tests;
#[cfg(test)]
mod visual_interaction_hover_card_tests;
#[cfg(test)]
mod visual_interaction_hover_idempotency_tests;
#[cfg(test)]
mod visual_interaction_icon_options_tests;
#[cfg(test)]
mod visual_interaction_icon_tests;
#[cfg(test)]
mod visual_interaction_icon_text_button_tests;
#[cfg(test)]
mod visual_interaction_key_cap_options_tests;
#[cfg(test)]
mod visual_interaction_key_cap_tests;
#[cfg(test)]
mod visual_interaction_layout_options_tests;
#[cfg(test)]
mod visual_interaction_list_tests;
#[cfg(test)]
mod visual_interaction_live_component_options_tests;
#[cfg(test)]
mod visual_interaction_loading_dots_tests;
#[cfg(test)]
mod visual_interaction_menu_button_operation_tests;
#[cfg(test)]
mod visual_interaction_menu_button_tests;
#[cfg(test)]
mod visual_interaction_menu_operation_tests;
#[cfg(test)]
mod visual_interaction_menu_tests;
#[cfg(test)]
mod visual_interaction_modal_overlay_tests;
#[cfg(test)]
mod visual_interaction_modal_tests;
#[cfg(test)]
mod visual_interaction_motion_tests;
#[cfg(test)]
mod visual_interaction_navigation_options_tests;
#[cfg(test)]
mod visual_interaction_notification_toast_tests;
#[cfg(test)]
mod visual_interaction_overlay_options_tests;
#[cfg(test)]
mod visual_interaction_popover_tests;
#[cfg(test)]
mod visual_interaction_primitive_options_tests;
#[cfg(test)]
mod visual_interaction_progress_bar_indeterminate_tests;
#[cfg(test)]
mod visual_interaction_progress_bar_inspector_tests;
#[cfg(test)]
mod visual_interaction_progress_bar_loading_options_tests;
#[cfg(test)]
mod visual_interaction_progress_bar_runtime_tests;
#[cfg(test)]
mod visual_interaction_progress_bar_tests;
#[cfg(test)]
mod visual_interaction_radio_operation_tests;
#[cfg(test)]
mod visual_interaction_radio_tests;
#[cfg(test)]
mod visual_interaction_resize_route_tests;
#[cfg(test)]
mod visual_interaction_runtime_options_tests;
#[cfg(test)]
mod visual_interaction_runtime_structured_assertions;
#[cfg(test)]
mod visual_interaction_runtime_structured_options_tests;
#[cfg(test)]
mod visual_interaction_search_box_tests;
#[cfg(test)]
mod visual_interaction_search_control_options_tests;
#[cfg(test)]
mod visual_interaction_search_control_strip_tests;
#[cfg(test)]
mod visual_interaction_segmented_toggle_state_tests;
#[cfg(test)]
mod visual_interaction_segmented_toggle_tests;
#[cfg(test)]
mod visual_interaction_select_box_tests;
#[cfg(test)]
mod visual_interaction_selection_instance_tests;
#[cfg(test)]
mod visual_interaction_selection_list_preset_tests;
#[cfg(test)]
mod visual_interaction_selection_list_tests;
#[cfg(test)]
mod visual_interaction_selection_options_tests;
#[cfg(test)]
mod visual_interaction_settings_list_options_tests;
#[cfg(test)]
mod visual_interaction_settings_list_tests;
#[cfg(test)]
mod visual_interaction_shortcut_cheatsheet_options_tests;
#[cfg(test)]
mod visual_interaction_shortcut_cheatsheet_tests;
#[cfg(test)]
mod visual_interaction_shortcut_combo_tests;
#[cfg(test)]
mod visual_interaction_side_menu_tests;
#[cfg(test)]
mod visual_interaction_skeleton_cluster_tests;
#[cfg(test)]
mod visual_interaction_skeleton_options_tests;
#[cfg(test)]
mod visual_interaction_skeleton_tests;
#[cfg(test)]
mod visual_interaction_slide_control_tests;
#[cfg(test)]
mod visual_interaction_spacer_options_tests;
#[cfg(test)]
mod visual_interaction_spacer_tests;
#[cfg(test)]
mod visual_interaction_spinner_tests;
#[cfg(test)]
mod visual_interaction_split_pane_options_tests;
#[cfg(test)]
mod visual_interaction_startup_state_panel_tests;
#[cfg(test)]
mod visual_interaction_status_bar_options_tests;
#[cfg(test)]
mod visual_interaction_status_bar_state_tests;
#[cfg(test)]
mod visual_interaction_status_bar_tests;
#[cfg(test)]
mod visual_interaction_surface_gesture_tests;
#[cfg(test)]
mod visual_interaction_surface_options_tests;
#[cfg(test)]
mod visual_interaction_svg_button_options_tests;
#[cfg(test)]
mod visual_interaction_tabs_context_group_tests;
#[cfg(test)]
mod visual_interaction_tabs_context_support;
#[cfg(test)]
mod visual_interaction_tabs_context_tests;
#[cfg(test)]
mod visual_interaction_tabs_group_move_tests;
#[cfg(test)]
mod visual_interaction_tabs_keyboard_tests;
#[cfg(test)]
mod visual_interaction_tabs_options_tests;
#[cfg(test)]
mod visual_interaction_tabs_order_tests;
#[cfg(test)]
mod visual_interaction_tabs_parity_tests;
#[cfg(test)]
mod visual_interaction_tabs_pin_tests;
#[cfg(test)]
mod visual_interaction_tabs_scroll_tests;
#[cfg(test)]
mod visual_interaction_tabs_state_tests;
#[cfg(test)]
mod visual_interaction_tabs_tests;
#[cfg(test)]
mod visual_interaction_test_support;
#[cfg(test)]
mod visual_interaction_tests;
#[cfg(test)]
mod visual_interaction_text_area_clear_tests;
#[cfg(test)]
mod visual_interaction_text_area_hover_tests;
#[cfg(test)]
mod visual_interaction_text_area_keyboard_tests;
#[cfg(test)]
mod visual_interaction_text_area_scroll_tests;
#[cfg(test)]
mod visual_interaction_text_area_state_tests;
#[cfg(test)]
mod visual_interaction_text_area_tests;
#[cfg(test)]
mod visual_interaction_text_entry_options_tests;
#[cfg(test)]
mod visual_interaction_text_input_clear_tests;
#[cfg(test)]
mod visual_interaction_text_input_event_tests;
#[cfg(test)]
mod visual_interaction_text_input_hover_tests;
#[cfg(test)]
mod visual_interaction_text_input_layout_tests;
#[cfg(test)]
mod visual_interaction_text_input_state_tests;
#[cfg(test)]
mod visual_interaction_text_input_tests;
#[cfg(test)]
mod visual_interaction_text_selection_paste_tests;
#[cfg(test)]
mod visual_interaction_text_selection_tests;
#[cfg(test)]
mod visual_interaction_text_tests;
#[cfg(test)]
mod visual_interaction_theme_tokens_tests;
#[cfg(test)]
mod visual_interaction_toast_stack_manager_tests;
#[cfg(test)]
mod visual_interaction_toggle_state_tests;
#[cfg(test)]
mod visual_interaction_toggle_tests;
#[cfg(test)]
mod visual_interaction_toolbar_options_tests;
#[cfg(test)]
mod visual_interaction_toolbar_state_tests;
#[cfg(test)]
mod visual_interaction_toolbar_tests;
#[cfg(test)]
mod visual_interaction_tooltip_tests;
#[cfg(test)]
mod visual_interaction_tree_view_scroll_tests;
#[cfg(test)]
mod visual_interaction_tree_view_tests;
#[cfg(test)]
mod visual_interaction_virtualization_options_tests;
#[cfg(test)]
mod visual_interaction_virtualization_tests;
#[cfg(test)]
mod visual_interaction_window_control_button_group_tests;
#[cfg(test)]
mod visual_kuc_dependency_edge_tests;
#[cfg(test)]
mod visual_layout_align_center_tests;
#[cfg(test)]
mod visual_layout_live_stack_grid_tests;
#[cfg(test)]
mod visual_layout_live_tests;
#[cfg(test)]
mod visual_layout_row_tests;
#[cfg(test)]
mod visual_layout_scroll_area_geometry_tests;
#[cfg(test)]
mod visual_layout_scroll_split_tests;
#[cfg(test)]
mod visual_layout_split_tests;
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
mod window_frame;
mod window_interaction;
mod window_keyboard;
mod window_modal_plan;
mod window_mouse_trace;
mod window_options;
mod window_pair;
mod window_text_caret;

pub use canvas::Canvas;
pub use coverage::StorybookVisualCoverageReport;
pub use live_interaction_audit::StorybookLiveInteractionAuditReport;
pub use presentation::StorybookPresentation;
pub use runtime::{
    StorybookDependencyRuntimeReport, StorybookKeyboardRuntimeReport,
    StorybookMouseTraceRuntimeReport, StorybookRuntimeReport, StorybookVisualError,
    StorybookWindowRun,
};
use std::path::Path;
pub use text::TextRenderer;
pub use text_command_root_storybook::FullRootArtifactError;
pub use text_selection::SelectableTextRun;
pub use text_surface_script_types::TextSurfaceArtifactError;
pub use types::StorybookVisual;
pub use ui_tree_canvas::UiTreeCanvasRenderer;
pub use ui_tree_canvas_types::{
    CanvasBlitRequest, RgbaBlitRequest, UiTreeHitRect, UiTreeHostActionHit,
    UiTreeHostActionHitQuery, UiTreeInteractionTarget, UiTreeNodeHit, UiTreeRenderArea,
};
pub use ui_tree_interaction_surface::UiTreeInteractionSurface;
pub use ui_tree_storybook_host::UiTreeStorybookHost;
pub use ui_tree_surface_host::UiTreeSurfaceHost;

impl StorybookVisual {
    pub fn write_text_command_root_artifact(
        output_dir: &std::path::Path,
    ) -> Result<(), FullRootArtifactError> {
        text_command_root_storybook::write_artifact(output_dir)
    }

    pub fn write_text_surface_artifact(
        output_dir: &std::path::Path,
    ) -> Result<(), TextSurfaceArtifactError> {
        text_surface_artifact_writer::write_scripted_artifact(output_dir)
    }

    pub fn write_command_chrome_artifact(
        output_dir: &std::path::Path,
    ) -> Result<(), command_chrome_script_types::CommandChromeArtifactError> {
        command_chrome_artifact_writer::write_scripted_artifact(output_dir)
    }
}

impl StorybookVisual {
    #[must_use]
    pub fn dependency_runtime_report(self) -> StorybookDependencyRuntimeReport {
        runtime_dependency::runtime_report()
    }

    #[must_use]
    pub fn keyboard_runtime_report(self) -> StorybookKeyboardRuntimeReport {
        window_keyboard::runtime_report()
    }

    #[must_use]
    pub fn mouse_trace_runtime_report(self) -> StorybookMouseTraceRuntimeReport {
        window_mouse_trace::runtime_report()
    }

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
            selected_instance_id: window_interaction::DEFAULT_INSTANCE_ID,
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
        let screen_state = clicked_preset_screen_state(selected_page, preset_index);
        render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
            theme_id,
            selected_page,
            selected_instance_id: window_interaction::DEFAULT_INSTANCE_ID,
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

    #[must_use]
    pub fn live_interaction_audit_report(self) -> StorybookLiveInteractionAuditReport {
        live_interaction_audit::live_interaction_audit_report()
    }
}

fn clicked_preset_screen_state(
    selected_page: &str,
    preset_index: usize,
) -> screen_state::StorybookScreenState {
    let mut screen_state = screen_state::StorybookScreenState::default();
    if selected_page == "checkbox" && preset_index == 2 {
        return screen_state;
    }
    if selected_page == "checkbox" && preset_index == 1 {
        screen_state.apply_checkbox_checked_preset_default();
    }
    if button_options::is_button_page(selected_page) {
        screen_state.register_button_click(selected_page);
    } else {
        screen_state.register_preview_action(selected_page);
    }
    screen_state
}
