use super::style::CodeDiffStyle;
use super::types::{CodeDiffLine, CodeDiffLineKind, CodeDiffTextRange};
use floem::IntoView;
use floem::peniko::Color;
use floem::views::{Decorators, button, h_stack, label};

const FONT_SIZE: f32 = 12.0;
const LINE_HEIGHT: f32 = 22.0;
const NUMBER_WIDTH: f32 = 42.0;
const MARK_WIDTH: f32 = 18.0;
const CODE_MIN_WIDTH: f32 = 360.0;
const ROW_PADDING_X: f32 = 8.0;

pub(crate) struct CodeDiffRowView;

impl CodeDiffRowView {
    pub(crate) fn code_line(line: CodeDiffLine, style: CodeDiffStyle) -> impl IntoView {
        let number = line
            .line_number
            .map_or(String::new(), |value| value.to_string());
        let mark = match line.kind {
            CodeDiffLineKind::Added => "+",
            CodeDiffLineKind::Removed => "−",
            CodeDiffLineKind::Equal => " ",
            CodeDiffLineKind::Placeholder => "·",
        };
        let text = display_text(&line.text, &line.highlights);
        let row_bg = style.row_bg(line.kind);
        let mark_color = style.mark(line.kind);
        let text_color = code_color(line.kind, style.text);

        h_stack((
            label(move || number.clone()).style(move |style| {
                style
                    .width(NUMBER_WIDTH)
                    .color(mark_color)
                    .font_size(FONT_SIZE)
            }),
            label(move || mark).style(move |style| {
                style
                    .width(MARK_WIDTH)
                    .color(mark_color)
                    .font_size(FONT_SIZE)
            }),
            label(move || text.clone()).style(move |style| {
                style
                    .min_width(CODE_MIN_WIDTH)
                    .font_size(FONT_SIZE)
                    .color(text_color)
            }),
        ))
        .style(move |style| {
            style
                .height(LINE_HEIGHT)
                .items_center()
                .padding_horiz(ROW_PADDING_X)
                .background(row_bg)
        })
    }

    pub(crate) fn omitted_row(
        block_index: usize,
        hidden_count: usize,
        expanded: bool,
        style: CodeDiffStyle,
        on_toggle: impl Fn(usize) + 'static,
    ) -> impl IntoView {
        let icon = if expanded { "▾" } else { "▸" };
        let text = format!("{icon} 非表示 {hidden_count} 行");
        button(label(move || text.clone()).style(move |s| s.font_size(FONT_SIZE).color(style.text)))
            .action(move || on_toggle(block_index))
            .style(move |s| {
                s.width_full()
                    .height(LINE_HEIGHT)
                    .background(style.omitted_bg)
                    .border(1.0)
                    .border_color(style.border)
            })
    }
}

fn display_text(text: &str, ranges: &[CodeDiffTextRange]) -> String {
    if text.is_empty() {
        return "↵".to_string();
    }

    text.chars()
        .enumerate()
        .map(|(index, value)| {
            if ranges.iter().any(|range| range.contains(index)) {
                visible_changed_char(value)
            } else {
                value
            }
        })
        .collect()
}

fn visible_changed_char(value: char) -> char {
    match value {
        ' ' => '·',
        '\t' => '→',
        _ => value,
    }
}

fn code_color(kind: CodeDiffLineKind, default_color: Color) -> Color {
    match kind {
        CodeDiffLineKind::Added | CodeDiffLineKind::Removed | CodeDiffLineKind::Equal => {
            default_color
        }
        CodeDiffLineKind::Placeholder => default_color,
    }
}
