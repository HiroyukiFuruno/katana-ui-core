use super::{
    Canvas, UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost,
    UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::{Spacer, Text};
use katana_ui_core::layout::{Alignment, Row};
use katana_ui_core::render_model::{
    UiDimension, UiHostActionSpec, UiNode, UiNodeId, UiNodeKind, UiScrollAreaProps, UiTextSpan,
};
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
const SCROLL_OFFSET: u32 = 2;
const SCROLL_CONTENT_HEIGHT: usize = ROW_HEIGHT + SCROLL_OFFSET as usize;

#[test]
fn centered_row_scroll_offset_repositions_child_action_when_content_exceeds_viewport() {
    let host = host();
    let unscrolled_actions = host.host_action_hits(&scroll_root(0), area());
    let scrolled_actions = host.host_action_hits(&scroll_root(SCROLL_OFFSET), area());
    let unscrolled_child = action_hit(&unscrolled_actions, "child");
    let scrolled_child = action_hit(&scrolled_actions, "child");

    assert_eq!(3, unscrolled_child.rect.y);
    assert_eq!(1, scrolled_child.rect.y);
    assert_eq!(
        SCROLL_OFFSET as usize,
        unscrolled_child
            .rect
            .y
            .saturating_sub(scrolled_child.rect.y),
        "the Spacer makes content taller than Row28, so offset_y must scroll"
    );
}

#[test]
fn centered_row_clips_root_link_lines_and_retains_glyph_backed_hits() {
    let host = host();
    let root = centered_row_children(vec![
        action_text("prefix", "P"),
        linked_text("link\nlower\nthird"),
    ]);
    let empty_root = centered_row_children(vec![empty_text(), empty_link_text()]);
    let area = UiTreeRenderArea {
        scroll_y: SCROLL_OFFSET as f32,
        ..area()
    };
    let mut canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
    let mut empty_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);

    host.render(&mut canvas, &root, area);
    host.render(&mut empty_canvas, &empty_root, area);
    let actions = host.host_action_hits(&root, area);
    let links = actions
        .iter()
        .filter(|hit| hit.action.action_id == "ui.link.open")
        .collect::<Vec<_>>();

    assert_eq!(
        2,
        links.len(),
        "the third line is outside Row28 and is removed"
    );
    assert!(
        actions.iter().any(|hit| hit.action.action_id == "prefix"),
        "clipping the link child must preserve the preceding child action"
    );
    assert!(
        links
            .iter()
            .all(|hit| hit.rect.y + hit.rect.height <= ROW_HEIGHT)
    );
    assert!(
        links
            .iter()
            .any(|hit| rect_has_changed_pixel(&canvas, &empty_canvas, hit.rect))
    );
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

fn centered_row_children(children: Vec<UiNode>) -> UiNode {
    let row: UiNode = children
        .into_iter()
        .fold(Row::new().align(Alignment::Center), |row, child| {
            row.child(child)
        })
        .into();
    row.height(UiDimension::px(ROW_HEIGHT as u16))
        .stable_node_id(UiNodeId::new("row"))
        .host_action(UiHostActionSpec::command("row", "row"))
}

fn action_text(id: &str, value: &str) -> UiNode {
    let node: UiNode = Text::new(value)
        .text_role("body")
        .host_action(UiHostActionSpec::command(id, id))
        .into();
    node.stable_node_id(UiNodeId::new(id))
}

fn linked_text(value: &str) -> UiNode {
    Text::new(value)
        .text_role("body")
        .text_spans(vec![UiTextSpan {
            text: value.to_owned(),
            style: Default::default(),
            link_target: "https://example.test/link".to_owned(),
        }])
        .into()
}

fn empty_text() -> UiNode {
    Text::new("").text_role("body").into()
}

fn empty_link_text() -> UiNode {
    Text::new("\n\n").text_role("body").into()
}

fn scroll_root(offset_y: u32) -> UiNode {
    UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            offset_y,
            viewport_width: AREA_WIDTH as u32,
            viewport_height: ROW_HEIGHT as u32,
            content_width: AREA_WIDTH as u32,
            content_height: SCROLL_CONTENT_HEIGHT as u32,
            ..UiScrollAreaProps::default()
        })
        .child(centered_row_children(vec![action_text("child", "WWWW")]))
        .child(UiNode::from(Spacer::new("")).height(UiDimension::px(SCROLL_OFFSET as u16)))
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

fn action_hit<'a>(
    hits: &'a [super::UiTreeHostActionHit],
    action_id: &str,
) -> &'a super::UiTreeHostActionHit {
    hits.iter()
        .find(|hit| hit.action.action_id == action_id)
        .kuc_expect("row alignment action hit")
}

fn rect_has_changed_pixel(
    canvas: &Canvas,
    empty_canvas: &Canvas,
    rect: super::UiTreeHitRect,
) -> bool {
    (rect.y..rect.y.saturating_add(rect.height)).any(|y| {
        (rect.x..rect.x.saturating_add(rect.width)).any(|x| {
            x < canvas.width()
                && y < canvas.height()
                && canvas.pixels()[y * canvas.width() + x]
                    != empty_canvas.pixels()[y * empty_canvas.width() + x]
        })
    })
}
