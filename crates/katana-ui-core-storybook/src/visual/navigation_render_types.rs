use super::navigation_guides::PageDepth;
use super::navigation_tree::{NavigationRow, TreeExpansionState};

#[derive(Clone, Copy)]
pub(super) struct NavigationRenderOptions<'a> {
    pub(super) selected_page: &'a str,
    pub(super) expansion: TreeExpansionState,
    pub(super) scroll_y: usize,
    pub(super) show_lines: bool,
    pub(super) show_text_connectors: bool,
}

#[derive(Clone, Copy)]
pub(super) struct NavigationGuideOptions {
    pub(super) show_lines: bool,
    pub(super) show_text_connectors: bool,
}

#[derive(Clone, Copy)]
pub(super) struct NavigationRowContext<'a> {
    pub(super) rows: &'a [NavigationRow],
    pub(super) row_index: usize,
    pub(super) y: usize,
}

#[derive(Clone, Copy)]
pub(super) struct NavigationBranchContext<'a> {
    pub(super) open: bool,
    pub(super) row: NavigationRowContext<'a>,
    pub(super) guides: NavigationGuideOptions,
}

#[derive(Clone, Copy)]
pub(super) struct NavigationPageContext<'a> {
    pub(super) selected: bool,
    pub(super) depth: PageDepth,
    pub(super) row: NavigationRowContext<'a>,
    pub(super) guides: NavigationGuideOptions,
}
