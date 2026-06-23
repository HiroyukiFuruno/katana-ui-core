#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeViewHitTestInput {
    pub pointer_x: u32,
    pub pointer_y: u32,
    pub scroll_offset_y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeViewAction {
    SelectNode { node_id: String },
    ToggleNode { node_id: String },
    FocusNode { node_id: String },
    HoverNode { node_id: String, hovered: bool },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeViewHitRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeViewHitTarget {
    pub node_id: String,
    pub rect: TreeViewHitRect,
    pub action: TreeViewAction,
    pub hover_action: TreeViewAction,
}
