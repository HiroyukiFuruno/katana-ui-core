use super::canvas::Canvas;
use super::navigation_tree::TreeExpansionState;
use super::palette::VisualPalette;
use super::render_context::{RenderContext, ScenarioContext, ShellContext};
use super::scrollbar;
use super::shell;
use super::text::TextRenderer;
use crate::catalog::{StoryCatalog, StorybookStyleSheet};
use crate::panel::StorybookPanel;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::ThemeSnapshot;

pub(super) const WIDTH: usize = 1440;
pub(super) const VIEWPORT_HEIGHT: usize = 920;
pub(super) const CANVAS_HEIGHT: usize = 1840;
pub(super) const HEIGHT: usize = VIEWPORT_HEIGHT;
pub(super) const FRAME_DELAY_MS: u64 = 16;

pub(super) fn render_storybook_canvas() -> Canvas {
    render_storybook_canvas_for("dark", "button", false)
}

pub(super) fn render_storybook_canvas_for(
    theme_id: &str,
    selected_page: &str,
    operation: bool,
) -> Canvas {
    render_storybook_canvas_for_preset(theme_id, selected_page, usize::from(operation), 0)
}

pub(super) fn render_storybook_canvas_scrolled(
    theme_id: &str,
    selected_page: &str,
    operation: bool,
    scroll_y: usize,
) -> Canvas {
    render_storybook_canvas_for_preset(theme_id, selected_page, usize::from(operation), scroll_y)
}

pub(super) fn render_storybook_canvas_for_preset(
    theme_id: &str,
    selected_page: &str,
    preset_index: usize,
    scroll_y: usize,
) -> Canvas {
    render_storybook_canvas_with_options(
        theme_id,
        selected_page,
        preset_index,
        scroll_y,
        true,
        TreeExpansionState::default(),
    )
}

pub(super) fn render_storybook_canvas_with_options(
    theme_id: &str,
    selected_page: &str,
    preset_index: usize,
    scroll_y: usize,
    scrollbar_visible: bool,
    tree_expansion: TreeExpansionState,
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
    let mut canvas = Canvas::new(WIDTH, CANVAS_HEIGHT, palette.background);
    let render = RenderContext {
        text: &text,
        code_text: &code_text,
        examples: &examples,
        style_sheet: &style_sheet,
        palette: &palette,
    };
    let scenario = ScenarioContext {
        selected_page,
        preset_index,
        tree_expansion,
        scrollbar_visible,
    };
    shell::draw(
        &mut canvas,
        ShellContext {
            root: tree.root(),
            render,
            scenario,
        },
    );
    let mut viewport = canvas.viewport_y(scroll_y, VIEWPORT_HEIGHT, palette.background);
    if scrollbar_visible {
        scrollbar::draw(&mut viewport, &palette, scroll_y);
    }
    viewport
}

fn theme_for(theme_id: &str) -> ThemeSnapshot {
    if theme_id == "light" {
        return ThemeSnapshot::light();
    }
    ThemeSnapshot::dark()
}
