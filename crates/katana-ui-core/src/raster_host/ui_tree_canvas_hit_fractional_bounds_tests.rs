use super::*;
use crate::raster_host::{
    UiTreeDocumentTypography, UiTreeNodeHit, UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::{Spacer, Text};
use katana_ui_core::layout::Stack;
use katana_ui_core::render_model::{
    UiCommonProps, UiDimension, UiEdgeInsets, UiHostActionSpec, UiInteractionState, UiNode,
    UiNodeId, UiNodeKind, UiPosition, UiScrollAreaProps, UiTextSpan, UiTextWrapMode,
};

const AREA_WIDTH: usize = 240;
const AREA_HEIGHT: usize = 180;
const BODY_FONT_SIZE: f32 = 16.0;
const BODY_LINE_BOX: f32 = 21.0;
const BODY_BASELINE: f32 = 12.5;
const HEADING_FONT_SIZE: f32 = 24.0;
const HEADING_LINE_BOX: f32 = 31.5;
const HEADING_BASELINE: f32 = 18.5;
const ROOT_PADDING_TOP: u16 = 24;
const SPACER_HEIGHT: u16 = 20;
const INTEGER_TOP: u16 = 31;
const ONE_PIXEL_HEIGHT: u16 = 1;
const FRACTIONAL_TOP: usize = 31;
const FRACTIONAL_HEIGHT: usize = 33;
const TWO_LINE_TOP: usize = 75;
const TWO_LINE_HEIGHT: usize = 43;
const INTEGER_HEIGHT: usize = 42;
const RETINA_SCALE: usize = 2;
const LEGACY_INTEGER_TEXT_HEIGHT: usize = 20;
const LEGACY_FRACTIONAL_TEXT_HEIGHT: usize = 21;
const LEGACY_LAST_HOVER_ROW: usize = 51;

#[test]
fn fractional_heading_bounds_include_the_last_scaled_paint_row_for_node_and_action() {
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(heading("leading"))
        .child(action_heading("target"));
    let (node, action) = target_bounds(&root, "target");

    assert_eq!(FRACTIONAL_TOP, node.rect.y);
    assert_eq!(FRACTIONAL_HEIGHT, node.rect.height);
    assert_eq!(node.rect, action.rect);
    assert!(node.rect.y * RETINA_SCALE <= 63);
    assert!((node.rect.y + node.rect.height) * RETINA_SCALE > 63);
}

#[test]
fn fractional_two_line_body_bounds_include_hover_surface_end_without_changing_advance() {
    let root = UiNode::new(UiNodeKind::Column, "")
        .common(UiCommonProps::default().padding(UiEdgeInsets {
            top: UiDimension::px(ROOT_PADDING_TOP),
            ..UiEdgeInsets::default()
        }))
        .child(heading("leading"))
        .child(UiNode::from(Spacer::new("")).height(UiDimension::px(SPACER_HEIGHT)))
        .child(action_body("target"));
    let (node, action) = target_bounds(&root, "target");

    assert_eq!(TWO_LINE_TOP, node.rect.y);
    assert_eq!(TWO_LINE_HEIGHT, node.rect.height);
    assert_eq!(node.rect, action.rect);
}

#[test]
fn explicit_document_text_height_expands_only_its_fractional_hit_boundary() {
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(heading("leading"))
        .child(action_heading("target").height(UiDimension::px(ONE_PIXEL_HEIGHT)))
        .child(heading("following"));
    let (node, action) = target_bounds(&root, "target");

    assert_eq!(FRACTIONAL_TOP, node.rect.y);
    assert_eq!(2, node.rect.height);
    assert_eq!(node.rect, action.rect);
    assert_eq!(
        FRACTIONAL_TOP + ONE_PIXEL_HEIGHT as usize,
        node_hit(&root, "following").rect.y,
        "the requested logical height must keep the following origin unchanged"
    );
}

#[test]
fn integer_two_line_body_keeps_its_existing_hit_height() {
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(UiNode::from(Spacer::new("")).height(UiDimension::px(INTEGER_TOP)))
        .child(action_body("target"));
    let (node, action) = target_bounds(&root, "target");

    assert_eq!(INTEGER_TOP as usize, node.rect.y);
    assert_eq!(INTEGER_HEIGHT, node.rect.height);
    assert_eq!(node.rect, action.rect);
}

#[test]
fn legacy_text_bounds_include_the_last_fractional_hover_background_row() {
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(heading("leading"))
        .child(
            UiNode::from(Text::new("legacy").text_role("legacy"))
                .stable_node_id(UiNodeId::new("target"))
                .interaction(UiInteractionState {
                    hovered: true,
                    ..UiInteractionState::default()
                })
                .host_action(UiHostActionSpec::command("target", "target")),
        );
    let (node, action) = target_bounds(&root, "target");
    let mut canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
    renderer().render(&mut canvas, &root, area());

    assert_eq!(FRACTIONAL_TOP, node.rect.y);
    assert_eq!(LEGACY_FRACTIONAL_TEXT_HEIGHT, node.rect.height);
    assert_eq!(node.rect, action.rect);
    assert_ne!(
        0,
        canvas.pixels()[LEGACY_LAST_HOVER_ROW * canvas.width() + AREA_WIDTH - 1],
        "legacy hover background must remain inside its action hit"
    );
    assert_eq!(
        LEGACY_INTEGER_TEXT_HEIGHT,
        renderer().document_node_hit_rects(
            &UiNode::from(Text::new("legacy").text_role("legacy"))
                .stable_node_id(UiNodeId::new("integer")),
            area(),
        )[0]
        .rect
        .height,
    );
}

#[test]
fn absolute_document_text_keeps_the_fractional_container_origin_in_its_bounds() {
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: AREA_WIDTH as u32,
            viewport_height: AREA_HEIGHT as u32,
            content_height: AREA_HEIGHT as u32,
            offset_y: ONE_PIXEL_HEIGHT as u32,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(heading("leading"))
                .child(
                    UiNode::from(
                        Stack::new().child(action_heading("target").position(UiPosition::Absolute)),
                    )
                    .height(UiDimension::px(AREA_HEIGHT as u16)),
                ),
        );
    let (node, action) = target_bounds(&root, "target");

    assert_eq!(FRACTIONAL_TOP, node.rect.y);
    assert_eq!(FRACTIONAL_HEIGHT, node.rect.height);
    assert_eq!(node.rect, action.rect);
}

#[test]
fn fractional_document_link_line_keeps_its_existing_line_clip_height() {
    let root =
        UiNode::new(UiNodeKind::Column, "")
            .child(heading("leading"))
            .child(
                UiNode::from(Text::new("Heading").text_role("heading").text_spans(vec![
                    UiTextSpan {
                        text: "Heading".to_owned(),
                        style: Default::default(),
                        link_target: "https://example.test/heading".to_owned(),
                    },
                ]))
                .stable_node_id(UiNodeId::new("target")),
            );
    let node = node_hit(&root, "target");
    let action = renderer()
        .document_host_action_hit_rects(&root, area())
        .into_iter()
        .find(|hit| hit.action.action_id == "ui.link.open")
        .kuc_expect("link action hit");

    assert_eq!(FRACTIONAL_HEIGHT, node.rect.height);
    assert_eq!(HEADING_LINE_BOX.ceil() as usize, action.rect.height);
}

fn heading(id: &str) -> UiNode {
    UiNode::from(Text::new("Heading").text_role("heading")).stable_node_id(UiNodeId::new(id))
}

fn action_heading(id: &str) -> UiNode {
    UiNode::from(Text::new("Heading").text_role("heading"))
        .stable_node_id(UiNodeId::new(id))
        .host_action(UiHostActionSpec::command(id, id))
}

fn action_body(id: &str) -> UiNode {
    UiNode::from(
        Text::new("First document row.\nSecond document row.")
            .text_role("body")
            .wrap(UiTextWrapMode::Wrap),
    )
    .stable_node_id(UiNodeId::new(id))
    .host_action(UiHostActionSpec::command(id, id))
}

fn target_bounds(root: &UiNode, target: &str) -> (UiTreeNodeHit, UiTreeHostActionHit) {
    let renderer = renderer();
    let node = node_hit(root, target);
    let action = renderer
        .document_host_action_hit_rects(root, area())
        .into_iter()
        .find(|hit| hit.action.action_id == target)
        .kuc_expect("target action hit");
    (node, action)
}

fn renderer() -> UiTreeCanvasRenderer {
    UiTreeCanvasRenderer::with_document_typography(
        ThemeSnapshot::dark(),
        UiTreeDocumentTypography::new()
            .with_body_baseline(UiTreeTextRoleBaselineTypography::new(
                BODY_FONT_SIZE,
                BODY_LINE_BOX,
                BODY_BASELINE,
            ))
            .with_heading_1_baseline(UiTreeTextRoleBaselineTypography::new(
                HEADING_FONT_SIZE,
                HEADING_LINE_BOX,
                HEADING_BASELINE,
            )),
    )
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

fn node_hit(root: &UiNode, target: &str) -> UiTreeNodeHit {
    renderer()
        .document_node_hit_rects(root, area())
        .into_iter()
        .find(|hit| hit.node_id.as_str() == target)
        .kuc_expect("target node hit")
}
