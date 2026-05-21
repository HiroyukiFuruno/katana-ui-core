mod button_options;
mod button_options_draw;
mod canvas;
mod canvas_round_rect;
mod coverage;
mod coverage_markers;
mod dedicated;
mod dedicated_atoms;
mod dedicated_basic;
mod dedicated_common;
mod dedicated_complex;
mod dedicated_context_menu;
mod dedicated_context_menu_anchor;
mod dedicated_context_menu_labels;
mod dedicated_context_menu_metrics;
mod dedicated_context_menu_popup;
mod dedicated_dod_atom_button_live;
mod dedicated_dod_atom_button_live_status;
mod dedicated_dod_atom_button_live_surface;
mod dedicated_dod_atom_buttons;
mod dedicated_dod_atom_motion;
mod dedicated_dod_atom_primitives;
mod dedicated_dod_atom_swatch_live;
mod dedicated_dod_atoms;
mod dedicated_dod_common;
mod dedicated_dod_form_binary_choice_live;
mod dedicated_dod_form_choice_marks;
mod dedicated_dod_form_choice_status;
mod dedicated_dod_form_input_live;
mod dedicated_dod_form_inputs;
mod dedicated_dod_form_overlays;
mod dedicated_dod_form_segmented_live;
mod dedicated_dod_form_select_live;
mod dedicated_dod_forms;
mod dedicated_dod_metrics;
mod dedicated_dod_molecule_basic;
mod dedicated_dod_molecule_code_diff;
mod dedicated_dod_molecule_color_diff;
mod dedicated_dod_molecule_disclosure;
mod dedicated_dod_molecule_key_cap;
mod dedicated_dod_molecule_surfaces;
mod dedicated_dod_molecule_tree;
mod dedicated_dod_molecule_tree_parts;
mod dedicated_dod_molecules;
mod dedicated_dod_status;
mod dedicated_feedback;
mod inspector;
mod inspector_rows;
mod interaction_spec;
mod layout_metrics;
mod modal;
mod navigation;
mod navigation_icons;
mod navigation_tree;
mod palette;
#[cfg(test)]
mod panel_scroll_interaction_tests;
mod panel_scroll_state;
mod panel_scrollbars;
mod presentation;
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
mod scrollbar;
mod scrollbar_model;
mod shell;
mod switch_control;
mod text;
#[cfg(test)]
mod text_tests;
mod types;
#[cfg(test)]
mod visual_interaction_button_tests;
#[cfg(test)]
mod visual_interaction_test_support;
#[cfg(test)]
mod visual_interaction_tests;
#[cfg(test)]
mod visual_tests;
mod window;
mod window_coordinates;
mod window_interaction;
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
            scroll_y,
            scrollbar_visible,
            panel_scroll: panel_scroll_state::PanelScrollOffsets::default(),
            tree_expansion: navigation_tree::TreeExpansionState::default(),
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
            scroll_y,
            scrollbar_visible,
            panel_scroll: panel_scroll_state::PanelScrollOffsets::default(),
            tree_expansion: navigation_tree::TreeExpansionState::default(),
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
