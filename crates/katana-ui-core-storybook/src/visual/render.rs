use super::canvas::Canvas;
use super::navigation_tree::TreeExpansionState;
use super::palette::VisualPalette;
use super::panel_scroll_state::PanelScrollOffsets;
use super::render_context::{RenderContext, ScenarioContext, ShellContext};
use super::screen_state::StorybookScreenState;
use super::scrollbar;
use super::shell;
use super::text::TextRenderer;
use crate::catalog::StoryCatalog;
use crate::panel::StorybookPanel;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::ThemeSnapshot;

pub(super) const WIDTH: usize = 1440;
pub(super) const VIEWPORT_HEIGHT: usize = 920;
pub(super) const CANVAS_HEIGHT: usize = 3840;
pub(super) const HEIGHT: usize = VIEWPORT_HEIGHT;
pub(super) const FRAME_DELAY_MS: u64 = 16;

#[derive(Clone, Copy)]
pub(super) struct StorybookRenderOptions<'a> {
    pub(super) theme_id: &'a str,
    pub(super) selected_page: &'a str,
    pub(super) preset_index: usize,
    pub(super) scroll_y: usize,
    pub(super) scrollbar_visible: bool,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) screen_state: StorybookScreenState,
}

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
    render_storybook_canvas_with_options(StorybookRenderOptions {
        theme_id,
        selected_page,
        preset_index,
        scroll_y,
        scrollbar_visible: true,
        panel_scroll: PanelScrollOffsets::default(),
        tree_expansion: TreeExpansionState::default(),
        screen_state: StorybookScreenState::default(),
    })
}

pub(super) fn render_storybook_canvas_with_options(options: StorybookRenderOptions<'_>) -> Canvas {
    let catalog = StoryCatalog;
    let examples = catalog.examples();
    let theme = theme_for(options.theme_id);
    let facade = UiCoreFacade::new(theme.clone());
    let palette = VisualPalette::from_theme(facade.theme());
    let tree = StorybookPanel::new(theme).build(&examples);
    let text = TextRenderer::load(&facade, facade.default_font_role());
    let code_text = TextRenderer::load(&facade, "code");
    let mut canvas = Canvas::new(WIDTH, CANVAS_HEIGHT, palette.background);
    let render = RenderContext {
        text: &text,
        code_text: &code_text,
        examples: &examples,
        palette: &palette,
    };
    let scenario = ScenarioContext {
        selected_page: options.selected_page,
        preset_index: options.preset_index,
        tree_expansion: options.tree_expansion,
        scrollbar_visible: options.scrollbar_visible,
        panel_scroll: options.panel_scroll,
        screen_state: options.screen_state,
    };
    shell::draw(
        &mut canvas,
        ShellContext {
            root: tree.root(),
            render,
            scenario,
        },
    );
    let mut viewport = canvas.viewport_y(options.scroll_y, VIEWPORT_HEIGHT, palette.background);
    if options.scrollbar_visible {
        scrollbar::draw(&mut viewport, &palette, options.scroll_y);
    }
    viewport
}

#[cfg(test)]
pub(super) fn render_storybook_canvas_with_screen_state(
    theme_id: &str,
    selected_page: &str,
    preset_index: usize,
    screen_state: StorybookScreenState,
) -> Canvas {
    render_storybook_canvas_with_options(StorybookRenderOptions {
        theme_id,
        selected_page,
        preset_index,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: PanelScrollOffsets::default(),
        tree_expansion: TreeExpansionState::default(),
        screen_state,
    })
}

fn theme_for(theme_id: &str) -> ThemeSnapshot {
    if theme_id == "light" {
        return ThemeSnapshot::light();
    }
    ThemeSnapshot::dark()
}
