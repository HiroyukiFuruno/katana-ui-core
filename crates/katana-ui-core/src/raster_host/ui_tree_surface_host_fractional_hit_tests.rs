use super::{
    UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost, UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use crate::theme::ThemeSnapshot;
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{
    UiCommonProps, UiDimension, UiEdgeInsets, UiHostActionSpec, UiNode, UiNodeKind,
    UiScrollAreaProps,
};

const TEST_AREA_WIDTH: usize = 240;
const TEST_AREA_HEIGHT: usize = 80;

#[test]
fn surface_host_action_hits_keep_fractional_cursor_through_container_spacing() {
    let document_typography = UiTreeDocumentTypography::new()
        .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5));
    let first: UiNode = Text::new("First").text_role("body").into();
    let trailing: UiNode = Text::new("Trailing")
        .text_role("body")
        .host_action(UiHostActionSpec::command("trailing", "Trailing"))
        .into();
    let root = UiNode::new(UiNodeKind::Column, "")
        .common(UiCommonProps::default().padding(UiEdgeInsets {
            top: UiDimension::px(8),
            ..UiEdgeInsets::default()
        }))
        .child(first)
        .child(trailing);
    let host =
        UiTreeSurfaceHost::with_document_typography(ThemeSnapshot::dark(), document_typography);

    let hits = host.document_host_action_hits(&root, test_area());
    let trailing = hits
        .iter()
        .find(|hit| hit.action.action_id == "trailing")
        .kuc_expect("trailing action hit");

    assert_eq!(39, trailing.rect.y);
}

#[test]
fn surface_host_action_hits_keep_fractional_cursor_in_scroll_viewport() {
    let document_typography = UiTreeDocumentTypography::new()
        .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5));
    let first: UiNode = Text::new("First").text_role("body").into();
    let trailing: UiNode = Text::new("Trailing")
        .text_role("body")
        .host_action(UiHostActionSpec::command("trailing", "Trailing"))
        .into();
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: TEST_AREA_WIDTH as u32,
            viewport_height: TEST_AREA_HEIGHT as u32,
            content_height: TEST_AREA_HEIGHT as u32,
            ..UiScrollAreaProps::default()
        })
        .child(first)
        .child(trailing);
    let host =
        UiTreeSurfaceHost::with_document_typography(ThemeSnapshot::dark(), document_typography);

    let hits = host.document_host_action_hits(&root, test_area());
    let trailing = hits
        .iter()
        .find(|hit| hit.action.action_id == "trailing")
        .kuc_expect("trailing action hit");

    assert_eq!(31, trailing.rect.y);
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
