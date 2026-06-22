use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn quoted_list_item_draws_material_bullet_and_offsets_text() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 80, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "List item 1")
        .text(UiTextProps {
            role: "blockquote".to_string(),
            ..UiTextProps::default()
        })
        .common(quote_bullet_common(1))
        .height(UiDimension::Px(46));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 56, 8));
    assert!(count_pixel(&canvas, palette.text) > 0);
    assert_eq!(Some(palette.background), pixel_at(&canvas, 84, 17));
}

#[test]
fn list_marker_bullet_uses_text_color_material_dot() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(120, 60, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "  ")
        .text(UiTextProps {
            role: "list-marker".to_string(),
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(46));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 64,
            height: 60,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.text), pixel_at(&canvas, 70, 17));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 56, 6));
}

#[test]
fn list_marker_center_matches_list_item_text_ink_center() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 60, palette.background);
    let marker = UiNode::from(Text::new("  ").text_role("list-marker"))
        .width(UiDimension::Px(44))
        .height(UiDimension::Px(46));
    let body = UiNode::from(Text::new("List item").text_role("list-item"))
        .width(UiDimension::Px(160))
        .height(UiDimension::Px(46));
    let root = UiNode::from(Row::new().child(marker).child(body));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 60,
            scroll_y: 0.0,
        },
    );

    let marker_bounds = vertical_bounds_for_color_in_x_range(&canvas, palette.text, 64, 82)
        .kuc_expect("list marker should draw text-color pixels");
    let text_bounds = vertical_bounds_for_color_in_x_range(&canvas, palette.text, 100, 220)
        .kuc_expect("list item body should draw text-color pixels");

    assert!(
        marker_bounds
            .center_twice()
            .abs_diff(text_bounds.center_twice())
            <= 4,
        "list marker center must align with body text ink: marker={marker_bounds:?}, text={text_bounds:?}"
    );
}

#[test]
fn nested_list_markers_switch_shape_by_depth() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(180, 120, palette.background);
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(
            UiNode::new(UiNodeKind::Text, "  ")
                .text(UiTextProps {
                    role: "list-marker".to_string(),
                    ..UiTextProps::default()
                })
                .common(list_depth_common(1))
                .height(UiDimension::Px(46)),
        )
        .child(
            UiNode::new(UiNodeKind::Text, "  ")
                .text(UiTextProps {
                    role: "list-marker".to_string(),
                    ..UiTextProps::default()
                })
                .common(list_depth_common(2))
                .height(UiDimension::Px(46)),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 124,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 70, 17));
    assert_eq!(Some(palette.text), pixel_at(&canvas, 70, 13));
    assert_eq!(Some(palette.text), pixel_at(&canvas, 67, 60));
}

#[test]
fn ordered_list_marker_renders_text_without_selection_square() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(140, 60, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "1.")
        .text(UiTextProps {
            role: "list-marker".to_string(),
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(46));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 84,
            height: 60,
            scroll_y: 0.0,
        },
    );

    assert_eq!(0, count_pixel(&canvas, palette.selection));
    assert!(count_pixel(&canvas, palette.text) > 0);
}

#[test]
fn layout_container_does_not_shift_markdown_body_text() {
    let direct = render_canvas(UiNode::new(UiNodeKind::Text, "Body"));
    let column = render_canvas(
        UiNode::new(UiNodeKind::Column, "").child(UiNode::new(UiNodeKind::Text, "Body")),
    );

    assert_eq!(
        first_content_x(&direct),
        first_content_x(&column),
        "layout-only nodes must not add visual indentation"
    );
}

#[test]
fn layout_container_respects_explicit_padding_for_document_surface() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(160, 80, palette.background);
    let root = UiNode::new(UiNodeKind::Column, "")
        .common(
            katana_ui_core::render_model::UiCommonProps::default().padding(UiEdgeInsets {
                top: UiDimension::Px(12),
                right: UiDimension::Px(0),
                bottom: UiDimension::Px(0),
                left: UiDimension::Px(24),
            }),
        )
        .child(UiNode::new(UiNodeKind::Text, "Body"));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 80,
            scroll_y: 0.0,
        },
    );

    let first_x = first_content_x(&canvas).kuc_expect("padded child text should render");
    assert!(
        (24..=32).contains(&first_x),
        "padding left must not be ignored: {first_x}"
    );
    let first_row = first_row_for_non_background(&canvas, palette.background)
        .kuc_expect("padded child text should render");
    assert!(
        first_row >= 12,
        "padding top must not be ignored: {first_row}"
    );
}

#[test]
fn layout_container_padding_limits_child_background_width() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(160, 64, palette.background);
    let root = UiNode::new(UiNodeKind::Column, "")
        .common(
            katana_ui_core::render_model::UiCommonProps::default().padding(UiEdgeInsets {
                top: UiDimension::Px(0),
                right: UiDimension::Px(24),
                bottom: UiDimension::Px(0),
                left: UiDimension::Px(24),
            }),
        )
        .child(UiNode::new(UiNodeKind::Text, "code").text(UiTextProps {
            role: "code".to_string(),
            ..UiTextProps::default()
        }));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 64,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.code_background), pixel_at(&canvas, 24, 0));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 159, 0));
}
