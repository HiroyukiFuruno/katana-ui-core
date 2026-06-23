use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_dod_molecule_tree_parts as parts;
use super::palette::VisualPalette;
use katana_ui_core::render_model::{UiTreeLineStyle, UiTreeNodeProps};

const DOTTED_SEGMENT_LENGTH: usize = 4;
const DOTTED_SEGMENT_GAP: usize = 6;
const DASHED_SEGMENT_LENGTH: usize = 10;
const DASHED_SEGMENT_GAP: usize = 6;

#[derive(Clone, Copy)]
pub(super) struct TreeRowLayout {
    pub(super) index: usize,
    pub(super) visual_index: usize,
    pub(super) x: usize,
    pub(super) y: usize,
}

#[derive(Clone, Copy)]
pub(super) struct TreeLineOptions<'a> {
    pub(super) rows: &'a [UiTreeNodeProps],
    pub(super) style: UiTreeLineStyle,
    pub(super) width: usize,
    pub(super) visible: bool,
    pub(super) icons_visible: bool,
}

#[derive(Clone, Copy)]
pub(super) struct TreeGuideLayout<'a> {
    pub(super) node: &'a UiTreeNodeProps,
    pub(super) rows: &'a [UiTreeNodeProps],
    pub(super) row_index: usize,
    pub(super) row_center_y: usize,
    pub(super) row_y: usize,
    pub(super) x: usize,
    pub(super) style: UiTreeLineStyle,
    pub(super) width: usize,
    pub(super) draw_horizontal_connector: bool,
}

pub(super) fn draw_indent_guides(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    layout: TreeGuideLayout<'_>,
) {
    let node = layout.node;
    let row_center_x =
        layout.x + parts::DISCLOSURE_X + m::PX_4 + m::PX_1 + node.depth * parts::INDENT_STEP;
    let previous_depth = layout
        .row_index
        .checked_sub(1)
        .and_then(|index| layout.rows.get(index).map(|node| node.depth));
    let next_depth = layout.rows.get(layout.row_index + 1).map(|node| node.depth);

    for level in 0..=node.depth {
        draw_tree_level(
            canvas,
            palette,
            &layout,
            row_center_x,
            previous_depth,
            next_depth,
            level,
        );
    }
    if layout.draw_horizontal_connector {
        draw_styled_line(
            canvas,
            TreeLineSpec {
                x: row_center_x - (parts::INDENT_STEP - m::PX_4),
                y: layout.row_center_y,
                length: parts::INDENT_STEP - m::PX_4,
                width: layout.width,
                style: layout.style,
                vertical: false,
                color: palette.border,
            },
        );
    }
}

fn draw_tree_level(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    layout: &TreeGuideLayout<'_>,
    row_center_x: usize,
    previous_depth: Option<usize>,
    next_depth: Option<usize>,
    level: usize,
) {
    let node = layout.node;
    let level_center_x =
        row_center_x - node.depth * parts::INDENT_STEP + level * parts::INDENT_STEP;
    let has_up = node.depth > level && previous_depth.is_some_and(|depth| depth >= level);
    let has_down = next_depth.is_some_and(|depth| depth > level);
    if !has_up && !has_down {
        return;
    }

    let start_y = if has_up {
        layout.row_y.saturating_sub(m::PX_2)
    } else {
        layout.row_center_y
    };
    let end_y = if has_down {
        layout.row_y + parts::ROW_HEIGHT + m::PX_2
    } else {
        layout.row_center_y + 1
    };
    draw_styled_line(
        canvas,
        TreeLineSpec {
            x: level_center_x,
            y: start_y,
            length: end_y.saturating_sub(start_y),
            width: layout.width,
            style: layout.style,
            vertical: true,
            color: palette.border,
        },
    );
}

#[derive(Clone, Copy)]
struct TreeLineSpec {
    x: usize,
    y: usize,
    length: usize,
    width: usize,
    style: UiTreeLineStyle,
    vertical: bool,
    color: u32,
}

fn draw_styled_line(canvas: &mut Canvas, spec: TreeLineSpec) {
    match spec.style {
        UiTreeLineStyle::Solid => draw_segment(canvas, TreeSegmentSpec::solid(spec)),
        UiTreeLineStyle::Dotted => draw_segment(
            canvas,
            TreeSegmentSpec::patterned(spec, DOTTED_SEGMENT_LENGTH, DOTTED_SEGMENT_GAP),
        ),
        UiTreeLineStyle::Dashed => draw_segment(
            canvas,
            TreeSegmentSpec::patterned(spec, DASHED_SEGMENT_LENGTH, DASHED_SEGMENT_GAP),
        ),
    }
}

#[derive(Clone, Copy)]
struct TreeSegmentSpec {
    x: usize,
    y: usize,
    length: usize,
    width: usize,
    on_length: usize,
    off_length: usize,
    vertical: bool,
    color: u32,
}

impl TreeSegmentSpec {
    const fn solid(line: TreeLineSpec) -> Self {
        Self::patterned(line, 0, line.width)
    }

    const fn patterned(line: TreeLineSpec, on_length: usize, off_length: usize) -> Self {
        Self {
            x: line.x,
            y: line.y,
            length: line.length,
            width: line.width,
            on_length,
            off_length,
            vertical: line.vertical,
            color: line.color,
        }
    }
}

fn draw_segment(canvas: &mut Canvas, spec: TreeSegmentSpec) {
    if spec.on_length == 0 {
        draw_solid_segment(canvas, spec);
        return;
    }

    let pitch = spec.on_length + spec.off_length;
    for offset in (0..spec.length).step_by(pitch) {
        let segment_length = spec.on_length.min(spec.length.saturating_sub(offset));
        draw_segment_part(canvas, spec, offset, segment_length);
    }
    draw_segment_tail(canvas, spec, pitch);
}

fn draw_solid_segment(canvas: &mut Canvas, spec: TreeSegmentSpec) {
    if spec.vertical {
        common::fill(
            canvas,
            Rect::new(spec.x, spec.y, spec.width, spec.length),
            spec.color,
        );
        return;
    }
    common::fill(
        canvas,
        Rect::new(spec.x, spec.y, spec.length, spec.width),
        spec.color,
    );
}

fn draw_segment_part(canvas: &mut Canvas, spec: TreeSegmentSpec, offset: usize, length: usize) {
    if spec.vertical {
        common::fill(
            canvas,
            Rect::new(spec.x, spec.y + offset, spec.width, length),
            spec.color,
        );
        return;
    }
    common::fill(
        canvas,
        Rect::new(spec.x + offset, spec.y, length, spec.width),
        spec.color,
    );
}

fn draw_segment_tail(canvas: &mut Canvas, spec: TreeSegmentSpec, pitch: usize) {
    if spec.length == 0 || spec.on_length == 0 {
        return;
    }
    let tail_offset = (spec.length / pitch) * pitch;
    let tail = spec.length.saturating_sub(tail_offset);
    if tail_offset < spec.length && tail > 0 && tail <= spec.on_length {
        draw_segment_part(canvas, spec, tail_offset, tail);
    }
}
