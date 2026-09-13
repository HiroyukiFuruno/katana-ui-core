use crate::atom::{Spacer, Text};
use crate::raster_host::{
    Canvas, UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost,
    UiTreeTextRoleBaselineTypography,
};
use crate::render_model::{
    UiDimension, UiHostActionSpec, UiNode, UiNodeId, UiNodeKind, UiScrollAreaProps, UiTree,
};
use crate::test_assert::KucTestExpect;
use crate::theme::ThemeSnapshot;

const AREA_WIDTH: usize = 160;
const AREA_HEIGHT: usize = 96;
const TEXT_CLIP_HEIGHT: u16 = 32;
const HEADING_FONT_SIZE: f32 = 20.0;
const HEADING_BASELINE: f32 = 20.0;
const BODY_FONT_SIZE: f32 = 14.0;
const BODY_BASELINE: f32 = 16.0;
const FRACTIONAL_LINE_BOX: f32 = 31.5;
const SCROLL_OFFSET: u32 = 2;
const SCROLL_VIEWPORT_HEIGHT: u32 = 64;

#[test]
fn hover_surface_keeps_fractional_document_text_and_following_hits_in_place() {
    let host = fractional_host();
    let normal = document_tree(false);
    let hovered = document_tree(true);

    assert_hover_keeps_document_hits(&host, &normal, &hovered);
    assert_following_pixels_are_unchanged(&host, &normal, &hovered);
}

#[test]
fn hover_surface_keeps_legacy_text_and_following_hits_in_place() {
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    assert_hover_keeps_document_hits(&host, &legacy_tree(false), &legacy_tree(true));
}

#[test]
fn hover_surface_keeps_scrolled_document_hits_in_place() {
    let host = fractional_host();
    let normal = scroll_tree(false);
    let hovered = scroll_tree(true);

    assert_hover_keeps_document_hits(&host, &normal, &hovered);
}

#[test]
fn hover_surface_keeps_explicit_scrolled_text_and_following_hits_in_place() {
    let host = fractional_host();
    let normal = explicit_scroll_tree(false);
    let hovered = explicit_scroll_tree(true);

    assert_hover_keeps_document_hits(&host, &normal, &hovered);
}

fn assert_hover_keeps_document_hits(host: &UiTreeSurfaceHost, normal: &UiTree, hovered: &UiTree) {
    let normal_nodes = host.document_node_hits(normal.root(), area());
    let hovered_nodes = host.document_node_hits(hovered.root(), area());
    let normal_actions = host.document_host_action_hits(normal.root(), area());
    let hovered_actions = host.document_host_action_hits(hovered.root(), area());

    assert_eq!(
        node_rect(&normal_nodes, "target"),
        node_rect(&hovered_nodes, "target"),
        "the child text keeps its logical document hit"
    );
    assert_eq!(
        node_rect(&normal_nodes, "following"),
        node_rect(&hovered_nodes, "following"),
        "the wrapper must not advance the following document node"
    );
    assert_eq!(
        action_rect(&normal_actions, "following"),
        action_rect(&hovered_actions, "following"),
        "the wrapper must not advance the following host action"
    );
}

fn assert_following_pixels_are_unchanged(
    host: &UiTreeSurfaceHost,
    normal: &UiTree,
    hovered: &UiTree,
) {
    let following = node_rect(&host.document_node_hits(normal.root(), area()), "following");
    let mut normal_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
    let mut hovered_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);

    host.render(&mut normal_canvas, normal.root(), area());
    host.render(&mut hovered_canvas, hovered.root(), area());

    for y in following.y..following.y.saturating_add(following.height) {
        for x in following.x..following.x.saturating_add(following.width) {
            assert_eq!(
                normal_canvas.pixels()[y * AREA_WIDTH + x],
                hovered_canvas.pixels()[y * AREA_WIDTH + x],
                "hover must not shift following glyphs"
            );
        }
    }
}

fn fractional_host() -> UiTreeSurfaceHost {
    UiTreeSurfaceHost::with_document_typography(
        ThemeSnapshot::dark(),
        UiTreeDocumentTypography::new()
            .with_heading_1_baseline(UiTreeTextRoleBaselineTypography::new(
                HEADING_FONT_SIZE,
                FRACTIONAL_LINE_BOX,
                HEADING_BASELINE,
            ))
            .with_body_baseline(UiTreeTextRoleBaselineTypography::new(
                BODY_FONT_SIZE,
                FRACTIONAL_LINE_BOX,
                BODY_BASELINE,
            )),
    )
}

fn document_tree(hovered: bool) -> UiTree {
    hover_tree(
        UiNode::new(UiNodeKind::Column, "")
            .child(UiNode::from(Text::new("Heading").text_role("heading-1")))
            .child(target_text("paragraph"))
            .child(following_text()),
        hovered,
    )
}

fn legacy_tree(hovered: bool) -> UiTree {
    hover_tree(
        UiNode::new(UiNodeKind::Column, "")
            .child(target_text("body"))
            .child(following_text()),
        hovered,
    )
}

fn scroll_tree(hovered: bool) -> UiTree {
    scroll_tree_with_target(hovered, scroll_target_text())
}

fn explicit_scroll_tree(hovered: bool) -> UiTree {
    scroll_tree_with_target(hovered, target_text("paragraph"))
}

fn scroll_tree_with_target(hovered: bool, target: UiNode) -> UiTree {
    let content = UiNode::new(UiNodeKind::Column, "")
        .child(target)
        .child(following_text())
        .child(UiNode::from(Spacer::new("")).height(UiDimension::px(SCROLL_OFFSET as u16)));
    hover_tree(
        UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                offset_y: SCROLL_OFFSET,
                viewport_width: AREA_WIDTH as u32,
                viewport_height: SCROLL_VIEWPORT_HEIGHT,
                content_width: AREA_WIDTH as u32,
                content_height: SCROLL_VIEWPORT_HEIGHT + SCROLL_OFFSET,
                ..UiScrollAreaProps::default()
            })
            .child(content),
        hovered,
    )
}

fn hover_tree(root: UiNode, hovered: bool) -> UiTree {
    let tree = UiTree::new(root);
    if hovered {
        tree.with_hover_surface_for_node_id(Some(&UiNodeId::new("target")))
    } else {
        tree
    }
}

fn target_text(role: &str) -> UiNode {
    UiNode::from(Text::new("Target").text_role(role))
        .height(UiDimension::px(TEXT_CLIP_HEIGHT))
        .stable_node_id(UiNodeId::new("target"))
        .host_action(UiHostActionSpec::command("target", "Target"))
}

fn scroll_target_text() -> UiNode {
    UiNode::from(Text::new("Target").text_role("paragraph"))
        .stable_node_id(UiNodeId::new("target"))
        .host_action(UiHostActionSpec::command("target", "Target"))
}

fn following_text() -> UiNode {
    UiNode::from(Text::new("Following").text_role("paragraph"))
        .stable_node_id(UiNodeId::new("following"))
        .host_action(UiHostActionSpec::command("following", "Following"))
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

fn node_rect(
    hits: &[crate::raster_host::UiTreeNodeHit],
    node_id: &str,
) -> crate::raster_host::UiTreeHitRect {
    hits.iter()
        .find(|hit| hit.node_id.as_str() == node_id)
        .kuc_expect("document node hit")
        .rect
}

fn action_rect(
    hits: &[crate::raster_host::UiTreeHostActionHit],
    action_id: &str,
) -> crate::raster_host::UiTreeHitRect {
    hits.iter()
        .find(|hit| hit.action.action_id == action_id)
        .kuc_expect("document host action hit")
        .rect
}
