mod canvas;
mod card;
mod coverage;
mod dedicated;
mod dedicated_atoms;
mod dedicated_basic;
mod dedicated_common;
mod dedicated_complex;
mod dedicated_feedback;
mod modal;
mod navigation;
mod palette;
mod preview;
mod render;
mod render_context;
mod runtime;
mod shell;
mod text;
#[cfg(test)]
mod text_tests;
mod types;
mod window;

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

    pub fn save_modal_png(self, path: &Path) -> image::ImageResult<()> {
        modal::render_modal_canvas().save_png(path)
    }

    #[must_use]
    pub fn coverage_report(self) -> StorybookVisualCoverageReport {
        let examples = crate::StoryCatalog.examples();
        coverage::visual_coverage_report(&examples)
    }
}

#[cfg(test)]
mod tests {
    use super::{StorybookVisual, palette};

    #[test]
    fn visual_renderer_draws_nonblank_panel() {
        let canvas = StorybookVisual.render();

        assert_eq!(1280, canvas.width());
        assert_eq!(820, canvas.height());
        assert!(canvas.non_background_pixels(palette::DEFAULT_BACKGROUND) > 1000);
    }

    #[test]
    fn visual_renderer_covers_required_ui_without_fallback() {
        let report = StorybookVisual.coverage_report();

        assert_eq!(16, report.required_ui);
        assert!(report.modal_required);
        assert_eq!(0, report.required_ui_fallbacks);
        assert_eq!(0, report.initial_visible_fallbacks);
    }
}
