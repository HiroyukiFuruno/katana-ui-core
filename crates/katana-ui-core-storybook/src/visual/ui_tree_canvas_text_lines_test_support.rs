use super::*;

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn html_span_node(
    label: &str,
    role: &str,
) -> UiNode {
    Text::new(label)
        .text_role(role)
        .text_spans(vec![UiTextSpan {
            text: label.to_string(),
            style: Default::default(),
            link_target: String::new(),
        }])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn emoji_node() -> UiNode {
    Text::new("🔥")
        .text_role("body")
        .text_spans(vec![UiTextSpan {
            text: "🔥".to_string(),
            style: UiTextSpanStyle::default().emoji(),
            link_target: String::new(),
        }])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn whitespace_strike_node() -> UiNode {
    Text::new("          ")
        .font_role("code")
        .text_role("code")
        .text_spans(vec![UiTextSpan {
            text: "          ".to_string(),
            style: UiTextSpanStyle {
                strikethrough: true,
                color_rgba: [0xcc, 0x66, 0x33, 0xff],
                ..UiTextSpanStyle::default()
            },
            link_target: String::new(),
        }])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn underline_node() -> UiNode {
    Text::new("Underline")
        .text_role("body")
        .text_spans(vec![UiTextSpan {
            text: "Underline".to_string(),
            style: UiTextSpanStyle {
                underline: true,
                ..UiTextSpanStyle::default()
            },
            link_target: "https://example.com".to_string(),
        }])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn padded_link_node() -> UiNode {
    Text::new("English  日本語 ")
        .text_role("body")
        .text_spans(vec![
            UiTextSpan::plain("English"),
            UiTextSpan {
                text: " 日本語 ".to_string(),
                style: UiTextSpanStyle::default(),
                link_target: "#lang-ja".to_string(),
            },
        ])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn html_right_link_node() -> UiNode {
    Text::new("Right aligned link")
        .text_role("html-right")
        .text_spans(vec![UiTextSpan {
            text: "Right aligned link".to_string(),
            style: UiTextSpanStyle::default(),
            link_target: "https://example.com/kdv".to_string(),
        }])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn html_padded_link_node() -> UiNode {
    Text::new("日本語  ")
        .text_role("html-left")
        .text_spans(vec![UiTextSpan {
            text: "日本語  ".to_string(),
            style: UiTextSpanStyle::default(),
            link_target: "https://example.com/kdv".to_string(),
        }])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn split_space_node() -> UiNode {
    Text::new("H1 Heading")
        .text_role("heading")
        .text_spans(vec![
            UiTextSpan::plain("H1"),
            UiTextSpan::plain(" "),
            UiTextSpan::plain("Heading"),
        ])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn single_space_node() -> UiNode {
    Text::new("H1 Heading")
        .text_role("heading")
        .text_spans(vec![UiTextSpan::plain("H1 Heading")])
        .into()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn horizontal_ink_bounds(
    canvas: &Canvas,
) -> Option<(usize, usize)> {
    let mut min_x = usize::MAX;
    let mut max_x = 0usize;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if *pixel == TEST_BACKGROUND {
            continue;
        }
        let x = index % canvas.width();
        min_x = min_x.min(x);
        max_x = max_x.max(x);
    }
    (min_x <= max_x).then_some((min_x, max_x))
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn widest_empty_column_run_between_ink(
    canvas: &Canvas,
    background: u32,
) -> Option<usize> {
    let (min_x, max_x) = horizontal_non_background_bounds(canvas, background)?;
    let mut widest = 0usize;
    let mut current = 0usize;
    for x in min_x..=max_x {
        if column_has_ink(canvas, x, background) {
            widest = widest.max(current);
            current = 0;
            continue;
        }
        current += 1;
    }
    Some(widest.max(current))
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn horizontal_non_background_bounds(
    canvas: &Canvas,
    background: u32,
) -> Option<(usize, usize)> {
    let mut min_x = usize::MAX;
    let mut max_x = 0usize;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if *pixel == background {
            continue;
        }
        let x = index % canvas.width();
        min_x = min_x.min(x);
        max_x = max_x.max(x);
    }
    (min_x <= max_x).then_some((min_x, max_x))
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn vertical_non_background_bounds(
    canvas: &Canvas,
    background: u32,
) -> Option<(usize, usize)> {
    let mut min_y = usize::MAX;
    let mut max_y = 0usize;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if *pixel == background {
            continue;
        }
        let y = index / canvas.width();
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    (min_y <= max_y).then_some((min_y, max_y))
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn column_has_ink(
    canvas: &Canvas,
    x: usize,
    background: u32,
) -> bool {
    (0..canvas.height()).any(|y| canvas.pixels()[y * canvas.width() + x] != background)
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn row_color_count(
    canvas: &Canvas,
    y: usize,
    color: u32,
) -> usize {
    let start = y * canvas.width();
    canvas.pixels()[start..start + canvas.width()]
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn row_color_bounds(
    canvas: &Canvas,
    y: usize,
    color: u32,
) -> Option<(usize, usize)> {
    let start = y.checked_mul(canvas.width())?;
    let row = canvas
        .pixels()
        .get(start..start.checked_add(canvas.width())?)?;
    let min_x = row.iter().position(|pixel| *pixel == color)?;
    let max_x = row.iter().rposition(|pixel| *pixel == color)?;
    Some((min_x, max_x))
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn chromatic_pixel_count(
    canvas: &Canvas,
) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel != TEST_BACKGROUND)
        .filter(|pixel| is_chromatic(**pixel))
        .count()
}

pub(in crate::visual::ui_tree_canvas_text::text_lines) fn is_chromatic(pixel: u32) -> bool {
    let red = (pixel >> RED_SHIFT) & CHANNEL_MASK;
    let green = (pixel >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = pixel & CHANNEL_MASK;
    red != green || green != blue
}
