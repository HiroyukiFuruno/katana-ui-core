mod canvas;
mod card;
mod coverage;
mod dedicated;
mod dedicated_atoms;
mod dedicated_basic;
mod dedicated_common;
mod dedicated_complex;
mod dedicated_feedback;
mod inspector;
mod layout_metrics;
mod modal;
mod navigation;
mod navigation_icons;
mod navigation_tree;
mod palette;
mod preset_tabs;
mod preview;
mod preview_contract;
mod preview_contract_rows;
mod preview_detail;
mod render;
mod render_context;
mod runtime;
mod scrollbar;
mod shell;
mod text;
#[cfg(test)]
mod text_tests;
mod types;
#[cfg(test)]
mod visual_tests;
mod window;
mod window_interaction;
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
        render::render_storybook_canvas_with_options(
            theme_id,
            selected_page,
            preset_index,
            scroll_y,
            scrollbar_visible,
            navigation_tree::TreeExpansionState::default(),
        )
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
