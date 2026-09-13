use super::{
    Canvas, UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost,
    UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::Text;
use katana_ui_core::layout::{Alignment, Row};
use katana_ui_core::render_model::{UiDimension, UiHostActionSpec, UiNode, UiNodeId, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

const AREA_WIDTH: usize = 160;
const AREA_HEIGHT: usize = 80;
const ROW_HEIGHT: usize = 28;
const BODY_FONT_SIZE: f32 = 14.0;
const BODY_LINE_BOX_HEIGHT: f32 = 21.0;
const BODY_BASELINE: f32 = 15.0;
const HEADING_FONT_SIZE: f32 = 20.0;
const HEADING_LINE_BOX_HEIGHT: f32 = 31.5;
const HEADING_BASELINE: f32 = 20.0;

#[test]
fn centered_fixed_row_uses_fractional_child_offset_for_draw_and_hits() {
    let host = host();
    let root = centered_row(action_text("body", "child", "WWWW"));
    let empty_root = centered_row(empty_text("body"));
    let start_root = start_row(action_text("body", "start-child", "WWWW"));
    let empty_start_root = start_row(empty_text("body"));
    let area = area();
    let mut canvas = Canvas::new_scaled(AREA_WIDTH, AREA_HEIGHT, 2.0, 0);
    let mut empty_canvas = Canvas::new_scaled(AREA_WIDTH, AREA_HEIGHT, 2.0, 0);
    let mut start_canvas = Canvas::new_scaled(AREA_WIDTH, AREA_HEIGHT, 2.0, 0);
    let mut empty_start_canvas = Canvas::new_scaled(AREA_WIDTH, AREA_HEIGHT, 2.0, 0);

    host.render(&mut canvas, &root, area);
    host.render(&mut empty_canvas, &empty_root, area);
    host.render(&mut start_canvas, &start_root, area);
    host.render(&mut empty_start_canvas, &empty_start_root, area);
    let nodes = host.document_node_hits(&root, area);
    let actions = host.document_host_action_hits(&root, area);
    let row = node_hit(&nodes, "row");
    let child = node_hit(&nodes, "child");
    let row_action = action_hit(&actions, "row");
    let child_action = action_hit(&actions, "child");

    assert_eq!(0, row.rect.y);
    assert_eq!(ROW_HEIGHT, row.rect.height);
    assert_eq!(row.rect, row_action.rect);
    assert_eq!(3, child.rect.y);
    assert_eq!(22, child.rect.height);
    assert_eq!(child.rect, child_action.rect);
    assert_eq!(
        7,
        glyph_bounds(&canvas, &empty_canvas)
            .0
            .saturating_sub(glyph_bounds(&start_canvas, &empty_start_canvas).0),
        "2x canvas must retain the 3.5px logical center offset"
    );
}

#[test]
fn centered_row_clamps_large_child_and_keeps_short_child_centered() {
    let host = host();
    let short = centered_row(action_text("body", "short", "short").height(UiDimension::px(10)));
    let large = centered_row(action_text("body", "large", "large\nchild"));

    let short_hits = host.document_node_hits(&short, area());
    let large_hits = host.document_node_hits(&large, area());
    let large_actions = host.document_host_action_hits(&large, area());
    let short_hit = node_hit(&short_hits, "short");
    let large_hit = node_hit(&large_hits, "large");

    assert_eq!(9, short_hit.rect.y);
    assert_eq!(10, short_hit.rect.height);
    assert_eq!(0, large_hit.rect.y);
    assert_eq!(ROW_HEIGHT, large_hit.rect.height);
    assert_eq!(large_hit.rect, action_hit(&large_actions, "large").rect);
}

#[test]
fn fixed_start_row_keeps_existing_child_origin() {
    let host = host();
    let child = action_text("body", "child", "WWWW");
    let row: UiNode = Row::new().child(child).into();
    let row = row.height(UiDimension::px(ROW_HEIGHT as u16));

    let hits = host.document_node_hits(&row, area());
    let child_hit = node_hit(&hits, "child");

    assert_eq!(0, child_hit.rect.y);
    assert_eq!(BODY_LINE_BOX_HEIGHT as usize, child_hit.rect.height);
}

#[test]
fn auto_height_center_row_keeps_start_and_centers_non_text_child() {
    let host = host();
    let auto_row: UiNode = Row::new()
        .align(Alignment::Center)
        .child(action_text("body", "auto", "WWWW"))
        .into();
    let button = UiNode::new(UiNodeKind::Button, "Button")
        .stable_node_id(UiNodeId::new("button"))
        .host_action(UiHostActionSpec::command("button", "button"));
    let fixed_row: UiNode = Row::new().align(Alignment::Center).child(button).into();
    let fixed_row = fixed_row.height(UiDimension::px(ROW_HEIGHT as u16));

    let auto_hits = host.document_node_hits(&auto_row, area());
    let fixed_hits = host.document_node_hits(&fixed_row, area());

    assert_eq!(0, node_hit(&auto_hits, "auto").rect.y);
    assert_eq!(4, node_hit(&fixed_hits, "button").rect.y);
    let fixed_actions = host.document_host_action_hits(&fixed_row, area());
    assert_eq!(
        node_hit(&fixed_hits, "button").rect,
        action_hit(&fixed_actions, "button").rect,
    );
    let mut canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
    let empty_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
    host.render(&mut canvas, &fixed_row, area());
    let (paint_top, paint_bottom) = glyph_bounds(&canvas, &empty_canvas);
    let hit = node_hit(&fixed_hits, "button").rect;
    assert_eq!(hit.y, paint_top);
    assert_eq!(hit.y + hit.height - 1, paint_bottom);
}

#[test]
fn legacy_center_row_keeps_existing_start_origin_and_row_hit_height() {
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let centered = centered_row(action_text("body", "child", "WWWW"));
    let start = start_row(action_text("body", "start-child", "WWWW"))
        .stable_node_id(UiNodeId::new("start-row"))
        .host_action(UiHostActionSpec::command("start-row", "start-row"));
    let centered_nodes = host.document_node_hits(&centered, area());
    let centered_actions = host.document_host_action_hits(&centered, area());
    let start_nodes = host.document_node_hits(&start, area());
    let start_actions = host.document_host_action_hits(&start, area());

    assert_eq!(0, node_hit(&centered_nodes, "child").rect.y);
    assert_eq!(
        node_hit(&start_nodes, "start-child").rect.y,
        node_hit(&centered_nodes, "child").rect.y,
        "legacy Center must retain Start child placement"
    );
    assert_eq!(
        node_hit(&start_nodes, "start-row").rect.height,
        node_hit(&centered_nodes, "row").rect.height,
        "legacy Center must retain Start row hit height"
    );
    assert_eq!(
        action_hit(&start_actions, "start-row").rect.height,
        action_hit(&centered_actions, "row").rect.height,
    );
}

#[test]
fn centered_row_encloses_fractional_start_in_its_node_and_action_hits() {
    let host = host();
    let leading: UiNode = Text::new("Heading").text_role("heading").into();
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(leading)
        .child(centered_row(action_text("body", "child", "WWWW")));
    let nodes = host.document_node_hits(&root, area());
    let actions = host.document_host_action_hits(&root, area());
    let row = node_hit(&nodes, "row");
    let child = node_hit(&nodes, "child");

    assert_eq!(31, row.rect.y);
    assert_eq!(29, row.rect.height);
    assert_eq!(row.rect, action_hit(&actions, "row").rect);
    assert_eq!(35, child.rect.y);
    assert_eq!(21, child.rect.height);
}

fn host() -> UiTreeSurfaceHost {
    UiTreeSurfaceHost::with_document_typography(
        ThemeSnapshot::dark(),
        UiTreeDocumentTypography::new()
            .with_body_baseline(UiTreeTextRoleBaselineTypography::new(
                BODY_FONT_SIZE,
                BODY_LINE_BOX_HEIGHT,
                BODY_BASELINE,
            ))
            .with_heading_1_baseline(UiTreeTextRoleBaselineTypography::new(
                HEADING_FONT_SIZE,
                HEADING_LINE_BOX_HEIGHT,
                HEADING_BASELINE,
            )),
    )
}

fn centered_row(child: UiNode) -> UiNode {
    let row: UiNode = Row::new().align(Alignment::Center).child(child).into();
    row.height(UiDimension::px(ROW_HEIGHT as u16))
        .stable_node_id(UiNodeId::new("row"))
        .host_action(UiHostActionSpec::command("row", "row"))
}

fn start_row(child: UiNode) -> UiNode {
    let row: UiNode = Row::new().child(child).into();
    row.height(UiDimension::px(ROW_HEIGHT as u16))
}

fn action_text(role: &str, id: &str, value: &str) -> UiNode {
    let node: UiNode = Text::new(value)
        .text_role(role)
        .host_action(UiHostActionSpec::command(id, id))
        .into();
    node.stable_node_id(UiNodeId::new(id))
}

fn empty_text(role: &str) -> UiNode {
    Text::new("").text_role(role).into()
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

fn node_hit<'a>(hits: &'a [super::UiTreeNodeHit], id: &str) -> &'a super::UiTreeNodeHit {
    hits.iter()
        .find(|hit| hit.node_id.as_str() == id)
        .kuc_expect("row alignment node hit")
}

fn action_hit<'a>(
    hits: &'a [super::UiTreeHostActionHit],
    action_id: &str,
) -> &'a super::UiTreeHostActionHit {
    hits.iter()
        .find(|hit| hit.action.action_id == action_id)
        .kuc_expect("row alignment action hit")
}

fn glyph_bounds(canvas: &Canvas, empty_canvas: &Canvas) -> (usize, usize) {
    let mut top = canvas.height();
    let mut bottom = 0;
    for (index, (pixel, empty_pixel)) in canvas
        .pixels()
        .iter()
        .zip(empty_canvas.pixels())
        .enumerate()
    {
        if pixel != empty_pixel {
            let y = index / canvas.width();
            top = top.min(y);
            bottom = bottom.max(y);
        }
    }
    (top, bottom)
}
