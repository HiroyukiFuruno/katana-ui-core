use super::accessibility::TextSurfaceAccessibilityLabels;
use super::accessibility_tree::{TextSurfaceAccessibilityTreeInput, accessibility_tree};
use super::annotation::TextSurfaceAnnotation;
use super::frame_record::{
    TextSurfaceAnnotationFrame, TextSurfaceFrameRecord, TextSurfaceGutterFrame,
    TextSurfacePreeditFrame, TextSurfaceSelectionFrame,
};
use super::gutter::TextSurfaceGutter;
use super::layout_model::TextSurfaceLayout;
use super::props::TextSurfaceViewport;
use super::state::TextSurfaceState;
use crate::atom::TextAreaState;
use crate::render_model::{UiIconProps, UiRect};
use crate::text_selection::UiTextSelectionRange;

const MAXIMUM_GUTTER_MARKER_EXTENT_PX: u32 = 16;

pub(crate) struct TextSurfaceFrameInput<'a> {
    pub(crate) layout: &'a TextSurfaceLayout,
    pub(crate) viewport: TextSurfaceViewport,
    pub(crate) state: &'a TextSurfaceState,
    pub(crate) label: String,
    pub(crate) accessibility_actions: &'a TextSurfaceAccessibilityLabels,
    pub(crate) context_target_label: Option<&'a str>,
    pub(crate) disabled_reason: Option<String>,
    pub(crate) annotations: &'a [TextSurfaceAnnotation],
    pub(crate) gutter: Option<&'a TextSurfaceGutter>,
    pub(crate) gutter_width: u32,
    pub(crate) surface_bounds: UiRect,
    pub(crate) viewport_bounds: UiRect,
}

impl TextSurfaceFrameRecord {
    #[must_use]
    pub(crate) fn new(input: TextSurfaceFrameInput<'_>) -> Self {
        let TextSurfaceFrameInput {
            layout,
            viewport,
            state,
            label,
            accessibility_actions,
            context_target_label,
            disabled_reason,
            annotations,
            gutter,
            gutter_width,
            surface_bounds,
            viewport_bounds,
        } = input;
        let active_row = layout.logical_row_for_byte_offset(state.text_area.caret);
        let hovered_rows = resolved_hovered_rows(layout, gutter);
        Self {
            layout_identity: layout.identity.clone(),
            content_bounds: layout.content_bounds,
            surface_bounds,
            viewport_bounds,
            viewport,
            visible_logical_rows: layout.visible_logical_rows(viewport_bounds),
            caret: state.text_area.caret,
            selection_start: state.text_area.selection.start,
            selection_end: state.text_area.selection.end,
            selection: selection_frame(layout, &state.text_area),
            preedit: preedit_frame(layout, &state.text_area),
            annotations: annotation_frames(layout, annotations),
            gutter: gutter_frames(
                layout,
                gutter,
                gutter_width,
                active_row,
                hovered_rows.clone(),
            ),
            accessibility: accessibility_tree(TextSurfaceAccessibilityTreeInput {
                layout,
                state: &state.text_area,
                label,
                labels: accessibility_actions,
                context_target_label,
                disabled_reason,
                gutter,
                gutter_width,
                surface_bounds,
                active_row,
                hovered_rows,
            }),
        }
    }
}

pub(super) fn gutter_frames(
    layout: &TextSurfaceLayout,
    gutter: Option<&TextSurfaceGutter>,
    gutter_width: u32,
    active_row: Option<usize>,
    hovered_rows: Vec<usize>,
) -> Vec<TextSurfaceGutterFrame> {
    let Some(gutter) = gutter else {
        return Vec::new();
    };
    gutter
        .resolved_rows(layout)
        .iter()
        .filter_map(|row| {
            let line = layout
                .lines
                .iter()
                .find(|line| line.logical_row == row.logical_row)?;
            Some(TextSurfaceGutterFrame {
                row_id: super::gutter::TextSurfaceGutterRowId::for_logical_row(row.logical_row),
                logical_row: row.logical_row,
                active: active_row == Some(row.logical_row),
                hovered: hovered_rows.contains(&row.logical_row),
                display_label: row.display_label.clone(),
                marker_id: row.marker_id.clone(),
                accessibility_label: row.accessibility_label.clone(),
                accessibility_description: row.accessibility_description.clone(),
                visual_role: row.visual_role.clone(),
                icon: row.icon.clone(),
                marker_bounds: marker_bounds(
                    row.icon.as_ref(),
                    UiRect::new(
                        layout.content_bounds.x.saturating_sub(gutter_width as i32),
                        line.bounds.y,
                        gutter_width,
                        line.bounds.height,
                    ),
                ),
                bounds: UiRect::new(
                    layout.content_bounds.x.saturating_sub(gutter_width as i32),
                    line.bounds.y,
                    gutter_width,
                    line.bounds.height,
                ),
            })
        })
        .collect()
}

fn resolved_hovered_rows(
    layout: &TextSurfaceLayout,
    gutter: Option<&TextSurfaceGutter>,
) -> Vec<usize> {
    let Some(gutter) = gutter else {
        return Vec::new();
    };
    let Some(controlled) = &gutter.controlled_automatic else {
        return Vec::new();
    };
    let mut rows: Vec<usize> = controlled
        .hovered_rows
        .iter()
        .copied()
        .filter(|row| layout.has_logical_row(*row))
        .collect();
    rows.sort_unstable();
    rows.dedup();
    rows
}

fn marker_bounds(icon: Option<&UiIconProps>, row_bounds: UiRect) -> Option<UiRect> {
    icon.map(|_| {
        let extent = row_bounds
            .height
            .min(row_bounds.width)
            .clamp(1, MAXIMUM_GUTTER_MARKER_EXTENT_PX);
        UiRect::new(
            row_bounds
                .x
                .saturating_add_unsigned(row_bounds.width.saturating_sub(extent)),
            row_bounds
                .y
                .saturating_add_unsigned(row_bounds.height.saturating_sub(extent) / 2),
            extent,
            extent,
        )
    })
}

fn annotation_frames(
    layout: &TextSurfaceLayout,
    annotations: &[TextSurfaceAnnotation],
) -> Vec<TextSurfaceAnnotationFrame> {
    let mut indexed = annotations.iter().enumerate().collect::<Vec<_>>();
    indexed.sort_by(|(left_index, left), (right_index, right)| {
        right
            .priority
            .cmp(&left.priority)
            .then_with(|| left_index.cmp(right_index))
    });
    indexed
        .into_iter()
        .map(|(_, annotation)| TextSurfaceAnnotationFrame {
            id: annotation.id.clone(),
            visual_role: annotation.visual_role.clone(),
            style: annotation.style,
            priority: annotation.priority,
            tooltip: annotation.tooltip.clone(),
            rects: layout.selection_rects(annotation.range),
        })
        .collect()
}

fn preedit_frame(
    layout: &TextSurfaceLayout,
    state: &TextAreaState,
) -> Option<TextSurfacePreeditFrame> {
    let composition = state.composition.as_ref()?;
    let layout_composition = layout.composition_model()?;
    (layout_composition.source_start == state.selection.start
        && layout_composition.source_end == state.selection.end
        && layout_composition.preedit == composition.preedit)
        .then(|| TextSurfacePreeditFrame {
            text: layout_composition.preedit.clone(),
            range: layout_composition.preedit_range,
            rects: layout.selection_rects(layout_composition.preedit_range),
            caret: layout.caret_rect(layout_composition.caret_range),
        })
}

fn selection_frame(layout: &TextSurfaceLayout, state: &TextAreaState) -> TextSurfaceSelectionFrame {
    let range = layout.grapheme_range_for_byte_offsets(state.selection.start, state.selection.end);
    TextSurfaceSelectionFrame {
        range,
        rects: layout.selection_rects(range),
        caret: layout.caret_rect(UiTextSelectionRange::caret(range.caret_position())),
    }
}
