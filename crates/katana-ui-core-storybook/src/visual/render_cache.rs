use super::canvas::Canvas;
use super::navigation_tree::TreeExpansionState;
use super::panel_scroll_state::PanelScrollOffsets;
use super::render::StorybookRenderOptions;
use super::screen_state::StorybookScreenState;

#[derive(Clone, PartialEq)]
pub(super) struct ContentFrameKey {
    pub(super) theme_id: &'static str,
    pub(super) selected_page: String,
    pub(super) selected_instance_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) preset_tab_scroll_x: usize,
    pub(super) scale_bits: u32,
    pub(super) scrollbar_visible: bool,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) screen_state: StorybookScreenState,
    pub(super) show_navigation_lines: bool,
    pub(super) show_navigation_text_connectors: bool,
}

impl ContentFrameKey {
    pub(super) fn from_options_scaled(
        options: &StorybookRenderOptions<'_>,
        scale_factor: f32,
    ) -> Self {
        let mut panel_scroll = options.panel_scroll;
        panel_scroll.root_x = 0;
        panel_scroll.root_y = 0;
        Self {
            theme_id: theme_key(options.theme_id),
            selected_page: options.selected_page.to_string(),
            selected_instance_id: super::window_interaction::component_instance_id_for_page(
                options.selected_page,
                options.selected_instance_id,
            ),
            preset_index: options.preset_index,
            preset_tab_scroll_x: options.preset_tab_scroll_x,
            scale_bits: scale_factor.to_bits(),
            scrollbar_visible: options.scrollbar_visible,
            panel_scroll,
            tree_expansion: options.tree_expansion,
            screen_state: options.screen_state.clone(),
            show_navigation_lines: options.show_navigation_lines,
            show_navigation_text_connectors: options.show_navigation_text_connectors,
        }
    }
}

pub(super) struct ContentFrameCache {
    pub(super) key: ContentFrameKey,
    pub(super) canvas: Canvas,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StorybookFrameRendererStats {
    pub(super) theme_caches: usize,
    pub(super) content_renders: usize,
    pub(super) content_cache_hits: usize,
}

fn theme_key(theme_id: &str) -> &'static str {
    if theme_id == "light" {
        return "light";
    }
    "dark"
}
