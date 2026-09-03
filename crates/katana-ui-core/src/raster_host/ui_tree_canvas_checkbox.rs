use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_hit_metrics::dimension_px;
use super::ui_tree_canvas_hover::UiTreeCanvasHover;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::UiNode;

const TEXT_HEIGHT: usize = 20;
const TEXT_SIZE: f32 = 14.0;
const CHECKBOX_SIZE: usize = 16;
const CHECKBOX_INSET: usize = 2;
const CHECKBOX_LABEL_GAP: usize = 8;
const CHECKBOX_MARK_COLOR: u32 = 0x00ff_ffff;
const MARK_UNIT_DENOMINATOR: usize = 16;
const MARK_QUARTER_UNITS: usize = 4;
const DONE_MARK_KNEE_X_UNITS: usize = 7;
const DONE_MARK_LEFT_Y_UNITS: usize = 9;
const MARK_THREE_QUARTER_UNITS: usize = 12;
const CHECKBOX_STROKE_DIVISOR: usize = 8;
const MIN_CHECKBOX_STROKE: usize = 2;

pub(super) struct UiTreeCheckboxRenderer;

impl UiTreeCheckboxRenderer {
    pub(super) fn draw(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        let row_height = checkbox_row_height(node);
        let box_size = checkbox_box_size(row_height);
        let box_y = checkbox_y(*y, row_height, box_size);
        let mark = checkbox_mark(node);
        draw_box(canvas, x, box_y, box_size, mark.is_some(), palette);
        if let Some(mark) = mark {
            draw_mark(canvas, x, box_y, box_size, mark, checkbox_mark_color());
        }
        if !node.props().label.is_empty() {
            text.draw(
                canvas,
                &node.props().label,
                x + checkbox_label_gap(box_size),
                *y,
                TEXT_SIZE,
                palette.text,
            );
        }
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            *y,
            checkbox_row_width(text, node),
            row_height,
            palette,
        );
        *y = y.saturating_add(row_height);
    }
}

pub(super) fn checkbox_row_height(node: &UiNode) -> usize {
    dimension_px(&node.props().common.height).max(TEXT_HEIGHT)
}

fn checkbox_box_size(row_height: usize) -> usize {
    row_height
        .saturating_sub(CHECKBOX_INSET.saturating_mul(2))
        .clamp(1, CHECKBOX_SIZE)
}

fn checkbox_y(y: usize, row_height: usize, box_size: usize) -> usize {
    y.saturating_add(row_height.saturating_sub(box_size) / 2)
}

fn checkbox_label_gap(box_size: usize) -> usize {
    box_size.saturating_add(CHECKBOX_LABEL_GAP)
}

fn draw_box(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    size: usize,
    filled: bool,
    palette: UiTreeCanvasPalette,
) {
    if filled {
        canvas.fill_rect(x, y, size, size, palette.selection);
        canvas.stroke_rect(x, y, size, size, palette.selection);
        return;
    }
    canvas.fill_rect(x, y, size, size, palette.preview_background);
    canvas.stroke_rect(x, y, size, size, palette.muted_border);
}

fn checkbox_mark_color() -> u32 {
    CHECKBOX_MARK_COLOR
}

fn draw_mark(canvas: &mut Canvas, x: usize, y: usize, size: usize, mark: CheckboxMark, color: u32) {
    match mark {
        CheckboxMark::Done => draw_done_mark(canvas, x, y, size, color),
        CheckboxMark::Progress => draw_progress_mark(canvas, x, y, size, color),
        CheckboxMark::Blocked => draw_blocked_mark(canvas, x, y, size, color),
    }
}

fn draw_done_mark(canvas: &mut Canvas, x: usize, y: usize, size: usize, color: u32) {
    draw_line(
        canvas,
        (
            x + scaled_mark_coord(size, MARK_QUARTER_UNITS),
            y + scaled_mark_coord(size, DONE_MARK_LEFT_Y_UNITS),
        ),
        (
            x + scaled_mark_coord(size, DONE_MARK_KNEE_X_UNITS),
            y + scaled_mark_coord(size, MARK_THREE_QUARTER_UNITS),
        ),
        size,
        color,
    );
    draw_line(
        canvas,
        (
            x + scaled_mark_coord(size, DONE_MARK_KNEE_X_UNITS),
            y + scaled_mark_coord(size, MARK_THREE_QUARTER_UNITS),
        ),
        (
            x + scaled_mark_coord(size, MARK_THREE_QUARTER_UNITS),
            y + scaled_mark_coord(size, MARK_QUARTER_UNITS),
        ),
        size,
        color,
    );
}

fn draw_progress_mark(canvas: &mut Canvas, x: usize, y: usize, size: usize, color: u32) {
    draw_line(
        canvas,
        (
            x + scaled_mark_coord(size, MARK_QUARTER_UNITS),
            y + scaled_mark_coord(size, MARK_THREE_QUARTER_UNITS),
        ),
        (
            x + scaled_mark_coord(size, MARK_THREE_QUARTER_UNITS),
            y + scaled_mark_coord(size, MARK_QUARTER_UNITS),
        ),
        size,
        color,
    );
}

fn draw_blocked_mark(canvas: &mut Canvas, x: usize, y: usize, size: usize, color: u32) {
    let stroke = mark_stroke(size);
    canvas.fill_rect(
        x + scaled_mark_coord(size, MARK_QUARTER_UNITS),
        y + size.saturating_sub(stroke) / 2,
        size / 2,
        stroke,
        color,
    );
}

fn scaled_mark_coord(size: usize, units: usize) -> usize {
    size.saturating_mul(units) / MARK_UNIT_DENOMINATOR
}

fn draw_line(
    canvas: &mut Canvas,
    start: (usize, usize),
    end: (usize, usize),
    size: usize,
    color: u32,
) {
    let dx = end.0 as isize - start.0 as isize;
    let dy = end.1 as isize - start.1 as isize;
    let steps = dx.unsigned_abs().max(dy.unsigned_abs()).max(1);
    let stroke = mark_stroke(size);
    for step in 0..=steps {
        let x = start.0 as isize + dx * step as isize / steps as isize;
        let y = start.1 as isize + dy * step as isize / steps as isize;
        let x = x.max(0) as usize;
        let y = y.max(0) as usize;
        canvas.fill_rect(x, y, stroke, stroke, color);
    }
}

fn mark_stroke(size: usize) -> usize {
    (size / CHECKBOX_STROKE_DIVISOR).max(MIN_CHECKBOX_STROKE)
}

pub(super) fn checkbox_row_width(text: &TextRenderer, node: &UiNode) -> usize {
    if node.props().label.is_empty() {
        return checkbox_box_size(checkbox_row_height(node));
    }
    let box_size = checkbox_box_size(checkbox_row_height(node));
    checkbox_label_gap(box_size).saturating_add(text.measure_width(&node.props().label, TEXT_SIZE))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CheckboxMark {
    Done,
    Progress,
    Blocked,
}

fn checkbox_mark(node: &UiNode) -> Option<CheckboxMark> {
    match node.props().interaction.value.as_str() {
        "[x]" | "[X]" => Some(CheckboxMark::Done),
        "[/]" => Some(CheckboxMark::Progress),
        "[-]" => Some(CheckboxMark::Blocked),
        _ if node.props().checked => Some(CheckboxMark::Done),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CHECKBOX_SIZE, CheckboxMark, checkbox_mark, checkbox_mark_color, draw_box, draw_mark,
    };
    use crate::raster_host::canvas::Canvas;
    use crate::raster_host::palette::VisualPalette;
    use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use katana_ui_core::atom::Checkbox;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::{UiDimension, UiNode};

    const BACKGROUND: u32 = 0x010203;
    const SELECTION: u32 = 0x264f78;
    const BORDER: u32 = 0x30363d;

    #[test]
    fn checkbox_mark_uses_katana_task_state_value() {
        assert_eq!(None, checkbox_mark(&checkbox("[ ]", false)));
        assert_eq!(
            Some(CheckboxMark::Done),
            checkbox_mark(&checkbox("[x]", true))
        );
        assert_eq!(
            Some(CheckboxMark::Progress),
            checkbox_mark(&checkbox("[/]", true))
        );
        assert_eq!(
            Some(CheckboxMark::Blocked),
            checkbox_mark(&checkbox("[-]", true))
        );
    }

    #[test]
    fn checkbox_without_task_value_keeps_generic_checked_mark() {
        assert_eq!(Some(CheckboxMark::Done), checkbox_mark(&checkbox("", true)));
    }

    #[test]
    fn checked_task_checkbox_uses_selection_fill_and_light_mark() {
        let mut canvas = Canvas::new(24, 24, BACKGROUND);
        draw_box(&mut canvas, 2, 2, CHECKBOX_SIZE, true, palette());
        draw_mark(
            &mut canvas,
            2,
            2,
            CHECKBOX_SIZE,
            CheckboxMark::Done,
            checkbox_mark_color(),
        );

        assert!(count_color(&canvas, SELECTION) > 180);
        assert!(count_color(&canvas, checkbox_mark_color()) > 0);
    }

    #[test]
    fn task_progress_and_blocked_marks_have_visible_distinct_strokes() {
        let mut progress = Canvas::new(24, 24, BACKGROUND);
        let mut blocked = Canvas::new(24, 24, BACKGROUND);

        draw_mark(
            &mut progress,
            2,
            2,
            CHECKBOX_SIZE,
            CheckboxMark::Progress,
            checkbox_mark_color(),
        );
        draw_mark(
            &mut blocked,
            2,
            2,
            CHECKBOX_SIZE,
            CheckboxMark::Blocked,
            checkbox_mark_color(),
        );

        assert!(count_color(&progress, checkbox_mark_color()) > 0);
        assert!(count_color(&blocked, checkbox_mark_color()) > 0);
        assert_ne!(progress.pixels(), blocked.pixels());
    }

    #[test]
    fn empty_task_checkbox_keeps_empty_box() {
        let mut canvas = Canvas::new(24, 24, BACKGROUND);
        draw_box(&mut canvas, 2, 2, CHECKBOX_SIZE, false, palette());

        assert_eq!(0, count_color(&canvas, SELECTION));
        assert!(count_color(&canvas, BORDER) > 0);
        assert!(count_color(&canvas, BACKGROUND) >= CHECKBOX_SIZE);
    }

    #[test]
    fn explicit_checkbox_height_keeps_katana_icon_box_size() {
        let mut canvas = Canvas::new(40, 40, BACKGROUND);
        let node =
            UiNode::from(Checkbox::new("").checked(true).value("[x]")).height(UiDimension::px(32));
        let mut y = 0;

        super::UiTreeCheckboxRenderer::draw(
            &mut canvas,
            &crate::raster_host::text::TextRenderer::load(&UiCoreFacade::default(), "body"),
            &node,
            2,
            &mut y,
            palette(),
        );

        assert_eq!(32, y);
        assert!(count_color(&canvas, SELECTION) <= CHECKBOX_SIZE * CHECKBOX_SIZE);
        assert!(count_color(&canvas, 0xffffff) > 15);
    }

    fn checkbox(value: &str, checked: bool) -> UiNode {
        UiNode::from(Checkbox::new("").checked(checked).value(value))
    }

    fn count_color(canvas: &Canvas, color: u32) -> usize {
        canvas
            .pixels()
            .iter()
            .filter(|pixel| **pixel == color)
            .count()
    }

    fn palette() -> UiTreeCanvasPalette {
        UiTreeCanvasPalette {
            visual: VisualPalette {
                background: BACKGROUND,
                surface: BACKGROUND,
                panel: BACKGROUND,
                code_background: BACKGROUND,
                border: BORDER,
                hover_border: 0x569cd6,
                text: 0xd4d4d4,
                muted: 0x8e8e8e,
                accent: 0x569cd6,
                accent_foreground: 0xffffff,
                selection: SELECTION,
            },
            background: BACKGROUND,
            preview_background: BACKGROUND,
            selection: SELECTION,
            text: 0xd4d4d4,
            link: 0x4da3ff,
            code_background: BACKGROUND,
            inline_code_background: BACKGROUND,
            table_background: BACKGROUND,
            table_header_background: BACKGROUND,
            table_even_row_background: BACKGROUND,
            alert_background: BACKGROUND,
            alert_note_accent: 0x0969da,
            alert_tip_accent: 0x1a7f37,
            alert_important_accent: 0x8250df,
            alert_warning_accent: 0xbf8700,
            alert_caution_accent: 0xd1242f,
            quote_background: BACKGROUND,
            footnote_background: BACKGROUND,
            document_rule_border: BORDER,
            danger_accent: 0xe05252,
            muted_border: BORDER,
            pending_background: BACKGROUND,
            hover_background: BACKGROUND,
        }
    }
}
