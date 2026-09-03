use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_text_wrap_support::{
    diff_in_rect, pixel_at, range_has_content, rect_has_content, render_area,
};
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::atom::Text;
use katana_ui_core::layout::Column;
use katana_ui_core::render_model::{
    UiCommonProps, UiDimension, UiEdgeInsets, UiNode, UiNodeKind, UiScrollAreaProps, UiTextSpan,
    UiTextWrapMode, UiTree, UiVisualRole,
};
use katana_ui_core::theme::ThemeSnapshot;

const WRAP_VIEWPORT_WIDTH: usize = 180;
const WRAP_VIEWPORT_HEIGHT: usize = 80;
const WRAP_SCROLL_OFFSET_Y: u32 = 42;
const WRAP_CONTENT_HEIGHT: u32 = 90;
const WRAP_SPACER_HEIGHT: u16 = 22;
const WRAP_TARGET_HEIGHT: u16 = 40;

#[test]
fn body_text_wraps_to_second_visual_line() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(120, 120, palette.background);
    let tree = UiTree::new(
        Text::new("This is a very long line that must wrap")
            .text_role("body")
            .wrap(UiTextWrapMode::Wrap),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        tree.root(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert!(range_has_content(&canvas, palette.background, 40, 92));
}

#[test]
fn container_width_constrains_child_text_wrap_area() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(800, 140, palette.background);
    let root: UiNode = Column::new()
        .child(
            Text::new("This document body line wraps when container width is enforced.")
                .wrap(UiTextWrapMode::Wrap),
        )
        .into();
    let tree = UiTree::new(root.width(UiDimension::Px(180)));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        tree.root(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 800,
            height: 140,
            scroll_y: 0.0,
        },
    );

    assert!(range_has_content(&canvas, palette.background, 40, 92));
}

#[test]
fn hover_surface_for_wrapped_text_uses_measured_child_height() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(160, 96, palette.background);
    let text: UiNode = Text::new("Hover surface wraps long document text onto multiple rows.")
        .wrap(UiTextWrapMode::Wrap)
        .into();
    let tree = UiTree::new(text.stable_node_id("target"))
        .with_hover_surface_for_node_id(Some(&"target".into()));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        tree.root(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 96,
            scroll_y: 0.0,
        },
    );

    let highlighted = pixel_at(&canvas, 4, 36);
    assert_ne!(Some(palette.background), highlighted);
    assert_ne!(Some(palette.hover_background), highlighted);
}

#[test]
fn clipped_hover_surface_does_not_paint_child_below_explicit_height() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(180, 80, palette.background);
    let surface = UiNode::new(UiNodeKind::Stack, "")
        .visual_role(UiVisualRole::HoverSurface)
        .height(UiDimension::Px(40))
        .child(Text::new(
            "Hover surface child must not bleed past clipped bounds",
        ));
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: WRAP_VIEWPORT_WIDTH as u32,
            viewport_height: WRAP_VIEWPORT_HEIGHT as u32,
            offset_y: WRAP_SCROLL_OFFSET_Y,
            content_height: WRAP_CONTENT_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(UiNode::new(UiNodeKind::Stack, "").height(UiDimension::Px(22)))
                .child(surface),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert!(rect_has_content(&canvas, palette.background, 0, 0, 180, 20));
    assert!(
        !rect_has_content(&canvas, palette.background, 0, 20, 180, 42),
        "hover surface child pixels must be clipped to the visible surface height"
    );
}

#[test]
fn clipped_tree_hover_surface_does_not_paint_below_original_hit_height() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(180, 80, palette.background);
    let target: UiNode = Text::new("Hover target with explicit viewer row height").into();
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 180,
            viewport_height: 80,
            offset_y: 42,
            content_height: 90,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(
                    UiNode::new(UiNodeKind::Stack, "").height(UiDimension::Px(WRAP_SPACER_HEIGHT)),
                )
                .child(
                    target
                        .stable_node_id("hover-target")
                        .height(UiDimension::Px(WRAP_TARGET_HEIGHT)),
                ),
        );
    let tree = UiTree::new(root).with_hover_surface_for_node_id(Some(&"hover-target".into()));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        tree.root(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert!(rect_has_content(&canvas, palette.background, 0, 0, 180, 20));
    assert!(
        !rect_has_content(&canvas, palette.background, 0, 20, 180, 42),
        "tree hover surface must clip to the visible half of the original 40px hit"
    );
}

#[test]
fn clipped_tree_hover_surface_only_changes_visible_hit_pixels() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root = scrolled_hover_target_tree(false);
    let hovered_root = scrolled_hover_target_tree(true);
    let mut normal = Canvas::new(180, 80, palette.background);
    let mut hovered = Canvas::new(180, 80, palette.background);

    UiTreeCanvasRenderer::new(theme.clone()).render(&mut normal, root.root(), render_area(180, 80));
    UiTreeCanvasRenderer::new(theme).render(
        &mut hovered,
        hovered_root.root(),
        render_area(180, 80),
    );

    assert!(diff_in_rect(&normal, &hovered, 0, 0, 180, 20) > 0);
    assert_eq!(
        0,
        diff_in_rect(&normal, &hovered, 0, 20, 180, 42),
        "hover surface must not change pixels below the clipped visible hit height"
    );
}

#[test]
fn text_right_padding_constrains_wrap_area() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut no_padding = Canvas::new(260, 120, palette.background);
    let mut padded = Canvas::new(260, 120, palette.background);
    let text = "wide text reaches the reserved right side before clipping";

    UiTreeCanvasRenderer::new(theme.clone()).render(
        &mut no_padding,
        UiTree::new(Text::new(text).wrap(UiTextWrapMode::Wrap)).root(),
        render_area(260, 120),
    );
    UiTreeCanvasRenderer::new(theme).render(
        &mut padded,
        UiTree::new(Text::new(text).wrap(UiTextWrapMode::Wrap).common(
            UiCommonProps::default().padding(UiEdgeInsets {
                right: UiDimension::Px(120),
                ..UiEdgeInsets::default()
            }),
        ))
        .root(),
        render_area(260, 120),
    );

    assert!(rect_has_content(
        &no_padding,
        palette.background,
        150,
        8,
        255,
        28
    ));
    assert!(!rect_has_content(
        &padded,
        palette.background,
        150,
        8,
        255,
        28
    ));
}

fn scrolled_hover_target_tree(hovered: bool) -> UiTree {
    let target: UiNode = Text::new("5.5 Short + Long + Short Columns")
        .text_role("heading")
        .font_role("document-body")
        .text_spans(vec![
            UiTextSpan::plain("5.5"),
            UiTextSpan::plain(" Short + Long + Short Columns"),
        ])
        .into();
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: WRAP_VIEWPORT_WIDTH as u32,
            viewport_height: WRAP_VIEWPORT_HEIGHT as u32,
            offset_y: WRAP_SCROLL_OFFSET_Y,
            content_height: WRAP_CONTENT_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(
                    UiNode::new(UiNodeKind::Stack, "").height(UiDimension::Px(WRAP_SPACER_HEIGHT)),
                )
                .child(
                    target
                        .stable_node_id("hover-target")
                        .height(UiDimension::Px(WRAP_TARGET_HEIGHT)),
                ),
        );
    let tree = UiTree::new(root);
    if hovered {
        tree.with_hover_surface_for_node_id(Some(&"hover-target".into()))
    } else {
        tree
    }
}
