use super::{UiImageSurfaceFit, UiImageSurfaceHighlight, UiImageSurfaceProps};
use super::{UiImageSurfaceTransform, UiNode, UiNodeKind, UiTree};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiImageSurfaceRenderPlan {
    pub fingerprint: String,
    pub width: u32,
    pub height: u32,
    pub rgba_byte_len: usize,
    pub content_scale: u32,
    pub fit: UiImageSurfaceFit,
    pub accessibility_label: String,
    pub selection_text: String,
    pub highlight_rects: Vec<UiImageSurfaceHighlight>,
    pub transform: UiImageSurfaceTransform,
}

impl UiImageSurfaceRenderPlan {
    #[must_use]
    pub fn collect_from_tree(tree: &UiTree) -> Vec<Self> {
        let mut plans = Vec::new();
        Self::collect_from_node(tree.root(), &mut plans);
        plans
    }

    fn collect_from_node(node: &UiNode, plans: &mut Vec<Self>) {
        if node.kind() == UiNodeKind::ImageSurface {
            plans.push(Self::from_props(&node.props().image_surface));
        }
        for child in node.children() {
            Self::collect_from_node(child, plans);
        }
    }

    fn from_props(props: &UiImageSurfaceProps) -> Self {
        Self {
            fingerprint: props.fingerprint.clone(),
            width: props.width,
            height: props.height,
            rgba_byte_len: props.rgba.len(),
            content_scale: props.content_scale,
            fit: props.fit,
            accessibility_label: props.accessibility_label.clone(),
            selection_text: props.selection_text.clone(),
            highlight_rects: props.highlight_rects.clone(),
            transform: props.transform,
        }
    }
}
