use super::ui_tree_canvas_hit_metrics::dimension_px;
use super::ui_tree_canvas_scroll_measure::measured_node_height;
use super::ui_tree_canvas_text::UiTreeTextContext;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const MAX_SCROLL_HEIGHT_CACHE_ENTRIES: usize = 16_384;

#[derive(Default)]
pub(super) struct MeasuredNodeHeightCache {
    heights: HashMap<MeasuredNodeHeightCacheKey, usize>,
}

impl MeasuredNodeHeightCache {
    pub(super) fn height(
        &mut self,
        node: &UiNode,
        text_context: UiTreeTextContext<'_>,
        x: usize,
        area: UiTreeRenderArea,
    ) -> usize {
        let cache_key = MeasuredNodeHeightCacheKey::from_node(node, x, area);
        if let Some(height) = self.heights.get(&cache_key) {
            return *height;
        }
        let height = measured_node_height(node, text_context, x, area);
        if self.heights.len() >= MAX_SCROLL_HEIGHT_CACHE_ENTRIES {
            self.heights.clear();
        }
        self.heights.insert(cache_key, height);
        height
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MeasuredNodeHeightCacheKey {
    node_id_hash: u64,
    kind: UiNodeKind,
    state_id_hash: u64,
    label_hash: u64,
    child_count: usize,
    requested_height: usize,
    requested_width: usize,
    visual_role_hash: u64,
    padding_hash: u64,
    gap: usize,
    text_hash: u64,
    child_signature_hash: u64,
    open: bool,
    x: usize,
    area_width: usize,
}

impl MeasuredNodeHeightCacheKey {
    fn from_node(node: &UiNode, x: usize, area: UiTreeRenderArea) -> Self {
        Self {
            node_id_hash: stable_hash(node.id().as_str()),
            kind: node.kind(),
            state_id_hash: stable_hash(node.props().state_id.as_str()),
            label_hash: stable_hash(&node.props().label),
            child_count: node.children().len(),
            requested_height: dimension_px(&node.props().common.height),
            requested_width: dimension_px(&node.props().common.width),
            visual_role_hash: stable_hash(&format!("{:?}", node.props().visual_role)),
            padding_hash: edge_insets_hash(&node.props().common.padding),
            gap: dimension_px(&node.props().common.gap),
            text_hash: text_props_hash(node),
            child_signature_hash: child_signature_hash(node),
            open: node.props().interaction.open,
            x,
            area_width: area.width,
        }
    }
}

fn edge_insets_hash(insets: &katana_ui_core::render_model::UiEdgeInsets) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    dimension_px(&insets.top).hash(&mut hasher);
    dimension_px(&insets.right).hash(&mut hasher);
    dimension_px(&insets.bottom).hash(&mut hasher);
    dimension_px(&insets.left).hash(&mut hasher);
    hasher.finish()
}

fn text_props_hash(node: &UiNode) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    node.props().text.role.hash(&mut hasher);
    node.props().text.line_height_px.hash(&mut hasher);
    format!("{:?}", node.props().text.wrap).hash(&mut hasher);
    for span in &node.props().text.spans {
        span.text.hash(&mut hasher);
        span.link_target.hash(&mut hasher);
        span.style.bold.hash(&mut hasher);
        span.style.italic.hash(&mut hasher);
        span.style.monospace.hash(&mut hasher);
        span.style.underline.hash(&mut hasher);
        span.style.strikethrough.hash(&mut hasher);
        span.style.highlight.hash(&mut hasher);
        span.style.current_highlight.hash(&mut hasher);
        span.style.inline_code.hash(&mut hasher);
        span.style.inline_math.hash(&mut hasher);
        span.style.emoji.hash(&mut hasher);
        span.style.color_rgba.hash(&mut hasher);
    }
    hasher.finish()
}

fn child_signature_hash(node: &UiNode) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for child in node.children() {
        node_signature_hash(child).hash(&mut hasher);
    }
    hasher.finish()
}

fn node_signature_hash(node: &UiNode) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    node.kind().hash(&mut hasher);
    node.id().as_str().hash(&mut hasher);
    node.props().state_id.as_str().hash(&mut hasher);
    node.props().label.hash(&mut hasher);
    dimension_px(&node.props().common.width).hash(&mut hasher);
    dimension_px(&node.props().common.height).hash(&mut hasher);
    dimension_px(&node.props().common.gap).hash(&mut hasher);
    edge_insets_hash(&node.props().common.padding).hash(&mut hasher);
    edge_insets_hash(&node.props().common.margin).hash(&mut hasher);
    text_props_hash(node).hash(&mut hasher);
    child_signature_hash(node).hash(&mut hasher);
    hasher.finish()
}

fn stable_hash(value: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::text::TextRenderer;
    use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::visual::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn measured_height_cache_reuses_entries_and_evicts_at_capacity() {
        let facade = UiCoreFacade::default();
        let text = TextRenderer::load(&facade, "body");
        let code = TextRenderer::load(&facade, "code");
        let theme = ThemeSnapshot::dark();
        let text_context = UiTreeTextContext {
            text: &text,
            export_text: &text,
            code_text: &code,
            palette: UiTreeCanvasPalette::from_theme(&theme),
            typography: UiTreeDocumentTypography::default(),
        };
        let node = UiNode::new(UiNodeKind::Text, "cached");
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
            scroll_y: 0.0,
        };
        let mut cache = MeasuredNodeHeightCache::default();

        let first = cache.height(&node, text_context, 0, area);
        assert_eq!(first, cache.height(&node, text_context, 0, area));

        cache.heights.clear();
        for x in 0..MAX_SCROLL_HEIGHT_CACHE_ENTRIES {
            cache
                .heights
                .insert(MeasuredNodeHeightCacheKey::from_node(&node, x, area), 1);
        }
        cache.height(&node, text_context, MAX_SCROLL_HEIGHT_CACHE_ENTRIES, area);
        assert_eq!(1, cache.heights.len());
    }
}
