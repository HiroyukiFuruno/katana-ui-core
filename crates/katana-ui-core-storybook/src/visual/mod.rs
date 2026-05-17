mod canvas;
mod card;
mod modal;
mod render;
mod runtime;
mod text;
mod types;
mod window;

pub use canvas::Canvas;
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
}

#[cfg(test)]
mod tests {
    use super::{StorybookVisual, render};

    #[test]
    fn visual_renderer_draws_nonblank_panel() {
        let canvas = StorybookVisual.render();

        assert_eq!(1280, canvas.width());
        assert_eq!(820, canvas.height());
        assert!(canvas.non_background_pixels(render::BACKGROUND) > 1000);
    }
}
