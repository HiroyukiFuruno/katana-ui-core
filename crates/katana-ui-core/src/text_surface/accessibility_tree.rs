use super::accessibility::{
    TextSurfaceAccessibilityAction, TextSurfaceAccessibilityActionKind,
    TextSurfaceAccessibilityLabels, TextSurfaceAccessibilityNode, TextSurfaceAccessibilityTarget,
    TextSurfaceAccessibilityTree,
};
use super::frame::gutter_frames;
use super::gutter::TextSurfaceGutter;
use super::layout_model::TextSurfaceLayout;
use crate::accessibility::{AccessibilityLabel, AccessibilityRole};
use crate::atom::TextAreaState;
use crate::render_model::UiRect;
use crate::text_selection::UiTextSelectionRange;

pub(super) struct TextSurfaceAccessibilityTreeInput<'a> {
    pub(super) layout: &'a TextSurfaceLayout,
    pub(super) state: &'a TextAreaState,
    pub(super) label: String,
    pub(super) labels: &'a TextSurfaceAccessibilityLabels,
    pub(super) context_target_label: Option<&'a str>,
    pub(super) disabled_reason: Option<String>,
    pub(super) gutter: Option<&'a TextSurfaceGutter>,
    pub(super) gutter_width: u32,
    pub(super) surface_bounds: UiRect,
    pub(super) active_row: Option<usize>,
    pub(super) hovered_rows: Vec<usize>,
}

pub(super) fn accessibility_tree(
    input: TextSurfaceAccessibilityTreeInput<'_>,
) -> TextSurfaceAccessibilityTree {
    let TextSurfaceAccessibilityTreeInput {
        layout,
        state,
        label,
        labels,
        context_target_label,
        disabled_reason,
        gutter,
        gutter_width,
        surface_bounds,
        active_row,
        hovered_rows,
    } = input;
    let selection =
        layout.grapheme_range_for_byte_offsets(state.selection.start, state.selection.end);
    let root = accessibility_node(AccessibilityNodeInput {
        target: TextSurfaceAccessibilityTarget::Surface,
        role: AccessibilityRole::Input,
        label,
        bounds: surface_bounds,
        active: false,
        hovered: false,
        state,
        disabled_reason: disabled_reason.clone(),
        selection: Some(selection),
        description: None,
    });
    let gutter_targets = gutter_accessibility_targets(
        layout,
        gutter,
        gutter_width,
        active_row,
        hovered_rows,
        state,
        disabled_reason.clone(),
    );
    let context_target = context_target_label
        .filter(|label| !label.is_empty())
        .map(|label| {
            accessibility_node(AccessibilityNodeInput {
                target: TextSurfaceAccessibilityTarget::ContextSelection,
                role: AccessibilityRole::Button,
                label: label.to_string(),
                bounds: surface_bounds,
                active: false,
                hovered: false,
                state,
                disabled_reason: disabled_reason.clone(),
                selection: Some(selection),
                description: None,
            })
        });
    TextSurfaceAccessibilityTree {
        root,
        gutter_targets,
        context_target,
        actions: accessibility_actions(labels, state),
    }
}

fn gutter_accessibility_targets(
    layout: &TextSurfaceLayout,
    gutter: Option<&TextSurfaceGutter>,
    gutter_width: u32,
    active_row: Option<usize>,
    hovered_rows: Vec<usize>,
    state: &TextAreaState,
    disabled_reason: Option<String>,
) -> Vec<TextSurfaceAccessibilityNode> {
    gutter_frames(layout, gutter, gutter_width, active_row, hovered_rows)
        .into_iter()
        .flat_map(|frame| {
            let label = if frame.accessibility_label.is_empty() {
                frame.display_label.clone()
            } else {
                frame.accessibility_label.clone()
            };
            let row = accessibility_node(AccessibilityNodeInput {
                target: TextSurfaceAccessibilityTarget::GutterRow {
                    logical_row: frame.logical_row,
                },
                role: AccessibilityRole::Button,
                label: label.clone(),
                bounds: frame.bounds,
                active: frame.active,
                hovered: frame.hovered,
                state,
                disabled_reason: disabled_reason.clone(),
                selection: None,
                description: frame.accessibility_description.clone(),
            });
            let marker = frame.marker_id.map(|marker_id| {
                let bounds = frame.marker_bounds.unwrap_or(frame.bounds);
                accessibility_node(AccessibilityNodeInput {
                    target: TextSurfaceAccessibilityTarget::GutterMarker {
                        logical_row: frame.logical_row,
                        marker_id,
                    },
                    role: AccessibilityRole::Button,
                    label,
                    bounds,
                    active: frame.active,
                    hovered: frame.hovered,
                    state,
                    disabled_reason: disabled_reason.clone(),
                    selection: None,
                    description: frame.accessibility_description,
                })
            });
            std::iter::once(row).chain(marker).collect::<Vec<_>>()
        })
        .collect()
}

struct AccessibilityNodeInput<'a> {
    target: TextSurfaceAccessibilityTarget,
    role: AccessibilityRole,
    label: String,
    bounds: UiRect,
    active: bool,
    hovered: bool,
    state: &'a TextAreaState,
    disabled_reason: Option<String>,
    selection: Option<UiTextSelectionRange>,
    description: Option<String>,
}

fn accessibility_node(input: AccessibilityNodeInput<'_>) -> TextSurfaceAccessibilityNode {
    let AccessibilityNodeInput {
        target,
        role,
        label,
        bounds,
        active,
        hovered,
        state,
        disabled_reason,
        selection,
        description,
    } = input;
    TextSurfaceAccessibilityNode {
        target,
        role,
        label: AccessibilityLabel::new(label),
        bounds,
        active,
        hovered,
        focused: state.focused,
        editable: !state.readonly && !state.disabled,
        readonly: state.readonly,
        disabled: state.disabled,
        disabled_reason: state.disabled.then_some(disabled_reason).flatten(),
        description,
        selection,
    }
}

fn accessibility_actions(
    labels: &TextSurfaceAccessibilityLabels,
    state: &TextAreaState,
) -> Vec<TextSurfaceAccessibilityAction> {
    [
        TextSurfaceAccessibilityActionKind::Copy,
        TextSurfaceAccessibilityActionKind::Cut,
        TextSurfaceAccessibilityActionKind::Paste,
        TextSurfaceAccessibilityActionKind::Undo,
        TextSurfaceAccessibilityActionKind::Redo,
    ]
    .into_iter()
    .filter_map(|kind| {
        labels
            .label_for(kind)
            .cloned()
            .map(|label| TextSurfaceAccessibilityAction {
                kind,
                label,
                enabled: accessibility_action_enabled(kind, state),
            })
    })
    .collect()
}

fn accessibility_action_enabled(
    kind: TextSurfaceAccessibilityActionKind,
    state: &TextAreaState,
) -> bool {
    let has_selection = state.selection.start != state.selection.end;
    match kind {
        TextSurfaceAccessibilityActionKind::Copy => !state.disabled && has_selection,
        TextSurfaceAccessibilityActionKind::Cut => {
            !state.disabled && !state.readonly && has_selection
        }
        TextSurfaceAccessibilityActionKind::Paste
        | TextSurfaceAccessibilityActionKind::Undo
        | TextSurfaceAccessibilityActionKind::Redo => !state.disabled && !state.readonly,
    }
}
