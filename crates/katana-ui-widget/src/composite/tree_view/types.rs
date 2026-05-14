use std::rc::Rc;

use crate::primitive::icon::IconSource;
use crate::theme::color::Color;

const NOOP_CALLBACK: fn() = || {};

/// Parent item の開閉をどこで発火するか。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeViewExpandTrigger {
    IconOnly,
    LabelOnly,
    IconAndLabel,
    Disabled,
}

/// 展開中 item の補助線種別。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeViewLineKind {
    Solid,
    Dashed,
    Dotted,
}

/// 展開中 item の水平補助線 style。
#[derive(Clone, Copy, Debug)]
pub struct TreeViewLineStyle {
    pub kind: TreeViewLineKind,
    pub thickness: f32,
    pub color: Color,
}

/// JSON に近い nested input 用の data node。
#[derive(Clone)]
pub struct TreeViewNode {
    pub id: String,
    pub label: String,
    pub icon: Option<IconSource>,
    pub expanded: bool,
    pub active: bool,
    pub disabled: bool,
    pub on_select: Rc<dyn Fn()>,
    pub on_context: Rc<dyn Fn()>,
    pub on_expand: Rc<dyn Fn()>,
    pub on_collapse: Rc<dyn Fn()>,
    pub children: Vec<TreeViewNode>,
}

impl TreeViewNode {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            expanded: false,
            active: false,
            disabled: false,
            on_select: Rc::new(NOOP_CALLBACK),
            on_context: Rc::new(NOOP_CALLBACK),
            on_expand: Rc::new(NOOP_CALLBACK),
            on_collapse: Rc::new(NOOP_CALLBACK),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: IconSource) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn() + 'static) -> Self {
        self.on_select = Rc::new(on_select);
        self
    }

    #[must_use]
    pub fn on_context(mut self, on_context: impl Fn() + 'static) -> Self {
        self.on_context = Rc::new(on_context);
        self
    }

    #[must_use]
    pub fn on_expand(mut self, on_expand: impl Fn() + 'static) -> Self {
        self.on_expand = Rc::new(on_expand);
        self
    }

    #[must_use]
    pub fn on_collapse(mut self, on_collapse: impl Fn() + 'static) -> Self {
        self.on_collapse = Rc::new(on_collapse);
        self
    }

    #[must_use]
    pub fn children(mut self, children: Vec<TreeViewNode>) -> Self {
        self.children = children;
        self
    }
}

impl TreeViewLineStyle {
    #[must_use]
    pub const fn new(kind: TreeViewLineKind, thickness: f32, color: Color) -> Self {
        Self {
            kind,
            thickness,
            color,
        }
    }
}

/// Data model for one row of a tree view.
#[derive(Clone)]
pub struct TreeViewItem {
    pub label: String,
    pub icon: Option<IconSource>,
    pub indent: usize,
    pub expanded: bool,
    pub active: bool,
    pub disabled: bool,
    pub on_select: Rc<dyn Fn()>,
    pub on_context: Rc<dyn Fn()>,
    pub on_expand: Rc<dyn Fn()>,
    pub on_collapse: Rc<dyn Fn()>,
    pub children: Vec<TreeViewItem>,
}

impl TreeViewItem {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            indent: 0,
            expanded: false,
            active: false,
            disabled: false,
            on_select: Rc::new(NOOP_CALLBACK),
            on_context: Rc::new(NOOP_CALLBACK),
            on_expand: Rc::new(NOOP_CALLBACK),
            on_collapse: Rc::new(NOOP_CALLBACK),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: IconSource) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn() + 'static) -> Self {
        self.on_select = Rc::new(on_select);
        self
    }

    #[must_use]
    pub fn on_context(mut self, on_context: impl Fn() + 'static) -> Self {
        self.on_context = Rc::new(on_context);
        self
    }

    #[must_use]
    pub fn on_expand(mut self, on_expand: impl Fn() + 'static) -> Self {
        self.on_expand = Rc::new(on_expand);
        self
    }

    #[must_use]
    pub fn on_collapse(mut self, on_collapse: impl Fn() + 'static) -> Self {
        self.on_collapse = Rc::new(on_collapse);
        self
    }

    #[must_use]
    pub fn children(mut self, children: Vec<TreeViewItem>) -> Self {
        self.children = children;
        self
    }

    #[must_use]
    pub const fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl From<TreeViewNode> for TreeViewItem {
    fn from(node: TreeViewNode) -> Self {
        Self {
            label: node.label,
            icon: node.icon,
            indent: 0,
            expanded: node.expanded,
            active: node.active,
            disabled: node.disabled,
            on_select: node.on_select,
            on_context: node.on_context,
            on_expand: node.on_expand,
            on_collapse: node.on_collapse,
            children: node.children.into_iter().map(Self::from).collect(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct TreeViewProps {
    pub items: Vec<TreeViewItem>,
    pub show_indent_lines: bool,
    pub show_horizontal_lines: bool,
    pub horizontal_line_style: TreeViewLineStyle,
    pub show_expand_controls: bool,
    pub expand_trigger: TreeViewExpandTrigger,
    pub force_open: bool,
    pub row_height: f32,
    pub virtualized: bool,
}
