use super::{
    ALERT_ICON_CENTER, ALERT_ICON_DOT_RADIUS, ALERT_ICON_X_OFFSET, ALERT_ICON_Y_OFFSET,
    ALERT_PANEL_PADDING_Y, ALERT_STRIPE_WIDTH, CAUTION_ICON_DOT_Y, CAUTION_ICON_OUTLINE,
    CAUTION_ICON_STEM_BOTTOM, CAUTION_ICON_STEM_TOP, Canvas, IMPORTANT_ICON_DOT_Y,
    IMPORTANT_ICON_OUTLINE, IMPORTANT_ICON_STEM_BOTTOM, IMPORTANT_ICON_STEM_TOP,
    NOTE_ICON_DOT_RADIUS, NOTE_ICON_DOT_Y, NOTE_ICON_RADIUS, NOTE_ICON_STEM_BOTTOM,
    NOTE_ICON_STEM_TOP, TIP_ICON_BASE_END_X, TIP_ICON_BASE_START_X, TIP_ICON_BASE_Y,
    TIP_ICON_CENTER_Y, TIP_ICON_LINE_END_X, TIP_ICON_LINE_START_X, TIP_ICON_LINE_Y,
    TIP_ICON_RADIUS, UiNode, UiTone, UiTreeCanvasPalette, UiTreeRenderArea, UiTreeTextMetrics,
    WARNING_ICON_DOT_Y, WARNING_ICON_OUTLINE, WARNING_ICON_STEM_BOTTOM, WARNING_ICON_STEM_TOP,
};

pub(super) fn draw_alert(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    _area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    metrics: UiTreeTextMetrics,
) {
    let compact_height = metrics
        .background_height
        .saturating_sub(ALERT_PANEL_PADDING_Y.saturating_mul(2));
    let (stripe_y, stripe_height) = if compact_height == 0 {
        (y, metrics.background_height)
    } else {
        (y.saturating_add(ALERT_PANEL_PADDING_Y), compact_height)
    };
    canvas.fill_rect(
        x,
        stripe_y,
        ALERT_STRIPE_WIDTH,
        stripe_height,
        alert_accent(node, palette),
    );
    draw_alert_icon(canvas, node, x, y, palette);
}

pub(super) fn alert_accent(node: &UiNode, palette: UiTreeCanvasPalette) -> u32 {
    match node.props().common.border.color_token.as_str() {
        "alert-tip" => palette.alert_tip_accent,
        "alert-important" => palette.alert_important_accent,
        "alert-warning" => palette.alert_warning_accent,
        "alert-caution" => palette.alert_caution_accent,
        "alert-note" => palette.alert_note_accent,
        _ => alert_accent_from_tone(node.props().severity, palette),
    }
}

pub(super) fn alert_accent_from_tone(tone: UiTone, palette: UiTreeCanvasPalette) -> u32 {
    match tone {
        UiTone::Success => palette.alert_tip_accent,
        UiTone::Warning => palette.alert_warning_accent,
        UiTone::Danger => palette.alert_caution_accent,
        UiTone::Accent => palette.alert_note_accent,
        UiTone::Neutral => palette.alert_note_accent,
    }
}

pub(super) fn draw_alert_icon(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    palette: UiTreeCanvasPalette,
) {
    let color = alert_accent(node, palette);
    let icon_x = x.saturating_add(ALERT_ICON_X_OFFSET);
    let icon_y = y.saturating_add(ALERT_ICON_Y_OFFSET);
    match node.props().common.border.color_token.as_str() {
        "alert-tip" => draw_tip_icon(canvas, icon_x, icon_y, color, palette.background),
        "alert-important" => draw_important_icon(canvas, icon_x, icon_y, color),
        "alert-warning" => draw_warning_icon(canvas, icon_x, icon_y, color),
        "alert-caution" => draw_caution_icon(canvas, icon_x, icon_y, color),
        _ => draw_note_icon(canvas, icon_x, icon_y, color),
    }
}

pub(super) fn draw_note_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    draw_stroked_circle(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(ALERT_ICON_CENTER),
        NOTE_ICON_RADIUS,
        color,
    );
    draw_stroked_line(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(NOTE_ICON_STEM_TOP),
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(NOTE_ICON_STEM_BOTTOM),
        color,
    );
    draw_filled_circle(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(NOTE_ICON_DOT_Y),
        NOTE_ICON_DOT_RADIUS,
        color,
    );
}

pub(super) fn draw_tip_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32, background: u32) {
    draw_stroked_circle_arc(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(TIP_ICON_CENTER_Y),
        TIP_ICON_RADIUS,
        color,
        background,
    );
    draw_stroked_line(
        canvas,
        x.saturating_add(TIP_ICON_LINE_START_X),
        y.saturating_add(TIP_ICON_LINE_Y),
        x.saturating_add(TIP_ICON_LINE_END_X),
        y.saturating_add(TIP_ICON_LINE_Y),
        color,
    );
    draw_stroked_line(
        canvas,
        x.saturating_add(TIP_ICON_BASE_START_X),
        y.saturating_add(TIP_ICON_BASE_Y),
        x.saturating_add(TIP_ICON_BASE_END_X),
        y.saturating_add(TIP_ICON_BASE_Y),
        color,
    );
}

pub(super) fn draw_important_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    draw_outline(canvas, x, y, color, &IMPORTANT_ICON_OUTLINE);
    draw_stroked_line(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(IMPORTANT_ICON_STEM_TOP),
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(IMPORTANT_ICON_STEM_BOTTOM),
        color,
    );
    draw_filled_circle(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(IMPORTANT_ICON_DOT_Y),
        ALERT_ICON_DOT_RADIUS,
        color,
    );
}

pub(super) fn draw_warning_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    draw_outline(canvas, x, y, color, &WARNING_ICON_OUTLINE);
    draw_stroked_line(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(WARNING_ICON_STEM_TOP),
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(WARNING_ICON_STEM_BOTTOM),
        color,
    );
    draw_filled_circle(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(WARNING_ICON_DOT_Y),
        ALERT_ICON_DOT_RADIUS,
        color,
    );
}

pub(super) fn draw_caution_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    draw_outline(canvas, x, y, color, &CAUTION_ICON_OUTLINE);
    draw_stroked_line(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(CAUTION_ICON_STEM_TOP),
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(CAUTION_ICON_STEM_BOTTOM),
        color,
    );
    draw_filled_circle(
        canvas,
        x.saturating_add(ALERT_ICON_CENTER),
        y.saturating_add(CAUTION_ICON_DOT_Y),
        ALERT_ICON_DOT_RADIUS,
        color,
    );
}

#[path = "ui_tree_canvas_text_role_alert_primitives.rs"]
mod primitives;
use primitives::{
    draw_filled_circle, draw_outline, draw_stroked_circle, draw_stroked_circle_arc,
    draw_stroked_line,
};

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::{UiBorder, UiDimension, UiNodeKind};
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn important_icon_and_remaining_tone_fallbacks_are_drawn() {
        let palette = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::light());
        assert_eq!(
            palette.alert_tip_accent,
            alert_accent_from_tone(UiTone::Success, palette)
        );
        assert_eq!(
            palette.alert_note_accent,
            alert_accent_from_tone(UiTone::Accent, palette)
        );

        let mut canvas = Canvas::new(48, 48, palette.background);
        draw_important_icon(&mut canvas, 4, 4, palette.alert_important_accent);

        assert!(
            canvas.non_background_pixels(palette.background) > 20,
            "important icon must paint its outline, stem, and dot"
        );
    }

    #[test]
    fn alert_tokens_cover_all_export_icon_shapes_and_tone_fallbacks() {
        let palette = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::light());
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 96,
            height: 72,
            scroll_y: 0.0,
        };
        for (token, tone, expected) in [
            ("alert-tip", UiTone::Success, palette.alert_tip_accent),
            (
                "alert-important",
                UiTone::Accent,
                palette.alert_important_accent,
            ),
            (
                "alert-warning",
                UiTone::Warning,
                palette.alert_warning_accent,
            ),
            (
                "alert-caution",
                UiTone::Danger,
                palette.alert_caution_accent,
            ),
            ("alert-note", UiTone::Neutral, palette.alert_note_accent),
        ] {
            let node = UiNode::new(UiNodeKind::Text, "Alert")
                .severity(tone)
                .border(UiBorder::solid(1, 0, token))
                .height(UiDimension::Px(64));
            let mut canvas = Canvas::new(96, 72, palette.background);
            draw_alert(
                &mut canvas,
                &node,
                4,
                4,
                area,
                palette,
                UiTreeTextMetrics::for_node(&node),
            );

            assert_eq!(expected, alert_accent(&node, palette));
            assert!(canvas.pixels().contains(&expected));
        }
        for (tone, expected) in [
            (UiTone::Success, palette.alert_tip_accent),
            (UiTone::Warning, palette.alert_warning_accent),
            (UiTone::Danger, palette.alert_caution_accent),
            (UiTone::Accent, palette.alert_note_accent),
            (UiTone::Neutral, palette.alert_note_accent),
        ] {
            assert_eq!(expected, alert_accent_from_tone(tone, palette));
        }
    }
}
