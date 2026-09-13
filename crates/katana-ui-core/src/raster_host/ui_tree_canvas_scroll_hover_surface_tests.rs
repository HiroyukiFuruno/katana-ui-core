use super::{auto_hover_text_child, logical_node_height};
use crate::atom::Text;
use crate::raster_host::{
    Canvas, UiTreeCanvasRenderer, UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost,
    UiTreeTextRoleBaselineTypography,
};
use crate::render_model::{
    UiDimension, UiNode, UiNodeId, UiNodeKind, UiScrollAreaProps, UiTree, UiVisualRole,
};
use crate::theme::ThemeSnapshot;

const EXPLICIT_HEIGHT: u16 = 40;
const FRACTIONAL_LINE_BOX: f32 = 31.5;
const AREA_WIDTH: usize = 160;
const AREA_HEIGHT: usize = 80;
const PARTIAL_SCROLL_OFFSET: u32 = 16;

#[test]
fn auto_hover_text_child_requires_an_automatic_single_text_surface() {
    let automatic = auto_hover_text_surface(UiDimension::Auto, vec![text_child()]);

    assert_eq!(
        Some(UiNodeKind::Text),
        auto_hover_text_child(&automatic).map(UiNode::kind)
    );
    assert!(auto_hover_text_child(&UiNode::new(UiNodeKind::Button, "button")).is_none());
    assert!(auto_hover_text_child(&UiNode::new(UiNodeKind::Stack, "")).is_none());
    assert!(
        auto_hover_text_child(&auto_hover_text_surface(
            UiDimension::Px(EXPLICIT_HEIGHT),
            vec![text_child()]
        ))
        .is_none()
    );
    assert!(
        auto_hover_text_child(&auto_hover_text_surface(UiDimension::Auto, Vec::new())).is_none()
    );
    assert!(
        auto_hover_text_child(&auto_hover_text_surface(
            UiDimension::Auto,
            vec![text_child(), text_child()]
        ))
        .is_none()
    );
    assert!(
        auto_hover_text_child(&auto_hover_text_surface(
            UiDimension::Auto,
            vec![UiNode::new(UiNodeKind::Divider, "divider")]
        ))
        .is_none()
    );
}

#[test]
fn logical_height_delegates_only_automatic_hover_text_surfaces() {
    let renderer =
        UiTreeCanvasRenderer::with_document_typography(
            ThemeSnapshot::dark(),
            UiTreeDocumentTypography::new().with_body_baseline(
                UiTreeTextRoleBaselineTypography::new(14.0, FRACTIONAL_LINE_BOX, 16.0),
            ),
        );
    let theme = ThemeSnapshot::dark();
    let automatic = auto_hover_text_surface(UiDimension::Auto, vec![text_child()]);
    let explicit = auto_hover_text_surface(UiDimension::Px(EXPLICIT_HEIGHT), vec![text_child()]);

    assert_eq!(
        FRACTIONAL_LINE_BOX,
        logical_node_height(
            &renderer,
            renderer.text_context(
                crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(&theme)
            ),
            &automatic,
            0,
            area(),
        )
    );
    assert_eq!(
        f32::from(EXPLICIT_HEIGHT),
        logical_node_height(
            &renderer,
            renderer.text_context(
                crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(&theme)
            ),
            &explicit,
            0,
            area(),
        )
    );
}

#[test]
fn partial_hover_text_clips_pixels_to_the_target_and_preserves_two_following_glyphs() {
    let host =
        UiTreeSurfaceHost::with_document_typography(
            ThemeSnapshot::dark(),
            UiTreeDocumentTypography::new().with_body_baseline(
                UiTreeTextRoleBaselineTypography::new(14.0, FRACTIONAL_LINE_BOX, 16.0),
            ),
        );
    let normal = partial_scroll_tree(false);
    let hovered = partial_scroll_tree(true);
    let normal_hits = host.viewport_node_hits(normal.root(), area());
    let target = node_rect(&normal_hits, "target");
    let first_following = node_rect(&normal_hits, "following");
    let second_following = node_rect(&normal_hits, "following-second");
    let mut normal_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
    let mut hovered_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);

    host.render(&mut normal_canvas, normal.root(), area());
    host.render(&mut hovered_canvas, hovered.root(), area());

    assert_hover_diff_stays_inside_target(&normal_canvas, &hovered_canvas, target);
    assert_canvas_rect_is_unchanged(&normal_canvas, &hovered_canvas, first_following);
    assert_canvas_rect_is_unchanged(&normal_canvas, &hovered_canvas, second_following);
}

fn auto_hover_text_surface(height: UiDimension, children: Vec<UiNode>) -> UiNode {
    children.into_iter().fold(
        UiNode::new(UiNodeKind::Stack, "")
            .height(height)
            .visual_role(UiVisualRole::HoverSurface),
        UiNode::child,
    )
}

fn text_child() -> UiNode {
    UiNode::from(Text::new("Text").text_role("body"))
}

fn partial_scroll_tree(hovered: bool) -> UiTree {
    let content = UiNode::new(UiNodeKind::Column, "")
        .child(partial_scroll_text("target"))
        .child(partial_scroll_text("following"))
        .child(partial_scroll_text("following-second"));
    let tree = UiTree::new(
        UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                offset_y: PARTIAL_SCROLL_OFFSET,
                viewport_width: AREA_WIDTH as u32,
                viewport_height: AREA_HEIGHT as u32,
                content_width: AREA_WIDTH as u32,
                content_height: (AREA_HEIGHT as u32).saturating_add(PARTIAL_SCROLL_OFFSET),
                ..UiScrollAreaProps::default()
            })
            .child(content),
    );
    if hovered {
        tree.with_hover_surface_for_node_id(Some(&UiNodeId::new("target")))
    } else {
        tree
    }
}

fn partial_scroll_text(node_id: &str) -> UiNode {
    UiNode::from(Text::new("Text").text_role("body")).stable_node_id(UiNodeId::new(node_id))
}

fn assert_hover_diff_stays_inside_target(
    normal: &Canvas,
    hovered: &Canvas,
    target: crate::raster_host::UiTreeHitRect,
) {
    let mut differences_inside_target = 0;
    for (index, (normal_pixel, hovered_pixel)) in
        normal.pixels().iter().zip(hovered.pixels()).enumerate()
    {
        if normal_pixel == hovered_pixel {
            continue;
        }
        let x = index % AREA_WIDTH;
        let y = index / AREA_WIDTH;
        assert!(
            x >= target.x
                && x < target.x.saturating_add(target.width)
                && y >= target.y
                && y < target.y.saturating_add(target.height),
            "partial hover must not paint outside the target"
        );
        differences_inside_target += 1;
    }
    assert!(
        differences_inside_target > 0,
        "partial hover must paint inside the target"
    );
}

fn assert_canvas_rect_is_unchanged(
    normal: &Canvas,
    hovered: &Canvas,
    rect: crate::raster_host::UiTreeHitRect,
) {
    for y in rect.y..rect.y.saturating_add(rect.height) {
        for x in rect.x..rect.x.saturating_add(rect.width) {
            assert_eq!(
                normal.pixels()[y * AREA_WIDTH + x],
                hovered.pixels()[y * AREA_WIDTH + x],
                "hover must not shift following glyphs"
            );
        }
    }
}

fn node_rect(
    hits: &[crate::raster_host::UiTreeNodeHit],
    node_id: &str,
) -> crate::raster_host::UiTreeHitRect {
    hits.iter()
        .find(|hit| hit.node_id.as_str() == node_id)
        .expect("viewport node hit")
        .rect
}

fn area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: AREA_HEIGHT,
        scroll_y: 0.0,
    }
}
