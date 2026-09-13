use super::{
    Canvas, UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost,
    UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiHostActionSpec, UiNode, UiNodeId, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

const TEST_AREA_WIDTH: usize = 240;
const TEST_AREA_HEIGHT: usize = 96;

#[test]
fn surface_host_higher_heading_typography_shares_fractional_renderer_and_hits() {
    let typography = UiTreeDocumentTypography::new()
        .with_heading_4_baseline(UiTreeTextRoleBaselineTypography::new(17.507, 26.5, 15.5))
        .with_heading_5_baseline(UiTreeTextRoleBaselineTypography::new(16.338, 24.5, 14.5))
        .with_heading_6_baseline(UiTreeTextRoleBaselineTypography::new(15.169, 23.0, 13.5));
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(higher_heading("heading-4", "h4"))
        .child(higher_heading("heading-5", "h5"))
        .child(higher_heading("heading-6", "h6"));
    let empty_root = UiNode::new(UiNodeKind::Column, "")
        .child(empty_higher_heading("heading-4"))
        .child(empty_higher_heading("heading-5"))
        .child(empty_higher_heading("heading-6"));
    let host = UiTreeSurfaceHost::with_document_typography(ThemeSnapshot::dark(), typography);
    let area = test_area();
    let mut canvas = Canvas::new(TEST_AREA_WIDTH, TEST_AREA_HEIGHT, 0);
    let mut empty_canvas = Canvas::new(TEST_AREA_WIDTH, TEST_AREA_HEIGHT, 0);

    host.render(&mut canvas, &root, area);
    host.render(&mut empty_canvas, &empty_root, area);
    let nodes = host.document_node_hits(&root, area);
    let actions = host.document_host_action_hits(&root, area);

    for (node_id, action_id, y, height) in [
        ("h4", "h4", 0, 27),
        ("h5", "h5", 26, 26),
        ("h6", "h6", 51, 23),
    ] {
        let node = nodes
            .iter()
            .find(|hit| hit.node_id.as_str() == node_id)
            .kuc_expect("higher-heading node hit");
        let action = actions
            .iter()
            .find(|hit| hit.action.action_id == action_id)
            .kuc_expect("higher-heading action hit");

        assert_eq!(y, node.rect.y, "{node_id} node y");
        assert_eq!(height, node.rect.height, "{node_id} node height");
        assert_eq!(
            node.rect, action.rect,
            "{node_id} renderer and action bounds"
        );
        assert!(
            has_text_pixel(&canvas, &empty_canvas, y, y + height),
            "{node_id} glyph draw"
        );
    }
}

fn higher_heading(role: &str, id: &str) -> UiNode {
    let node: UiNode = Text::new("WWWW")
        .text_role(role)
        .host_action(UiHostActionSpec::command(id, id))
        .into();
    node.stable_node_id(UiNodeId::new(id))
}

fn empty_higher_heading(role: &str) -> UiNode {
    Text::new("").text_role(role).into()
}

fn test_area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: TEST_AREA_WIDTH,
        height: TEST_AREA_HEIGHT,
        scroll_y: 0.0,
    }
}

fn has_text_pixel(canvas: &Canvas, empty_canvas: &Canvas, start_y: usize, end_y: usize) -> bool {
    canvas
        .pixels()
        .iter()
        .zip(empty_canvas.pixels())
        .enumerate()
        .any(|(index, (pixel, empty_pixel))| {
            pixel != empty_pixel && (start_y..end_y).contains(&(index / canvas.width()))
        })
}
