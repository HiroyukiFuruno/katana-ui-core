use super::layout_metrics::LayoutRect;
use super::visual_interaction_test_support::{pixel_at, rect_pixel_diff};
use super::window_interaction::{StorybookWindowState, TextInputKey, apply_click};
use super::{Canvas, palette, preview_detail, render, window_interaction};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-input";
const CARET_HIDDEN_FRAME: usize = 30;
const RUNTIME_MARKER_HEIGHT: usize = 4;
const LABEL_SIZE: f32 = 10.0;
const CHIP_TEST_WIDTH: usize = 68;
const CHIP_TEST_HEIGHT: usize = 18;
const CHIP_TEST_MIN_PADDING: usize = 2;
const FIELD_BORDER_WIDTH: usize = 1;
const FIELD_INNER_LEFT_MARGIN: usize = 2;
const TRAILING_BUTTON_LABELS: [&str; 3] = [".*", "ab", "Aa"];

#[test]
fn text_input_focus_cursor_blinks_inside_field() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let value_width = measured_body_width(state.screen_state.text_input_value());
    let caret = super::dedicated_dod_form_input_live::text_input_caret_rect_for_test(
        origin.x,
        origin.y,
        value_width,
    );
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    let inactive = render_with_state(&state);
    assert_eq!(Some(colors.surface), pixel_at(&inactive, caret.x, caret.y));

    let field = text_input_field_rect();
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    let active = render_with_state(&state);
    assert_eq!(Some(colors.accent), pixel_at(&active, caret.x, caret.y));

    assert!(
        state
            .screen_state
            .update_text_input_caret_visibility(CARET_HIDDEN_FRAME)
    );
    let hidden = render_with_state(&state);
    assert_eq!(Some(colors.surface), pixel_at(&hidden, caret.x, caret.y));
}

#[test]
fn text_input_caret_touches_measured_value_edge() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let value_width = measured_body_width("abcdefb");
    let caret = super::dedicated_dod_form_input_live::text_input_caret_rect_for_test(
        origin.x,
        origin.y,
        value_width,
    );

    assert_eq!(
        origin.x + super::dedicated_dod_form_input_live::FIELD_TEXT_X + value_width,
        caret.x
    );
}

#[test]
fn text_input_default_origin_uses_two_px_field_padding() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let field = super::dedicated_dod_form_input_live::search_field_rect(origin.x, origin.y);
    let text_x = super::dedicated_dod_form_input_live::text_input_text_x(origin.x, false);

    assert_eq!(
        field.x + FIELD_BORDER_WIDTH + FIELD_INNER_LEFT_MARGIN,
        text_x
    );
}

#[test]
fn text_input_empty_caret_uses_two_px_inner_left_margin() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let field = super::dedicated_dod_form_input_live::search_field_rect(origin.x, origin.y);
    let text_x = super::dedicated_dod_form_input_live::text_input_text_x(origin.x, false);
    let clip_width =
        super::dedicated_dod_form_input_live::text_input_text_clip_width(false, false, false);
    let caret = super::dedicated_dod_form_input_live::text_input_caret_rect_with_layout_for_test(
        text_x, origin.y, clip_width, 0,
    );

    assert_eq!(
        field.x + FIELD_BORDER_WIDTH + FIELD_INNER_LEFT_MARGIN,
        caret.x
    );
}

#[test]
fn text_input_leading_icon_is_vertically_centered_with_text_field() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let field = super::dedicated_dod_form_input_live::search_field_rect(origin.x, origin.y);
    let icon = super::dedicated_dod_form_input_live::text_input_search_icon_visual_rect_for_test(
        origin.x, origin.y,
    );

    assert!(
        center_y_twice(field).abs_diff(center_y_twice(icon)) <= 1,
        "icon center must align with text field center"
    );
}

#[test]
fn text_input_reserved_icon_space_keeps_legacy_left_slot_width() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let value_width = measured_body_width("search");
    let text_x = super::dedicated_dod_form_input_live::text_input_text_x(origin.x, true);
    let clip_width =
        super::dedicated_dod_form_input_live::text_input_text_clip_width(true, false, false);
    let caret = super::dedicated_dod_form_input_live::text_input_caret_rect_with_layout_for_test(
        text_x,
        origin.y,
        clip_width,
        value_width,
    );

    assert_eq!(
        origin.x + super::dedicated_dod_form_input_live::FIELD_TEXT_X_WITH_LEADING_SLOT,
        text_x
    );
    assert_eq!(text_x + value_width, caret.x);
}

#[test]
fn text_input_chip_labels_keep_inner_vertical_padding() {
    let facade = UiCoreFacade::default();
    let text = super::text::TextRenderer::load(&facade, "body");
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = super::dedicated_dod_common::Rect::new(4, 4, CHIP_TEST_WIDTH, CHIP_TEST_HEIGHT);
    let mut canvas = Canvas::new(96, 40, palette.surface);

    super::dedicated_dod_common::chip(&mut canvas, &text, &palette, rect, "IME", palette.accent);

    let (top, bottom) = non_chip_fill_bounds(&canvas, rect, palette.accent, palette.border);
    assert!(top <= bottom);
    assert!(top >= rect.y + CHIP_TEST_MIN_PADDING);
    assert!(bottom + CHIP_TEST_MIN_PADDING <= rect.y + rect.height);
}

#[test]
fn text_input_trailing_icon_button_labels_use_centered_text_box_layout() {
    let facade = UiCoreFacade::default();
    let text = super::text::TextRenderer::load(&facade, "body");
    let origin = preview_detail::component_action_hit_rect(PAGE);

    for (rect, label) in
        super::dedicated_dod_form_input_live::text_input_trailing_icon_button_rects(
            origin.x, origin.y,
        )
        .into_iter()
        .zip(TRAILING_BUTTON_LABELS)
    {
        let size = super::dedicated_dod_metrics::FONT_8;
        let origin = text.origin_in_box_for_test(
            label,
            super::text::TextBox::centered(rect.x, rect.y, rect.width, rect.height),
            size,
        );

        assert_eq!(
            rect.x + rect.width.saturating_sub(text.measure_width(label, size)) / 2,
            origin.x,
            "horizontal centering must be computed from the button rect and measured label width"
        );
        assert_eq!(
            rect.y, origin.y,
            "vertical centering must use the button rect as the text line box"
        );
    }
}

#[test]
fn text_input_status_chips_and_runtime_marker_do_not_overlap() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let frame = LayoutRect::new(
        origin.x,
        origin.y,
        super::dedicated_dod_common::AREA_WIDTH,
        super::dedicated_dod_common::AREA_HEIGHT,
    );

    assert_eq!(super::dedicated_dod_common::AREA_HEIGHT, origin.height);
    for rect in super::dedicated_dod_form_input_live::text_input_status_rects(origin.x, origin.y) {
        assert!(rect_inside(rect, frame));
    }
    for rect in super::dedicated_dod_form_input_live::text_input_chip_rects(origin.x, origin.y) {
        assert!(rect_inside(rect, frame));
    }
    for status in super::dedicated_dod_form_input_live::text_input_status_rects(origin.x, origin.y)
    {
        for chip in super::dedicated_dod_form_input_live::text_input_chip_rects(origin.x, origin.y)
        {
            assert!(!status.overlaps(chip));
        }
    }

    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let field = text_input_field_rect();
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('x')
    ));
    let after = render_with_state(&state);
    let marker = LayoutRect::new(
        origin.x,
        origin.bottom() - RUNTIME_MARKER_HEIGHT,
        origin.width,
        1,
    );
    let outside = LayoutRect::new(origin.x, origin.bottom(), origin.width, 1);

    assert!(rect_pixel_diff(marker, &before, &after) > 0);
    assert_eq!(0, rect_pixel_diff(outside, &before, &after));
}

fn render_with_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn text_input_field_rect() -> LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y)
}

fn measured_body_width(value: &str) -> usize {
    let facade = UiCoreFacade::default();
    let text = super::text::TextRenderer::load(&facade, "body");
    text.measure_width(value, LABEL_SIZE)
}

fn rect_inside(inner: LayoutRect, outer: LayoutRect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}

fn center_y_twice(rect: LayoutRect) -> usize {
    rect.y * 2 + rect.height
}

fn non_chip_fill_bounds(
    canvas: &Canvas,
    rect: super::dedicated_dod_common::Rect,
    fill: u32,
    border: u32,
) -> (usize, usize) {
    let mut top = rect.y + rect.height;
    let mut bottom = rect.y;
    for y in rect.y + 1..rect.y + rect.height - 1 {
        for x in rect.x + 1..rect.x + rect.width - 1 {
            let pixel = canvas.pixels()[y * canvas.width() + x];
            if pixel != fill && pixel != border {
                top = top.min(y);
                bottom = bottom.max(y);
            }
        }
    }
    (top, bottom)
}
