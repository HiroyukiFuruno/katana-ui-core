use super::navigation_tree::TreeExpansionState;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use crate::catalog::StoryExample;
use katana_ui_core::render_model::UiNode;
use katana_ui_core::style::StyleSheet;

#[derive(Clone, Copy)]
pub(super) struct RenderContext<'a> {
    pub(super) text: &'a TextRenderer,
    pub(super) code_text: &'a TextRenderer,
    pub(super) examples: &'a [StoryExample],
    pub(super) style_sheet: &'a StyleSheet,
    pub(super) palette: &'a VisualPalette,
}

#[derive(Clone, Copy)]
pub(super) struct ScenarioContext<'a> {
    pub(super) selected_page: &'a str,
    pub(super) preset_index: usize,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) scrollbar_visible: bool,
}

#[derive(Clone, Copy)]
pub(super) struct ShellContext<'a> {
    pub(super) root: &'a UiNode,
    pub(super) render: RenderContext<'a>,
    pub(super) scenario: ScenarioContext<'a>,
}

#[derive(Clone, Copy)]
pub(super) struct PreviewContext<'a> {
    pub(super) preview: &'a UiNode,
    pub(super) render: RenderContext<'a>,
    pub(super) selected_page: &'a str,
}
