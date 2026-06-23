use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn document_alert_uses_alert_kind_accent_for_stripe() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 120, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "Tip\nBody")
        .text(UiTextProps {
            role: "alert".to_string(),
            ..UiTextProps::default()
        })
        .severity(UiTone::Success)
        .border(UiBorder::solid(4, 0, "alert-tip"))
        .height(UiDimension::Px(92));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 56, 15));
    assert_eq!(Some(palette.alert_tip_accent), pixel_at(&canvas, 56, 16));
    assert_eq!(Some(palette.alert_tip_accent), pixel_at(&canvas, 60, 16));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 61, 16));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 64, 16));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 120, 16));
    assert!(
        count_pixel(&canvas, palette.alert_tip_accent) > 120,
        "alert must draw the accent stripe and outline icon without a filled card"
    );
    assert!(
        count_pixel(&canvas, palette.alert_background) < 16,
        "document alert must not fill a full background panel"
    );
}

#[test]
fn document_alert_icon_y_matches_kdv_export_surface_alignment() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 120, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "Warning\nBody")
        .text(UiTextProps {
            role: "alert".to_string(),
            ..UiTextProps::default()
        })
        .severity(UiTone::Warning)
        .border(UiBorder::solid(5, 0, "alert-warning"))
        .height(UiDimension::Px(92));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        Some(palette.alert_warning_accent),
        pixel_at(&canvas, 93, 20)
    );
    assert_eq!(
        Some(palette.alert_warning_accent),
        pixel_at(&canvas, 93, 27)
    );
}

#[test]
fn document_alert_icons_use_kdv_export_surface_shapes() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 180, palette.background);
    let root = UiNode::from(
        Column::new()
            .child(
                alert_with_tone("Tip\nBody", UiTone::Success).border(UiBorder::solid(
                    5,
                    0,
                    "alert-tip",
                )),
            )
            .child(
                alert_with_tone("Caution\nBody", UiTone::Danger).border(UiBorder::solid(
                    5,
                    0,
                    "alert-caution",
                )),
            ),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 180,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.alert_tip_accent), pixel_at(&canvas, 94, 20));
    assert_ne!(Some(palette.alert_tip_accent), pixel_at(&canvas, 87, 31));
    assert_eq!(
        Some(palette.alert_caution_accent),
        pixel_at(&canvas, 91, 66)
    );
    assert_ne!(
        Some(palette.alert_caution_accent),
        pixel_at(&canvas, 94, 82)
    );
}

#[test]
fn document_alert_tone_fallback_uses_gfm_warning_and_caution_order() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 160, palette.background);
    let root = UiNode::from(
        Column::new()
            .child(alert_with_tone("Warning\nBody", UiTone::Warning))
            .child(alert_with_tone("Caution\nBody", UiTone::Danger)),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 160,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        Some(palette.alert_warning_accent),
        pixel_at(&canvas, 56, 16)
    );
    assert_eq!(
        Some(palette.alert_caution_accent),
        pixel_at(&canvas, 56, 62)
    );
}

#[test]
fn text_node_explicit_height_controls_layout_advance() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 160, palette.background);
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(
            UiNode::new(UiNodeKind::Text, "Tall\nblock")
                .text(UiTextProps {
                    role: "alert".to_string(),
                    ..UiTextProps::default()
                })
                .height(UiDimension::Px(124)),
        )
        .child(
            UiNode::new(UiNodeKind::Text, "Next")
                .text(UiTextProps {
                    role: "body".to_string(),
                    ..UiTextProps::default()
                })
                .height(UiDimension::Px(46)),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 264,
            height: 160,
            scroll_y: 0.0,
        },
    );

    let next_y = first_row_containing_color_after(&canvas, palette.text, 100)
        .kuc_expect("second text node should advance by explicit first height");
    assert!(
        next_y >= 124,
        "next text y must respect explicit height: {next_y}"
    );
}
