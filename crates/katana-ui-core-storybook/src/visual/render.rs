use super::canvas::Canvas;
use super::navigation_tree::TreeExpansionState;
use super::palette::VisualPalette;
use super::panel_scroll_state::PanelScrollOffsets;
use super::preset_tab_scroll;
#[cfg(test)]
use super::render_cache::StorybookFrameRendererStats;
use super::render_cache::{ContentFrameCache, ContentFrameKey};
use super::render_context::{RenderContext, ScenarioContext, ShellContext};
use super::render_preset_defaults::apply_preset_default_screen_state;
use super::screen_state::StorybookScreenState;
use super::shell;
use super::text::TextRenderer;
use crate::DEFAULT_STORYBOOK_PAGE;
use crate::catalog::StoryCatalog;
use crate::catalog::StoryExample;
use crate::panel::StorybookPanel;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::UiTree;
use katana_ui_core::theme::ThemeSnapshot;
use std::cell::RefCell;

pub(super) const WIDTH: usize = 1440;
pub(super) const VIEWPORT_HEIGHT: usize = 920;
pub(super) const CANVAS_HEIGHT: usize = 3840;
pub(super) const HEIGHT: usize = VIEWPORT_HEIGHT;
pub(super) const FRAME_DELAY_MS: u64 = 16;

#[derive(Clone)]
pub(super) struct StorybookRenderOptions<'a> {
    pub(super) theme_id: &'a str,
    pub(super) selected_page: &'a str,
    pub(super) selected_instance_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) preset_tab_scroll_x: usize,
    pub(super) scroll_y: usize,
    pub(super) scrollbar_visible: bool,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) screen_state: StorybookScreenState,
    pub(super) show_navigation_lines: bool,
    pub(super) show_navigation_text_connectors: bool,
}

pub(super) fn render_storybook_canvas() -> Canvas {
    render_storybook_canvas_for("dark", DEFAULT_STORYBOOK_PAGE, false)
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
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index,
        preset_tab_scroll_x: preset_tab_scroll::active_index_scroll_x(selected_page, preset_index),
        scroll_y,
        scrollbar_visible: true,
        panel_scroll: PanelScrollOffsets::default(),
        tree_expansion: TreeExpansionState::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: StorybookScreenState::default(),
    })
}

pub(super) fn render_storybook_canvas_with_options(
    mut options: StorybookRenderOptions<'_>,
) -> Canvas {
    apply_preset_default_screen_state(&mut options);
    FRAME_RENDERER.with(|renderer| renderer.borrow_mut().render(options))
}

thread_local! {
    static FRAME_RENDERER: RefCell<StorybookFrameRenderer> =
        RefCell::new(StorybookFrameRenderer::new());
}

pub(super) struct StorybookFrameRenderer {
    examples: Vec<StoryExample>,
    light: ThemeFrameCache,
    dark: ThemeFrameCache,
    content_cache: Option<ContentFrameCache>,
    content_renders: usize,
    content_cache_hits: usize,
}

impl StorybookFrameRenderer {
    pub(super) fn new() -> Self {
        let catalog = StoryCatalog;
        let examples = catalog.examples();
        Self {
            light: ThemeFrameCache::new(ThemeSnapshot::light(), &examples),
            dark: ThemeFrameCache::new(ThemeSnapshot::dark(), &examples),
            examples,
            content_cache: None,
            content_renders: 0,
            content_cache_hits: 0,
        }
    }

    pub(super) fn render(&mut self, options: StorybookRenderOptions<'_>) -> Canvas {
        self.render_for_scale(options, 1.0)
    }

    pub(super) fn render_for_scale(
        &mut self,
        mut options: StorybookRenderOptions<'_>,
        scale_factor: f32,
    ) -> Canvas {
        apply_preset_default_screen_state(&mut options);
        let key = ContentFrameKey::from_options_scaled(&options, scale_factor);
        if let Some(cache) = self.content_cache.as_ref().filter(|cache| cache.key == key) {
            self.content_cache_hits += 1;
            let background = self.theme_cache(options.theme_id).background();
            return cache
                .canvas
                .viewport_y(options.scroll_y, VIEWPORT_HEIGHT, background);
        }

        let mut content_options = options.clone();
        content_options.scroll_y = 0;
        content_options.panel_scroll.root_x = 0;
        content_options.panel_scroll.root_y = 0;
        let (canvas, background) = {
            let theme = self.theme_cache(options.theme_id);
            (
                theme.render_content(&self.examples, &content_options, scale_factor),
                theme.background(),
            )
        };
        let viewport = canvas.viewport_y(options.scroll_y, VIEWPORT_HEIGHT, background);
        self.content_renders += 1;
        self.content_cache = Some(ContentFrameCache { key, canvas });
        viewport
    }

    fn theme_cache(&self, theme_id: &str) -> &ThemeFrameCache {
        if theme_id == "light" {
            return &self.light;
        }
        &self.dark
    }

    #[cfg(test)]
    pub(super) fn stats(&self) -> StorybookFrameRendererStats {
        StorybookFrameRendererStats {
            theme_caches: 2,
            content_renders: self.content_renders,
            content_cache_hits: self.content_cache_hits,
        }
    }
}

struct ThemeFrameCache {
    palette: VisualPalette,
    tree: UiTree,
    text: TextRenderer,
    code_text: TextRenderer,
}

impl ThemeFrameCache {
    fn new(theme: ThemeSnapshot, examples: &[StoryExample]) -> Self {
        let facade = UiCoreFacade::new(theme.clone());
        let palette = VisualPalette::from_theme(facade.theme());
        let tree = StorybookPanel::new(theme).build(examples);
        let text = TextRenderer::load(&facade, facade.default_font_role());
        let code_text = TextRenderer::load(&facade, "code");
        Self {
            palette,
            tree,
            text,
            code_text,
        }
    }

    fn render_content(
        &self,
        examples: &[StoryExample],
        options: &StorybookRenderOptions<'_>,
        scale_factor: f32,
    ) -> Canvas {
        let mut canvas =
            Canvas::new_scaled(WIDTH, CANVAS_HEIGHT, scale_factor, self.palette.background);
        let render = RenderContext {
            text: &self.text,
            code_text: &self.code_text,
            examples,
            palette: &self.palette,
        };
        let scenario = ScenarioContext {
            selected_page: options.selected_page,
            selected_instance_id: super::window_interaction::component_instance_id_for_page(
                options.selected_page,
                options.selected_instance_id,
            ),
            preset_index: options.preset_index,
            preset_tab_scroll_x: options.preset_tab_scroll_x,
            tree_expansion: options.tree_expansion,
            scrollbar_visible: options.scrollbar_visible,
            panel_scroll: options.panel_scroll,
            show_navigation_lines: options.show_navigation_lines,
            show_navigation_text_connectors: options.show_navigation_text_connectors,
            screen_state: &options.screen_state,
        };
        shell::draw(
            &mut canvas,
            ShellContext {
                root: self.tree.root(),
                render,
                scenario,
            },
        );
        canvas
    }

    fn background(&self) -> u32 {
        self.palette.background
    }
}

pub(super) fn render_storybook_canvas_with_screen_state(
    theme_id: &str,
    selected_page: &str,
    preset_index: usize,
    screen_state: StorybookScreenState,
) -> Canvas {
    render_storybook_canvas_with_options(StorybookRenderOptions {
        theme_id,
        selected_page,
        selected_instance_id: super::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index,
        preset_tab_scroll_x: preset_tab_scroll::active_index_scroll_x(selected_page, preset_index),
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: PanelScrollOffsets::default(),
        tree_expansion: TreeExpansionState::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state,
    })
}

#[cfg(test)]
mod tests {
    use super::{HEIGHT, StorybookFrameRenderer, StorybookRenderOptions, WIDTH};
    use crate::DEFAULT_STORYBOOK_PAGE;
    use crate::visual::layout_metrics::SCROLL_STEP;
    use crate::visual::navigation_tree::TreeExpansionState;
    use crate::visual::panel_scroll_state::PanelScrollOffsets;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn frame_renderer_reuses_static_assets_and_content_for_root_scroll_frames() {
        let mut renderer = StorybookFrameRenderer::new();
        let screen_state = StorybookScreenState::default();
        renderer.render(options(
            0,
            PanelScrollOffsets::default(),
            screen_state.clone(),
        ));

        let panel_scroll = PanelScrollOffsets {
            root_y: SCROLL_STEP,
            ..PanelScrollOffsets::default()
        };
        renderer.render(options(SCROLL_STEP, panel_scroll, screen_state));

        let stats = renderer.stats();
        assert_eq!(2, stats.theme_caches);
        assert_eq!(1, stats.content_renders);
        assert_eq!(1, stats.content_cache_hits);
    }

    #[test]
    fn render_for_scale_keeps_logical_size_and_scales_physical_size() {
        let mut renderer = StorybookFrameRenderer::new();
        let screen_state = StorybookScreenState::default();
        let options = options(0, PanelScrollOffsets::default(), screen_state);
        let frame = renderer.render_for_scale(options.clone(), 1.0);
        let scaled = renderer.render_for_scale(options, 2.0);

        assert_eq!(WIDTH, frame.logical_width());
        assert_eq!(HEIGHT, frame.logical_height());
        assert_eq!(WIDTH * 2, scaled.width());
        assert_eq!(HEIGHT * 2, scaled.height());
        assert_eq!(frame.logical_width(), scaled.logical_width());
        assert_eq!(frame.logical_height(), scaled.logical_height());
        assert_eq!(2.0, scaled.scale_factor());
    }

    #[test]
    fn frame_renderer_does_not_share_content_cache_across_component_instances() {
        let mut renderer = StorybookFrameRenderer::new();
        let mut primary = options(
            0,
            PanelScrollOffsets::default(),
            StorybookScreenState::default(),
        );
        primary.selected_page = "text-area";
        primary.selected_instance_id = "text-area.primary";
        let mut secondary = primary.clone();
        secondary.selected_instance_id = "text-area.secondary";

        renderer.render(primary);
        renderer.render(secondary);

        let stats = renderer.stats();
        assert_eq!(2, stats.content_renders);
        assert_eq!(0, stats.content_cache_hits);
    }

    fn options(
        scroll_y: usize,
        panel_scroll: PanelScrollOffsets,
        screen_state: StorybookScreenState,
    ) -> StorybookRenderOptions<'static> {
        StorybookRenderOptions {
            theme_id: "dark",
            selected_page: DEFAULT_STORYBOOK_PAGE,
            selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
            preset_index: 0,
            preset_tab_scroll_x: 0,
            scroll_y,
            scrollbar_visible: true,
            panel_scroll,
            tree_expansion: TreeExpansionState::default(),
            screen_state,
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
        }
    }

    #[test]
    fn render_for_scale_uses_separate_content_cache_per_scale() {
        let mut renderer = StorybookFrameRenderer::new();
        let screen_state = StorybookScreenState::default();
        let base_options = options(0, PanelScrollOffsets::default(), screen_state);

        renderer.render_for_scale(base_options.clone(), 1.0);
        renderer.render_for_scale(base_options.clone(), 2.0);
        renderer.render_for_scale(base_options.clone(), 1.0);
        renderer.render_for_scale(base_options, 1.0);

        let stats = renderer.stats();
        assert_eq!(3, stats.content_renders);
        assert_eq!(1, stats.content_cache_hits);
    }
}
