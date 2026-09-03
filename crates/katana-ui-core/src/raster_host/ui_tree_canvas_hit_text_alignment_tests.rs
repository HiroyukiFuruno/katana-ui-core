use super::*;
use crate::raster_host::ui_tree_canvas_text_role::UiTreeTextRoleRenderer;

#[test]
fn right_aligned_link_text_action_rect_trims_whitespace_without_losing_ink() {
    let root = UiNode::from(
        Text::new("Uppercase right link")
            .text_role("html-right")
            .text_spans(vec![UiTextSpan {
                text: " Uppercase right link ".to_string(),
                style: Default::default(),
                link_target: "https://example.test/docs".to_string(),
            }]),
    );
    let full_width = text_hit_width(&root, " Uppercase right link ");
    let leading_space_width = text_hit_width(&root, " ");
    let area = UiTreeRenderArea {
        x: 7,
        y: 11,
        width: 420,
        height: 120,
        scroll_y: 0.0,
    };
    let aligned_x = UiTreeTextRoleRenderer::aligned_x(&root, area.x, area, full_width);
    let expected_x = (aligned_x + leading_space_width as isize).max(0) as usize;

    let hits = UiTreeHostActionHitCollector::collect(&root, area);

    assert_eq!(1, hits.len());
    assert_eq!(expected_x, hits[0].rect.x);
    assert!(hits[0].rect.width < full_width);
    assert!(hits[0].rect.width > 120);
}

#[test]
fn underlined_link_text_action_rect_keeps_text_width() {
    let root = UiNode::from(
        Text::new("Uppercase right link")
            .text_role("html-right")
            .width(UiDimension::Px(872))
            .text_spans(vec![UiTextSpan {
                text: "Uppercase right link".to_string(),
                style: katana_ui_core::render_model::UiTextSpanStyle {
                    underline: true,
                    ..Default::default()
                },
                link_target: "https://example.test/docs".to_string(),
            }]),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 222,
            width: 748,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert!(
        hits[0].rect.width > 120,
        "underlined link rect must use text width, got {:?}",
        hits[0].rect
    );
}
