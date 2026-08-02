use super::ui_tree_canvas_hit_metrics::child_container_x;
use super::ui_tree_canvas_hit_metrics::{NODE_GAP, TEXT_HEIGHT, dimension_px, has_absolute_child};
use super::ui_tree_canvas_image_metrics::logical_image_height;
use super::ui_tree_canvas_text::{UiTreeTextContext, UiTreeTextRenderer};
use super::ui_tree_canvas_tree_parts;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiVisualRole};

pub(super) fn can_render_children_incrementally(node: &UiNode) -> bool {
    is_layout_container(node)
        && node.props().visual_role != UiVisualRole::HoverSurface
        && !has_absolute_child(node)
}

pub(super) fn can_render_partial_media_frame_stack(node: &UiNode) -> bool {
    node.kind() == UiNodeKind::Stack && has_absolute_child(node)
}

pub(super) fn measured_node_height(
    node: &UiNode,
    text_context: UiTreeTextContext<'_>,
    x: usize,
    area: UiTreeRenderArea,
) -> usize {
    let requested = dimension_px(&node.props().common.height);
    if requested > 0 {
        return requested;
    }
    match node.kind() {
        UiNodeKind::TreeView => tree_view_height(node),
        UiNodeKind::Row => node
            .children()
            .iter()
            .map(|child| measured_node_height(child, text_context, x, area))
            .max()
            .unwrap_or(TEXT_HEIGHT),
        UiNodeKind::ScrollArea => node.props().scroll_area.viewport_height as usize,
        UiNodeKind::ImageSurface => {
            let image = &node.props().image_surface;
            logical_image_height(image)
        }
        UiNodeKind::Accordion => accordion_height(node, text_context, x, area),
        UiNodeKind::Text => UiTreeTextRenderer::measure_node_height(text_context, node, x, area),
        UiNodeKind::Checkbox
        | UiNodeKind::Toggle
        | UiNodeKind::Input
        | UiNodeKind::TextArea
        | UiNodeKind::SearchBox
        | UiNodeKind::SelectBox
        | UiNodeKind::ComboBox
        | UiNodeKind::Button
        | UiNodeKind::TextButton
        | UiNodeKind::IconTextButton
        | UiNodeKind::Spinner
        | UiNodeKind::LoadingDots
        | UiNodeKind::Divider => TEXT_HEIGHT,
        _ => container_height(node, text_context, x, area),
    }
}

pub(super) fn container_gap(node: &UiNode) -> usize {
    if !matches!(node.kind(), UiNodeKind::Column) {
        return 0;
    }
    dimension_px(&node.props().common.gap)
}

pub(super) fn child_render_area(
    area: UiTreeRenderArea,
    node: &UiNode,
    child_x: usize,
    padding: ContainerPadding,
) -> UiTreeRenderArea {
    let available_width = area
        .width
        .saturating_sub(padding.left)
        .saturating_sub(padding.right);
    let requested_width = dimension_px(&node.props().common.width);
    let width = if requested_width > 0 {
        requested_width.min(available_width)
    } else {
        available_width
    };
    UiTreeRenderArea {
        x: child_x,
        y: area.y,
        width,
        height: area.height,
        scroll_y: area.scroll_y,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContainerPadding {
    pub(super) left: usize,
    pub(super) right: usize,
    pub(super) top: usize,
    pub(super) bottom: usize,
}

impl ContainerPadding {
    pub(super) fn from_node(node: &UiNode) -> Self {
        Self {
            left: dimension_px(&node.props().common.padding.left),
            right: dimension_px(&node.props().common.padding.right),
            top: dimension_px(&node.props().common.padding.top),
            bottom: dimension_px(&node.props().common.padding.bottom),
        }
    }
}

fn tree_view_height(node: &UiNode) -> usize {
    let label_height = if node.props().label.trim().is_empty() {
        0
    } else {
        ui_tree_canvas_tree_parts::ROW_HEIGHT
    };
    label_height
        .saturating_add(
            node.props()
                .tree
                .nodes
                .len()
                .saturating_mul(ui_tree_canvas_tree_parts::ROW_HEIGHT),
        )
        .saturating_add(NODE_GAP)
}

fn accordion_height(
    node: &UiNode,
    text_context: UiTreeTextContext<'_>,
    x: usize,
    area: UiTreeRenderArea,
) -> usize {
    let mut height = TEXT_HEIGHT;
    if node.props().interaction.open {
        height = height.saturating_add(children_height(node, text_context, x, area));
    }
    height
}

fn container_height(
    node: &UiNode,
    text_context: UiTreeTextContext<'_>,
    x: usize,
    area: UiTreeRenderArea,
) -> usize {
    let mut height = 0usize;
    if !node.props().label.trim().is_empty() && !is_layout_container(node) {
        height = height.saturating_add(TEXT_HEIGHT);
    }
    height
        .saturating_add(dimension_px(&node.props().common.padding.top))
        .saturating_add(children_height(node, text_context, x, area))
        .saturating_add(dimension_px(&node.props().common.padding.bottom))
}

fn children_height(
    node: &UiNode,
    text_context: UiTreeTextContext<'_>,
    x: usize,
    area: UiTreeRenderArea,
) -> usize {
    let gap = container_gap(node);
    let child_count = node.children().len();
    let padding = ContainerPadding::from_node(node);
    let child_x = child_container_x(node, x).saturating_add(padding.left);
    let child_area = child_render_area(area, node, child_x, padding);
    let content_height: usize = node
        .children()
        .iter()
        .map(|child| measured_node_height(child, text_context, child_x, child_area))
        .sum();
    content_height.saturating_add(child_count.saturating_sub(1).saturating_mul(gap))
}

fn is_layout_container(node: &UiNode) -> bool {
    matches!(
        node.kind(),
        UiNodeKind::AlignCenter
            | UiNodeKind::AlignNode
            | UiNodeKind::Column
            | UiNodeKind::Grid
            | UiNodeKind::Stack
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::text::TextRenderer;
    use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::visual::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
    use katana_ui_core::atom::Text;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::molecule::Accordion;
    use katana_ui_core::render_model::UiDimension;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn measured_height_covers_accordion_tree_label_and_explicit_child_width() {
        let context = text_context();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 160,
            scroll_y: 0.0,
        };
        let accordion: UiNode = Accordion::new("Details")
            .open(true)
            .child(Text::new("Body"))
            .into();
        assert!(measured_node_height(&accordion, context, 0, area) > TEXT_HEIGHT);

        let tree = UiNode::new(UiNodeKind::TreeView, "Tree");
        assert_eq!(
            ui_tree_canvas_tree_parts::ROW_HEIGHT + NODE_GAP,
            measured_node_height(&tree, context, 0, area)
        );

        let card = UiNode::new(UiNodeKind::Card, "Card");
        assert_eq!(TEXT_HEIGHT, measured_node_height(&card, context, 0, area));

        let child = UiNode::new(UiNodeKind::Text, "child").width(UiDimension::Px(200));
        let child_area = child_render_area(
            area,
            &child,
            8,
            ContainerPadding {
                left: 10,
                right: 20,
                top: 0,
                bottom: 0,
            },
        );
        assert_eq!(90, child_area.width);
        assert_eq!(8, child_area.x);
    }

    fn text_context() -> UiTreeTextContext<'static> {
        let theme = ThemeSnapshot::dark();
        UiTreeTextContext {
            text: text_renderer("body"),
            export_text: text_renderer("body"),
            code_text: text_renderer("code"),
            palette: UiTreeCanvasPalette::from_theme(&theme),
            typography: UiTreeDocumentTypography::default(),
        }
    }

    fn text_renderer(role: &str) -> &'static TextRenderer {
        let facade = Box::leak(Box::new(UiCoreFacade::default()));
        Box::leak(Box::new(TextRenderer::load(facade, role)))
    }
}
