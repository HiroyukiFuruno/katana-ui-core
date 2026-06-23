use super::*;

#[path = "ui_tree_canvas_hit_text_alignment_tests.rs"]
mod alignment_tests;

#[test]
fn collects_button_action_rect_from_render_layout() {
    let root = UiNode::from(
        Row::new().child(
            UiNode::from(Button::new("Copy"))
                .width(UiDimension::px(96))
                .height(UiDimension::px(20))
                .host_action(UiHostActionSpec::command("copy", "Copy")),
        ),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 4,
            y: 8,
            width: 320,
            height: 200,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("copy", hits[0].action.action_id);
    assert_eq!(UiCursor::Pointer, hits[0].cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 4,
            y: 8,
            width: 96,
            height: 20,
        },
        hits[0].rect
    );
}

#[test]
fn collects_link_text_action_rect_with_kuc_cursor() {
    let root = UiNode::from(Text::new("docs").text_spans(vec![UiTextSpan {
        text: "docs".to_string(),
        style: Default::default(),
        link_target: "https://example.test".to_string(),
    }]));
    let expected_width = text_hit_width(&root, "docs");

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 180,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("ui.link.open", hits[0].action.action_id);
    assert_eq!(UiCursor::Pointer, hits[0].cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 7,
            y: 11,
            width: expected_width,
            height: 20,
        },
        hits[0].rect
    );
}

#[test]
fn collects_link_text_action_rect_for_only_link_span() {
    let root = UiNode::from(Text::new("before docs after").text_spans(vec![
        UiTextSpan::plain("before "),
        UiTextSpan {
            text: "docs".to_string(),
            style: Default::default(),
            link_target: "https://example.test/docs".to_string(),
        },
        UiTextSpan::plain(" after"),
    ]));
    let expected_x = 7 + text_hit_width(&root, "before ");
    let expected_width = text_hit_width(&root, "docs");

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 240,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("ui.link.open", hits[0].action.action_id);
    assert_eq!("https://example.test/docs", hits[0].action.payload);
    assert_eq!(UiCursor::Pointer, hits[0].cursor);
    assert_eq!(
        UiTreeHitRect {
            x: expected_x,
            y: 11,
            width: expected_width,
            height: 20,
        },
        hits[0].rect
    );
}

#[test]
fn link_text_action_rect_uses_document_text_metrics() {
    let root = UiNode::from(
        Text::new("docs")
            .text_role("body")
            .text_spans(vec![UiTextSpan {
                text: "docs".to_string(),
                style: Default::default(),
                link_target: "https://example.test".to_string(),
            }]),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 180,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert!(hits[0].rect.width > 29);
    assert_eq!(34, hits[0].rect.height);
}

#[test]
fn centered_link_text_action_rect_follows_rendered_text_alignment() {
    let root = UiNode::from(
        Text::new("docs")
            .text_role("html-centered")
            .text_spans(vec![UiTextSpan {
                text: "docs".to_string(),
                style: Default::default(),
                link_target: "https://example.test".to_string(),
            }]),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 240,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert!(
        hits[0].rect.x > 80,
        "centered link hit rect must move with rendered text: {:?}",
        hits[0].rect
    );
}

#[test]
fn right_aligned_link_text_action_rect_follows_rendered_text_alignment() {
    let root =
        UiNode::from(
            Text::new("docs")
                .text_role("html-right")
                .text_spans(vec![UiTextSpan {
                    text: "docs".to_string(),
                    style: Default::default(),
                    link_target: "https://example.test".to_string(),
                }]),
        );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 240,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert!(
        hits[0].rect.x > 170,
        "right aligned link hit rect must move with rendered text: {:?}",
        hits[0].rect
    );
}

#[test]
fn link_text_action_rect_trims_inline_whitespace_like_underline_bounds() {
    let root = UiNode::from(Text::new("English | 日本語").text_spans(vec![
        UiTextSpan::plain("English |"),
        UiTextSpan {
            text: " 日本語 ".to_string(),
            style: Default::default(),
            link_target: "https://example.test/lang-ja".to_string(),
        },
    ]));
    let link_span_width = text_hit_width(&root, " 日本語 ");
    let leading_space_width = text_hit_width(&root, " ");
    let expected_x = 7 + text_hit_width(&root, "English |") + leading_space_width;
    let expected_width = link_span_width - leading_space_width - leading_space_width;

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 320,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!(
        UiTreeHitRect {
            x: expected_x,
            y: 11,
            width: expected_width,
            height: 20,
        },
        hits[0].rect
    );
}
