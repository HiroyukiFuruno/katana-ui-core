use super::canvas::Canvas;
use super::palette::VisualPalette;
use super::render_context::{RenderContext, ScenarioContext, ShellContext};
use super::shell;
use super::text::TextRenderer;
use crate::catalog::{StoryCatalog, StorybookStyleSheet};
use crate::panel::StorybookPanel;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::ThemeSnapshot;

pub(super) const WIDTH: usize = 1280;
pub(super) const HEIGHT: usize = 820;
pub(super) const FRAME_DELAY_MS: u64 = 16;

pub(super) fn render_storybook_canvas() -> Canvas {
    render_storybook_canvas_for("dark", "button", false)
}

pub(super) fn render_storybook_canvas_for(
    theme_id: &str,
    selected_page: &str,
    operation: bool,
) -> Canvas {
    let catalog = StoryCatalog;
    let examples = catalog.examples();
    let theme = theme_for(theme_id);
    let facade = UiCoreFacade::new(theme.clone());
    let palette = VisualPalette::from_theme(facade.theme());
    let tree = StorybookPanel::new(theme).build(&examples);
    let style_sheet = StorybookStyleSheet::default_sheet();
    let text = TextRenderer::load(&facade, facade.default_font_role());
    let code_text = TextRenderer::load(&facade, "code");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, palette.background);
    let render = RenderContext {
        text: &text,
        code_text: &code_text,
        examples: &examples,
        style_sheet: &style_sheet,
        palette: &palette,
    };
    let scenario = ScenarioContext {
        selected_page,
        operation,
    };
    shell::draw(
        &mut canvas,
        ShellContext {
            root: tree.root(),
            render,
            scenario,
        },
    );
    canvas
}

fn theme_for(theme_id: &str) -> ThemeSnapshot {
    if theme_id == "light" {
        return ThemeSnapshot::light();
    }
    ThemeSnapshot::dark()
}
