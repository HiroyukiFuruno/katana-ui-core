use super::{
    UI_DISCLOSURE_TOGGLE_ACTION_ID, UI_IMAGE_HIGHLIGHT_ACTION_ID, UI_LINK_OPEN_ACTION_ID,
    UiHostActionKind, UiHostActionPayload, UiHostActionPlan, UiHostActionSpec, UiTreeNodeKind,
    UiTreeRowActionKind,
};
use super::{
    UiContextMenuItem, UiContextMenuItemKind, UiImageSurfaceHighlight, UiNode, UiNodeKind, UiTree,
};

impl UiHostActionPlan {
    #[must_use]
    pub fn collect_from_tree(tree: &UiTree) -> Vec<Self> {
        Self::collect_from_root(tree.root())
    }

    #[must_use]
    pub fn collect_from_root(root: &UiNode) -> Vec<Self> {
        let mut plans = Vec::new();
        Self::collect_node(root, &mut plans);
        plans
    }

    #[must_use]
    pub fn collect_from_node(node: &UiNode) -> Vec<Self> {
        let mut plans = Vec::new();
        Self::push_node_actions(node, &mut plans);
        plans
    }

    fn collect_node(node: &UiNode, plans: &mut Vec<Self>) {
        Self::push_node_actions(node, plans);
        for child in node.children() {
            Self::collect_node(child, plans);
        }
    }

    fn push_node_actions(node: &UiNode, plans: &mut Vec<Self>) {
        Self::push_explicit_actions(node, plans);
        Self::push_button_command(node, plans);
        Self::push_text_entry_icon_buttons(node, plans);
        Self::push_text_links(node, plans);
        Self::push_accordion_toggle(node, plans);
        Self::push_tree_row_actions(node, plans);
        Self::push_image_highlights(node, plans);
        Self::push_context_menu_items(node, plans);
    }

    fn push_explicit_actions(node: &UiNode, plans: &mut Vec<Self>) {
        let node_enabled = node_enabled(node);
        for action in &node.props().common.host_actions {
            let mut spec = action.clone();
            spec.enabled = spec.enabled && node_enabled;
            plans.push(Self::new(node.id().clone(), spec));
        }
    }

    fn push_button_command(node: &UiNode, plans: &mut Vec<Self>) {
        if !is_button_like(node.kind()) || node.props().button.command.trim().is_empty() {
            return;
        }
        plans.push(Self {
            target: node.id().clone(),
            action_id: node.props().button.command.clone(),
            label: node.props().label.clone(),
            kind: UiHostActionKind::Command,
            enabled: node_enabled(node),
            payload: String::new(),
            typed_payload: UiHostActionPayload::None,
        });
    }

    fn push_text_entry_icon_buttons(node: &UiNode, plans: &mut Vec<Self>) {
        for slot in &node.props().text_entry.trailing_icon_buttons {
            let Some(action) = &slot.action else {
                continue;
            };
            if action.callback.trim().is_empty() {
                continue;
            }
            plans.push(Self {
                target: node.id().clone(),
                action_id: action.callback.clone(),
                label: action.label.clone(),
                kind: UiHostActionKind::Custom,
                enabled: node_enabled(node),
                payload: String::new(),
                typed_payload: UiHostActionPayload::None,
            });
        }
    }

    fn push_text_links(node: &UiNode, plans: &mut Vec<Self>) {
        for span in &node.props().text.spans {
            if span.link_target.trim().is_empty() {
                continue;
            }
            plans.push(Self {
                target: node.id().clone(),
                action_id: UI_LINK_OPEN_ACTION_ID.to_string(),
                label: span.text.clone(),
                kind: UiHostActionKind::Navigation,
                enabled: node_enabled(node),
                payload: span.link_target.clone(),
                typed_payload: UiHostActionPayload::None,
            });
        }
    }

    fn push_accordion_toggle(node: &UiNode, plans: &mut Vec<Self>) {
        if node.kind() != UiNodeKind::Accordion {
            return;
        }
        plans.push(Self {
            target: node.id().clone(),
            action_id: UI_DISCLOSURE_TOGGLE_ACTION_ID.to_string(),
            label: node.props().label.clone(),
            kind: UiHostActionKind::Disclosure,
            enabled: node_enabled(node),
            payload: format!("open={}", node.props().interaction.open),
            typed_payload: UiHostActionPayload::None,
        });
    }

    fn push_tree_row_actions(node: &UiNode, plans: &mut Vec<Self>) {
        if node.kind() != UiNodeKind::TreeView {
            return;
        }
        for tree_node in &node.props().tree.nodes {
            let action_kind = match tree_node.kind {
                UiTreeNodeKind::Directory => UiTreeRowActionKind::Toggle,
                UiTreeNodeKind::File => UiTreeRowActionKind::Select,
            };
            plans.push(Self::new(
                node.id().clone(),
                UiHostActionSpec::tree_row(&tree_node.label, &tree_node.id, action_kind),
            ));
        }
    }

    fn push_image_highlights(node: &UiNode, plans: &mut Vec<Self>) {
        if node.kind() != UiNodeKind::ImageSurface {
            return;
        }
        for highlight in &node.props().image_surface.highlight_rects {
            plans.push(Self {
                target: node.id().clone(),
                action_id: UI_IMAGE_HIGHLIGHT_ACTION_ID.to_string(),
                label: highlight.label.clone(),
                kind: UiHostActionKind::SurfaceControl,
                enabled: node_enabled(node),
                payload: highlight_payload(highlight),
                typed_payload: UiHostActionPayload::None,
            });
        }
    }

    fn push_context_menu_items(node: &UiNode, plans: &mut Vec<Self>) {
        if node.kind() != UiNodeKind::ContextMenu {
            return;
        }
        push_context_menu_item_plans(
            node,
            &node.props().context_menu.items,
            node_enabled(node),
            &[],
            plans,
        );
    }
}

fn is_button_like(kind: UiNodeKind) -> bool {
    matches!(
        kind,
        UiNodeKind::Button
            | UiNodeKind::TextButton
            | UiNodeKind::SvgButton
            | UiNodeKind::IconTextButton
    )
}

fn node_enabled(node: &UiNode) -> bool {
    !node.props().disabled && !node.props().common.disabled
}

fn highlight_payload(highlight: &UiImageSurfaceHighlight) -> String {
    format!(
        "rect={},{},{},{} current={}",
        highlight.rect.x,
        highlight.rect.y,
        highlight.rect.width,
        highlight.rect.height,
        highlight.current
    )
}

fn push_context_menu_item_plans(
    node: &UiNode,
    items: &[UiContextMenuItem],
    parent_enabled: bool,
    parent_path: &[usize],
    plans: &mut Vec<UiHostActionPlan>,
) {
    for (index, item) in items.iter().enumerate() {
        let path = child_path(parent_path, index);
        let item_enabled = parent_enabled && !item.disabled;
        if context_menu_item_dispatches_host_action(item.kind) {
            plans.push(UiHostActionPlan {
                target: node.id().clone(),
                action_id: item.id.clone(),
                label: item.label.clone(),
                kind: context_menu_host_action_kind(item.kind),
                enabled: item_enabled,
                payload: context_menu_item_payload(&path, item.kind),
                typed_payload: UiHostActionPayload::None,
            });
        }
        if context_menu_item_allows_child_dispatch(item.kind) {
            push_context_menu_item_plans(node, &item.children, item_enabled, &path, plans);
        }
    }
}

fn context_menu_item_dispatches_host_action(kind: UiContextMenuItemKind) -> bool {
    matches!(
        kind,
        UiContextMenuItemKind::Action
            | UiContextMenuItemKind::Toggle
            | UiContextMenuItemKind::Radio
    )
}

fn context_menu_item_allows_child_dispatch(kind: UiContextMenuItemKind) -> bool {
    !matches!(
        kind,
        UiContextMenuItemKind::Divider | UiContextMenuItemKind::Section
    )
}

fn context_menu_host_action_kind(kind: UiContextMenuItemKind) -> UiHostActionKind {
    match kind {
        UiContextMenuItemKind::Action => UiHostActionKind::Command,
        UiContextMenuItemKind::Toggle | UiContextMenuItemKind::Radio => UiHostActionKind::Custom,
        UiContextMenuItemKind::Submenu
        | UiContextMenuItemKind::Section
        | UiContextMenuItemKind::Divider => UiHostActionKind::Custom,
    }
}

fn context_menu_item_payload(path: &[usize], kind: UiContextMenuItemKind) -> String {
    format!(
        "path={} kind={}",
        path.iter()
            .map(|index| index.to_string())
            .collect::<Vec<_>>()
            .join("/"),
        context_menu_kind_name(kind)
    )
}

fn context_menu_kind_name(kind: UiContextMenuItemKind) -> &'static str {
    match kind {
        UiContextMenuItemKind::Action => "action",
        UiContextMenuItemKind::Toggle => "toggle",
        UiContextMenuItemKind::Radio => "radio",
        UiContextMenuItemKind::Submenu => "submenu",
        UiContextMenuItemKind::Section => "section",
        UiContextMenuItemKind::Divider => "divider",
    }
}

fn child_path(parent_path: &[usize], child_index: usize) -> Vec<usize> {
    let mut path = parent_path.to_vec();
    path.push(child_index);
    path
}
