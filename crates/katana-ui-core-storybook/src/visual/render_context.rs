use super::navigation_tree::TreeExpansionState;
use super::palette::VisualPalette;
use super::panel_scroll_state::PanelScrollOffsets;
use super::screen_state::StorybookScreenState;
use super::text::TextRenderer;
use crate::catalog::StoryExample;
use katana_ui_core::render_model::UiNode;

#[derive(Clone, Copy)]
pub(super) struct RenderContext<'a> {
    pub(super) text: &'a TextRenderer,
    pub(super) code_text: &'a TextRenderer,
    pub(super) examples: &'a [StoryExample],
    pub(super) palette: &'a VisualPalette,
}

#[derive(Clone, Copy)]
pub(super) struct ScenarioContext<'a> {
    pub(super) selected_page: &'a str,
    pub(super) selected_instance_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) preset_tab_scroll_x: usize,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) scrollbar_visible: bool,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) screen_state: &'a StorybookScreenState,
    pub(super) show_navigation_lines: bool,
    pub(super) show_navigation_text_connectors: bool,
}

#[cfg(test)]
impl<'a> ScenarioContext<'a> {
    pub(super) fn for_test(
        selected_page: &'a str,
        preset_index: usize,
        screen_state: &'a StorybookScreenState,
    ) -> Self {
        Self {
            selected_page,
            selected_instance_id: "primary",
            preset_index,
            preset_tab_scroll_x: 0,
            tree_expansion: TreeExpansionState::default(),
            scrollbar_visible: true,
            panel_scroll: PanelScrollOffsets::default(),
            screen_state,
            show_navigation_lines: false,
            show_navigation_text_connectors: false,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ShellContext<'a> {
    pub(super) root: &'a UiNode,
    pub(super) render: RenderContext<'a>,
    pub(super) scenario: ScenarioContext<'a>,
}
