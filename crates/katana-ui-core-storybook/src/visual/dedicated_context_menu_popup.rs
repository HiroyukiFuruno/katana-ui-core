use super::canvas::Canvas;
use super::dedicated_context_menu_labels as labels;
use super::dedicated_context_menu_metrics as cm;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::text::TextRenderer;

#[derive(Clone, Copy)]
struct MenuRowSpec<'a> {
    index: usize,
    label: &'a str,
    shortcut: &'a str,
    active: bool,
    destructive: bool,
}

pub(super) fn draw_menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    preset_index: usize,
    x: usize,
    y: usize,
) {
    let menu = Rect::new(
        x + cm::MENU_X,
        y + cm::MENU_Y,
        cm::MENU_WIDTH,
        cm::MENU_HEIGHT,
    );
    common::fill(canvas, menu, palette.panel);
    common::outline(canvas, palette, menu);
    draw_rows(canvas, text, palette, preset_index, x, y);
}

fn draw_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    preset_index: usize,
    x: usize,
    y: usize,
) {
    draw_row(canvas, text, palette, x, y, MenuRowSpec::edit());
    draw_row(canvas, text, palette, x, y, MenuRowSpec::copy());
    draw_row(canvas, text, palette, x, y, MenuRowSpec::insert());
    draw_divider(canvas, palette, x, y, cm::ROW_DIVIDER);
    draw_row(
        canvas,
        text,
        palette,
        x,
        y,
        MenuRowSpec::preset(preset_index),
    );
}

fn draw_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    spec: MenuRowSpec<'_>,
) {
    let row_y = y + cm::MENU_Y + cm::MENU_ROW_TOP_OFFSET + spec.index * cm::MENU_ROW_HEIGHT;
    let fill = if spec.active {
        palette.accent
    } else {
        palette.panel
    };
    let color = if spec.active {
        palette.background
    } else {
        palette.text
    };
    draw_row_fill(canvas, fill, x, row_y);
    draw_row_icon(canvas, palette, x, row_y);
    draw_row_text(canvas, text, x, row_y, spec, color);
}

fn draw_row_fill(canvas: &mut Canvas, fill: u32, x: usize, row_y: usize) {
    canvas.fill_rect(
        x + cm::MENU_X + cm::MENU_ROW_FILL_X_OFFSET,
        row_y,
        cm::MENU_WIDTH - cm::MENU_ROW_FILL_WIDTH_INSET,
        cm::MENU_ROW_HEIGHT,
        fill,
    );
}

fn draw_row_icon(canvas: &mut Canvas, palette: &VisualPalette, x: usize, row_y: usize) {
    canvas.stroke_rect(
        x + cm::MENU_X + cm::MENU_ROW_ICON_X_OFFSET,
        row_y + cm::MENU_ROW_ICON_Y_OFFSET,
        cm::ICON_SIZE,
        cm::ICON_SIZE,
        palette.border,
    );
}

fn draw_row_text(
    canvas: &mut Canvas,
    text: &TextRenderer,
    x: usize,
    row_y: usize,
    spec: MenuRowSpec<'_>,
    color: u32,
) {
    text.draw(
        canvas,
        spec.label,
        x + cm::MENU_X + cm::MENU_ROW_LABEL_X_OFFSET,
        row_y + cm::MENU_ROW_ICON_Y_OFFSET,
        m::FONT_8,
        if spec.destructive {
            common::DANGER
        } else {
            color
        },
    );
    text.draw(
        canvas,
        spec.shortcut,
        x + cm::MENU_X + cm::MENU_ROW_SHORTCUT_X_OFFSET,
        row_y + cm::MENU_ROW_ICON_Y_OFFSET,
        m::FONT_7,
        color,
    );
}

impl<'a> MenuRowSpec<'a> {
    const fn new(
        index: usize,
        label: &'a str,
        shortcut: &'a str,
        active: bool,
        destructive: bool,
    ) -> Self {
        Self {
            index,
            label,
            shortcut,
            active,
            destructive,
        }
    }

    const fn edit() -> Self {
        Self::new(cm::ROW_EDIT, "編集", "", false, false)
    }

    const fn copy() -> Self {
        Self::new(cm::ROW_COPY, "Copy", "Cmd+C", true, false)
    }

    const fn insert() -> Self {
        Self::new(cm::ROW_INSERT, "Insert", ">", false, false)
    }

    fn preset(preset_index: usize) -> Self {
        Self::new(
            cm::ROW_PRESET,
            labels::preset_row_label(preset_index),
            labels::preset_shortcut(preset_index),
            false,
            preset_index == cm::PRESET_ICON_SHORTCUT,
        )
    }
}

fn draw_divider(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize, index: usize) {
    let line_y = y + cm::MENU_Y + cm::DIVIDER_Y_OFFSET + index * cm::MENU_ROW_HEIGHT;
    common::fill(
        canvas,
        Rect::new(
            x + cm::MENU_X + cm::DIVIDER_X_OFFSET,
            line_y,
            cm::MENU_WIDTH - cm::DIVIDER_WIDTH_INSET,
            cm::DIVIDER_HEIGHT,
        ),
        palette.border,
    );
}
