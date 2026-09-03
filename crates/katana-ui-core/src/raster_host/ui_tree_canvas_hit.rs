use super::text::TextRenderer;
use super::ui_tree_canvas_checkbox::{checkbox_row_height, checkbox_row_width};
use super::ui_tree_canvas_hit_metrics::{
    INDENT, NODE_GAP, TEXT_HEIGHT, absolute_child_rect, button_dimensions, child_container_x,
    dimension_px, frame_height, has_absolute_child, is_absolute, remaining_width, render_origin_y,
    should_draw_container_label, toggle_dimensions,
};
use super::ui_tree_canvas_image_metrics::{
    image_target_size, logical_image_height_exact, logical_image_width_exact,
};
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_row_layout::UiTreeRowLayout;
use super::ui_tree_canvas_scroll_height_cache::MeasuredNodeHeightCache;
use super::ui_tree_canvas_scroll_measure::{
    ContainerPadding as ScrollContainerPadding, can_render_children_incrementally,
    child_render_area as scroll_child_render_area, container_gap as scroll_container_gap,
};
use super::ui_tree_canvas_text::{UiTreeTextContext, UiTreeTextRenderer};
use super::ui_tree_canvas_text_metrics::{UiTreeDocumentTypography, UiTreeTextMetrics};
use super::ui_tree_canvas_text_role::UiTreeTextRoleRenderer;
use super::ui_tree_canvas_tree_parts as tree_parts;
use super::ui_tree_canvas_types::{
    UiTreeHitRect, UiTreeHostActionHit, UiTreeNodeHit, UiTreeRenderArea,
};
#[path = "ui_tree_canvas_hit_geometry.rs"]
mod geometry;
use geometry::{
    ContainerPadding, ScrollHitClip, child_render_area, clip_scroll_hit, duplicate_panel_label,
    node_cursor, scroll_source_y, whitespace_width,
};
#[cfg(test)]
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::molecule::SettingsListLayoutMetrics;
use katana_ui_core::render_model::{
    UI_LINK_OPEN_ACTION_ID, UiCursor, UiHostActionPlan, UiNode, UiNodeId, UiNodeKind, UiTextSpan,
};
use katana_ui_core::theme::ThemeSnapshot;

#[path = "ui_tree_canvas_hit_methods.rs"]
mod methods;

pub(super) struct UiTreeHostActionHitCollector<'a> {
    area: UiTreeRenderArea,
    actions: Vec<UiHostActionPlan>,
    hits: Vec<UiTreeHostActionHit>,
    node_hits: Vec<UiTreeNodeHit>,
    y: usize,
    text: &'a TextRenderer,
    export_text: &'a TextRenderer,
    code_text: &'a TextRenderer,
    typography: UiTreeDocumentTypography,
    scroll_clip: ScrollHitClip,
    semantic_node_id: Option<UiNodeId>,
    height_cache: MeasuredNodeHeightCache,
}

impl UiTreeHostActionHitCollector<'_> {
    #[cfg(test)]
    pub(super) fn collect(root: &UiNode, area: UiTreeRenderArea) -> Vec<UiTreeHostActionHit> {
        let facade = UiCoreFacade::default();
        let text = TextRenderer::load(&facade, facade.default_font_role());
        let export_text = TextRenderer::load(&facade, facade.default_font_role());
        let code_text = TextRenderer::load(&facade, "code");
        Self::collect_with_renderers(
            root,
            area,
            &text,
            &export_text,
            &code_text,
            UiTreeDocumentTypography::default(),
        )
    }

    pub(super) fn collect_with_renderers<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
    ) -> Vec<UiTreeHostActionHit> {
        Self::collect_with_scroll_clip(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            ScrollHitClip::Viewport,
        )
    }

    pub(super) fn collect_viewport_with_renderers<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
    ) -> Vec<UiTreeHostActionHit> {
        let mut collector = Self::collector(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            ScrollHitClip::Document,
        );
        collector.node(root, 0);
        clip_action_hits_to_viewport(collector.hits, area)
    }

    pub(super) fn collect_viewport_interaction_with_renderers<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
    ) -> (Vec<UiTreeHostActionHit>, Vec<UiTreeNodeHit>) {
        let mut collector = Self::collector(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            ScrollHitClip::Document,
        );
        collector.node(root, 0);
        (
            clip_action_hits_to_viewport(collector.hits, area),
            clip_node_hits_to_viewport(collector.node_hits, area),
        )
    }

    pub(super) fn collect_node_hits_with_renderers<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
    ) -> Vec<UiTreeNodeHit> {
        let mut collector = Self::node_hit_collector(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            ScrollHitClip::Document,
        );
        collector.node(root, area.x);
        collector.node_hits
    }

    pub(super) fn collect_viewport_node_hits_with_renderers<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
    ) -> Vec<UiTreeNodeHit> {
        let mut collector = Self::node_hit_collector(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            ScrollHitClip::Document,
        );
        collector.node(root, 0);
        clip_node_hits_to_viewport(collector.node_hits, area)
    }

    pub(super) fn collect_document_with_renderers<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
    ) -> Vec<UiTreeHostActionHit> {
        Self::collect_with_scroll_clip(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            ScrollHitClip::Document,
        )
    }

    fn collect_with_scroll_clip<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
        scroll_clip: ScrollHitClip,
    ) -> Vec<UiTreeHostActionHit> {
        let mut collector = Self::collector(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            scroll_clip,
        );
        collector.node(root, area.x);
        collector.hits
    }

    fn node_hit_collector<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
        scroll_clip: ScrollHitClip,
    ) -> UiTreeHostActionHitCollector<'a> {
        Self::collector_with_actions(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            scroll_clip,
            Vec::new(),
        )
    }

    fn collector<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
        scroll_clip: ScrollHitClip,
    ) -> UiTreeHostActionHitCollector<'a> {
        let actions = if scroll_clip == ScrollHitClip::Viewport {
            Vec::new()
        } else {
            UiHostActionPlan::collect_from_root(root)
        };
        Self::collector_with_actions(
            root,
            area,
            text,
            export_text,
            code_text,
            typography,
            scroll_clip,
            actions,
        )
    }

    fn collector_with_actions<'a>(
        root: &UiNode,
        area: UiTreeRenderArea,
        text: &'a TextRenderer,
        export_text: &'a TextRenderer,
        code_text: &'a TextRenderer,
        typography: UiTreeDocumentTypography,
        scroll_clip: ScrollHitClip,
        actions: Vec<UiHostActionPlan>,
    ) -> UiTreeHostActionHitCollector<'a> {
        UiTreeHostActionHitCollector {
            area,
            actions,
            hits: Vec::new(),
            node_hits: Vec::new(),
            y: render_origin_y(root, area),
            text,
            export_text,
            code_text,
            typography,
            scroll_clip,
            semantic_node_id: None,
            height_cache: MeasuredNodeHeightCache::default(),
        }
    }
}

fn clip_action_hits_to_viewport(
    hits: Vec<UiTreeHostActionHit>,
    area: UiTreeRenderArea,
) -> Vec<UiTreeHostActionHit> {
    hits.into_iter()
        .filter_map(|mut hit| {
            hit.rect = clip_document_rect_to_viewport(hit.rect, area)?;
            Some(hit)
        })
        .collect()
}

fn clip_node_hits_to_viewport(
    hits: Vec<UiTreeNodeHit>,
    area: UiTreeRenderArea,
) -> Vec<UiTreeNodeHit> {
    hits.into_iter()
        .filter_map(|mut hit| {
            hit.rect = clip_document_rect_to_viewport(hit.rect, area)?;
            Some(hit)
        })
        .collect()
}

fn clip_document_rect_to_viewport(
    rect: UiTreeHitRect,
    area: UiTreeRenderArea,
) -> Option<UiTreeHitRect> {
    let source_y = area.scroll_y.round().max(0.0) as usize;
    let visible_left = rect.x;
    let visible_right = rect.x.saturating_add(rect.width).min(area.width);
    let visible_top = rect.y.max(source_y);
    let visible_bottom = rect
        .y
        .saturating_add(rect.height)
        .min(source_y.saturating_add(area.height));
    if visible_right <= visible_left || visible_bottom <= visible_top {
        return None;
    }
    Some(UiTreeHitRect {
        x: area.x.saturating_add(visible_left),
        y: area.y.saturating_add(visible_top.saturating_sub(source_y)),
        width: visible_right.saturating_sub(visible_left),
        height: visible_bottom.saturating_sub(visible_top),
    })
}

fn semantic_node_id(node: &UiNode) -> Option<UiNodeId> {
    let value = node.props().common.semantic_node_id.trim();
    if value.is_empty() {
        return None;
    }
    Some(value.to_string().into())
}

#[cfg(test)]
mod viewport_clip_tests {
    use super::{UiTreeHitRect, UiTreeRenderArea, clip_document_rect_to_viewport};

    #[test]
    fn empty_document_rect_is_not_hittable() {
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
            scroll_y: 0.0,
        };

        assert_eq!(
            None,
            clip_document_rect_to_viewport(
                UiTreeHitRect {
                    x: 20,
                    y: 0,
                    width: 0,
                    height: 10,
                },
                area
            )
        );
        assert_eq!(
            None,
            clip_document_rect_to_viewport(
                UiTreeHitRect {
                    x: 0,
                    y: 20,
                    width: 10,
                    height: 0,
                },
                area
            )
        );
    }
}
