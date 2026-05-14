mod lines;
mod ops;
mod render;
mod row_chrome;
#[cfg(test)]
mod tests;
mod types;
mod view;

pub use types::{
    TreeViewExpandTrigger, TreeViewItem, TreeViewLineKind, TreeViewLineStyle, TreeViewNode,
};

use crate::theme::Theme;
use floem::IntoView;

const DEFAULT_ROW_HEIGHT: f32 = 28.0;
const DEFAULT_LINE_COLOR: crate::theme::color::Color = crate::theme::color::Color {
    r: 120,
    g: 120,
    b: 120,
    a: 96,
};

/// Builder for tree widgets.
#[derive(Clone)]
pub struct TreeView {
    props: types::TreeViewProps,
}

impl TreeView {
    #[must_use]
    pub fn new(items: Vec<TreeViewItem>) -> Self {
        Self {
            props: types::TreeViewProps {
                items,
                show_indent_lines: true,
                show_horizontal_lines: false,
                horizontal_line_style: TreeViewLineStyle::new(
                    TreeViewLineKind::Solid,
                    1.0,
                    DEFAULT_LINE_COLOR,
                ),
                show_expand_controls: false,
                expand_trigger: TreeViewExpandTrigger::IconOnly,
                force_open: false,
                row_height: DEFAULT_ROW_HEIGHT,
                virtualized: false,
            },
        }
    }

    #[must_use]
    pub fn from_nodes(nodes: Vec<TreeViewNode>) -> Self {
        Self::new(nodes.into_iter().map(TreeViewItem::from).collect())
    }

    #[must_use]
    pub fn show_indent_lines(mut self, show_indent_lines: bool) -> Self {
        self.props.show_indent_lines = show_indent_lines;
        self
    }

    #[must_use]
    pub fn show_horizontal_lines(mut self, show_horizontal_lines: bool) -> Self {
        self.props.show_horizontal_lines = show_horizontal_lines;
        self
    }

    #[must_use]
    pub fn horizontal_line_style(mut self, style: TreeViewLineStyle) -> Self {
        self.props.horizontal_line_style = style;
        self
    }

    #[must_use]
    pub fn show_expand_controls(mut self, show_expand_controls: bool) -> Self {
        self.props.show_expand_controls = show_expand_controls;
        self
    }

    #[must_use]
    pub fn expand_trigger(mut self, expand_trigger: TreeViewExpandTrigger) -> Self {
        self.props.expand_trigger = expand_trigger;
        self
    }

    #[must_use]
    pub fn force_open(mut self, force_open: bool) -> Self {
        self.props.force_open = force_open;
        self
    }

    #[must_use]
    pub fn row_height(mut self, row_height: f32) -> Self {
        self.props.row_height = row_height;
        self
    }

    #[must_use]
    pub fn virtualized(mut self, virtualized: bool) -> Self {
        self.props.virtualized = virtualized;
        self
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        view::build_view(self.props, theme)
    }
}
