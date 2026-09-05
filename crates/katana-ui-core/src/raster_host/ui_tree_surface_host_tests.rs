use super::{
    Canvas, UiTreeDocumentTypography, UiTreeHitRect, UiTreeNodeHit, UiTreeRenderArea,
    UiTreeSurfaceHost, UiTreeTextRoleTypography,
};
use crate::test_assert::KucTestExpect;
use crate::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::atom::{Text, Toggle};
use katana_ui_core::render_model::{
    UiCursor, UiDimension, UiHostActionSpec, UiNode, UiNodeId, UiNodeKind,
};
use katana_ui_core::theme::ThemeSnapshot;

const TEST_AREA_WIDTH: usize = 240;
const TEST_AREA_HEIGHT: usize = 80;

#[test]
fn surface_host_renders_with_the_opt_in_first_candidate_face_policy() {
    let root: UiNode = Text::new("KatanA consumer text").into();
    let host = UiTreeSurfaceHost::with_text_raster_config(
        ThemeSnapshot::dark(),
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::FirstCandidate,
    );
    let mut canvas = Canvas::new(TEST_AREA_WIDTH, TEST_AREA_HEIGHT, 0);

    host.render(&mut canvas, &root, test_area());

    assert!(canvas.pixels().iter().any(|pixel| *pixel != 0));
}

#[test]
fn surface_host_returns_rendered_action_cursor_and_hover_from_one_contract() {
    let root = toggle_root();
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let area = test_area();
    let hit = host
        .host_action_hits_at(&root, area, 20.0, 20.0)
        .into_iter()
        .next()
        .kuc_expect("toggle host action hit");
    let (x, y) = hit.center_point();
    let hits = vec![hit];

    assert_eq!(UiCursor::Pointer, UiTreeSurfaceHost::cursor_at(&hits, x, y));
    assert_eq!(
        Some(UiNodeId::new("dark-toggle")),
        UiTreeSurfaceHost::hovered_action_node_id_at(&hits, x, y)
    );
}

#[test]
fn surface_host_returns_document_node_hits_for_text_nodes() {
    let root = UiNode::new(UiNodeKind::Text, "Hello").stable_node_id(UiNodeId::new("text-node"));
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let hits = host.document_node_hits(&root, test_area());

    assert!(hits.iter().any(|hit| hit.node_id.as_str() == "text-node"));
}

#[test]
fn surface_host_text_node_hit_respects_explicit_height() {
    let root: UiNode = Text::new("Hover text").text_role("heading-3").into();
    let root = root
        .height(UiDimension::px(30))
        .stable_node_id(UiNodeId::new("text-node"));
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let hit = host
        .document_node_hits(&root, test_area())
        .into_iter()
        .find(|hit| hit.node_id.as_str() == "text-node")
        .kuc_expect("text-node hit");

    assert_eq!(30, hit.rect.height);
}

#[test]
fn surface_host_document_typography_shares_raster_and_node_hit_metrics() {
    let document_typography = UiTreeDocumentTypography::new()
        .with_body(UiTreeTextRoleTypography::new(16.5, 23, 0))
        .with_heading_1(UiTreeTextRoleTypography::new(24.75, 40, 9));
    let body: UiNode = Text::new("WWWW").text_role("body").into();
    let heading: UiNode = Text::new("WWWW").text_role("heading").into();
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(body.stable_node_id(UiNodeId::new("body")))
        .child(heading.stable_node_id(UiNodeId::new("heading")));
    let host =
        UiTreeSurfaceHost::with_document_typography(ThemeSnapshot::dark(), document_typography);
    let mut canvas = Canvas::new(TEST_AREA_WIDTH, TEST_AREA_HEIGHT, 0);

    host.render(&mut canvas, &root, test_area());
    let hits = host.document_node_hits(&root, test_area());
    let body_hit = hits
        .iter()
        .find(|hit| hit.node_id.as_str() == "body")
        .kuc_expect("body node hit");
    let heading_hit = hits
        .iter()
        .find(|hit| hit.node_id.as_str() == "heading")
        .kuc_expect("heading node hit");

    assert_eq!(23, body_hit.rect.height);
    assert_eq!(23, heading_hit.rect.y);
    assert_eq!(40, heading_hit.rect.height);
    assert!(non_background_width(&canvas, 0, 23) > 0);
    assert!(non_background_width(&canvas, 23, 63) > non_background_width(&canvas, 0, 23));
}

#[test]
fn surface_host_hovered_node_id_prefers_semantic_node_id() {
    let hit = raster_host_hit(
        "generated-text-node",
        Some("viewer-block-node"),
        0,
        0,
        100,
        24,
    );

    assert_eq!(
        Some(UiNodeId::new("viewer-block-node")),
        UiTreeSurfaceHost::hovered_node_id_at(&[hit], 10.0, 10.0)
    );
}

#[test]
fn surface_host_hovered_node_id_falls_back_to_rendered_node_id() {
    let hit = raster_host_hit("rendered-node", None, 0, 0, 100, 24);

    assert_eq!(
        Some(UiNodeId::new("rendered-node")),
        UiTreeSurfaceHost::hovered_node_id_at(&[hit], 10.0, 10.0)
    );
}

#[test]
fn surface_host_document_node_hits_inherit_parent_semantic_node_id() {
    let text: UiNode = Text::new("Nested item").into();
    let column = UiNode::new(UiNodeKind::Column, "").child(text);
    let common = column
        .props()
        .common
        .clone()
        .semantic_node_id("viewer-list-node");
    let root = column.common(common);
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let hit = host
        .document_node_hits(&root, test_area())
        .into_iter()
        .find(|hit| hit.node_id.as_str().starts_with("Text:"))
        .kuc_expect("nested text hit");

    assert_eq!(
        Some(UiNodeId::new("viewer-list-node")),
        hit.semantic_node_id
    );
}

#[test]
fn surface_host_public_entrypoints_share_the_same_rendered_tree() {
    let root = toggle_root();
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let area = test_area();
    let mut canvas = Canvas::new(TEST_AREA_WIDTH, TEST_AREA_HEIGHT, 0);

    host.render(&mut canvas, &root, area);
    let document_actions = host.document_host_action_hits(&root, area);
    let viewport_actions = host.viewport_host_action_hits(&root, area);
    let all_actions = host.host_action_hits(&root, area);
    let viewport_nodes = host.viewport_node_hits(&root, area);
    let (interaction_actions, interaction_nodes) = host.viewport_interaction_hits(&root, area);

    assert_eq!(document_actions, all_actions);
    assert_eq!(viewport_actions, interaction_actions);
    assert_eq!(viewport_nodes, interaction_nodes);
    let hit = viewport_actions
        .first()
        .kuc_expect("viewport toggle action");
    let (x, y) = hit.center_point();
    assert_eq!(
        host.host_action_hits_at(&root, area, x, y),
        UiTreeSurfaceHost::hits_at(&viewport_actions, x, y)
    );
    assert_eq!(
        host.interaction_target_at(&root, area, x, y),
        UiTreeSurfaceHost::interaction_target_for_hits_at(&viewport_actions, &viewport_nodes, x, y,)
    );
    assert!(UiTreeSurfaceHost::context_menu_item_id_at(&root, x, y).is_none());
    assert!(UiTreeSurfaceHost::context_menu_host_action_at(&root, x, y).is_none());
    assert!(UiTreeSurfaceHost::context_menu_item_center_for_id(&root, "missing").is_none());
    assert!(canvas.pixels().iter().any(|pixel| *pixel != 0));
}

fn toggle_root() -> UiNode {
    UiNode::from(Toggle::new("Dark").checked(true))
        .stable_node_id(UiNodeId::new("dark-toggle"))
        .host_action(UiHostActionSpec::command("ui.toggle.dark", "Toggle dark"))
}

const fn test_area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: TEST_AREA_WIDTH,
        height: TEST_AREA_HEIGHT,
        scroll_y: 0.0,
    }
}

fn raster_host_hit(
    node_id: &str,
    semantic_node_id: Option<&str>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> UiTreeNodeHit {
    UiTreeNodeHit {
        node_id: UiNodeId::new(node_id),
        semantic_node_id: semantic_node_id.map(UiNodeId::new),
        rect: UiTreeHitRect {
            x,
            y,
            width,
            height,
        },
        cursor: UiCursor::Default,
    }
}

fn non_background_width(canvas: &Canvas, start_y: usize, end_y: usize) -> usize {
    let mut min_x = canvas.width();
    let mut max_x = 0;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        let x = index % canvas.width();
        let y = index / canvas.width();
        if *pixel != 0 && y >= start_y && y < end_y {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
        }
    }
    if min_x == canvas.width() {
        return 0;
    }
    max_x.saturating_sub(min_x).saturating_add(1)
}
