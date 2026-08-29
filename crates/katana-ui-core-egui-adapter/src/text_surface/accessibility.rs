use super::artifact_model::EguiTextSurfaceFrameRecord;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurfaceAccessibilityNode, TextSurfaceAccessibilityTarget, TextSurfaceLayout,
};

pub(super) fn publish_accesskit(
    ui: &mut egui::Ui,
    root_id: egui::Id,
    record: &EguiTextSurfaceFrameRecord,
    layout: &TextSurfaceLayout,
    placeholder: &str,
    single_line: bool,
) {
    let runs = text_runs(root_id, layout);
    let mut auxiliary = accessibility_nodes(root_id, &record.frame.accessibility);
    ui.ctx().accesskit_node_builder(root_id, |node| {
        let root = &record.frame.accessibility.root;
        node.set_role(if single_line {
            egui::accesskit::Role::TextInput
        } else {
            egui::accesskit::Role::MultilineTextInput
        });
        node.set_label(root.label.as_str());
        node.set_value(layout.text());
        if !placeholder.is_empty() {
            node.set_placeholder(placeholder);
        }
        node.set_bounds(bounds(root.bounds));
        node.add_action(egui::accesskit::Action::Focus);
        node.add_action(egui::accesskit::Action::SetTextSelection);
        node.add_action(egui::accesskit::Action::ShowContextMenu);
        if root.readonly {
            node.set_read_only();
        }
        if root.disabled {
            node.set_disabled();
        }
        if let Some(reason) = root.disabled_reason.as_deref() {
            node.set_description(reason);
        }
        let selection = text_selection(&runs, record.frame.selection.range);
        node.set_text_selection(selection);
    });
    let child_id = root_id.with("kuc-text-surface-accessibility-children");
    let child_ui = ui.new_child(
        egui::UiBuilder::new()
            .id(child_id)
            .accessibility_parent(root_id)
            .max_rect(egui_rect(record.frame.surface_bounds)),
    );
    for run in runs {
        let _ = child_ui.interact(egui_rect(run.bounds), run.id, egui::Sense::hover());
        child_ui.ctx().accesskit_node_builder(run.id, |node| {
            node.set_role(egui::accesskit::Role::TextRun);
            node.set_text_direction(egui::accesskit::TextDirection::LeftToRight);
            node.set_value(run.value);
            node.set_bounds(bounds(run.bounds));
            node.set_character_lengths(run.character_lengths);
            node.set_character_positions(run.character_positions);
            node.set_character_widths(run.character_widths);
        });
    }
    for node in auxiliary.drain(..) {
        let _ = child_ui.interact(egui_rect(node.source.bounds), node.id, egui::Sense::hover());
        child_ui.ctx().accesskit_node_builder(node.id, |builder| {
            builder.set_role(egui::accesskit::Role::Button);
            builder.set_label(node.source.label.as_str());
            builder.set_bounds(bounds(node.source.bounds));
            builder.add_action(egui::accesskit::Action::Click);
            if node.source.disabled {
                builder.set_disabled();
            }
            if let Some(description) = node.source.description.as_deref() {
                builder.set_description(description);
            } else if let Some(reason) = node.source.disabled_reason.as_deref() {
                builder.set_description(reason);
            }
        });
    }
}

struct TextRun {
    id: egui::Id,
    grapheme_start: usize,
    grapheme_end: usize,
    character_offsets: Vec<usize>,
    value: String,
    bounds: UiRect,
    character_lengths: Vec<u8>,
    character_positions: Vec<f32>,
    character_widths: Vec<f32>,
}

struct AuxiliaryNode<'a> {
    id: egui::Id,
    source: &'a TextSurfaceAccessibilityNode,
}

fn text_runs(root_id: egui::Id, layout: &TextSurfaceLayout) -> Vec<TextRun> {
    layout
        .lines
        .iter()
        .filter_map(|line| {
            let graphemes = layout
                .graphemes
                .iter()
                .filter(|grapheme| grapheme.bounds.y == line.bounds.y)
                .collect::<Vec<_>>();
            let first = graphemes.first()?;
            let last = graphemes.last()?;
            let mut value = String::new();
            let mut character_lengths = Vec::new();
            let mut character_positions = Vec::new();
            let mut character_widths = Vec::new();
            let mut character_offsets = vec![0];
            for grapheme in &graphemes {
                let segment = layout.text().get(grapheme.byte_start..grapheme.byte_end)?;
                let mut first_character = true;
                for character in segment.chars() {
                    value.push(character);
                    character_lengths.push(character.len_utf8() as u8);
                    character_positions.push((grapheme.bounds.x - line.bounds.x) as f32);
                    character_widths.push(if first_character {
                        grapheme.bounds.width as f32
                    } else {
                        0.0
                    });
                    first_character = false;
                }
                character_offsets.push(character_lengths.len());
            }
            (!value.is_empty()).then(|| TextRun {
                id: root_id.with(("text-run", line.logical_row)),
                grapheme_start: first.grapheme_index,
                grapheme_end: last.grapheme_index.saturating_add(1),
                character_offsets,
                value,
                bounds: line.bounds,
                character_lengths,
                character_positions,
                character_widths,
            })
        })
        .collect()
}

fn accessibility_nodes<'a>(
    root_id: egui::Id,
    tree: &'a katana_ui_core::text_surface::TextSurfaceAccessibilityTree,
) -> Vec<AuxiliaryNode<'a>> {
    tree.gutter_targets
        .iter()
        .chain(tree.context_target.iter())
        .map(|source| AuxiliaryNode {
            id: root_id.with(accessibility_target_key(&source.target)),
            source,
        })
        .collect()
}

fn text_selection(
    runs: &[TextRun],
    selection: katana_ui_core::text_selection::UiTextSelectionRange,
) -> egui::accesskit::TextSelection {
    egui::accesskit::TextSelection {
        anchor: text_position(runs, selection.anchor),
        focus: text_position(runs, selection.focus),
    }
}

fn text_position(runs: &[TextRun], grapheme_index: usize) -> egui::accesskit::TextPosition {
    let run = runs
        .iter()
        .find(|run| grapheme_index >= run.grapheme_start && grapheme_index <= run.grapheme_end)
        .or_else(|| runs.last());
    let Some(run) = run else {
        return egui::accesskit::TextPosition {
            node: egui::Id::NULL.accesskit_id(),
            character_index: 0,
        };
    };
    let grapheme_offset = grapheme_index
        .saturating_sub(run.grapheme_start)
        .min(run.character_offsets.len().saturating_sub(1));
    let character_index = run.character_offsets[grapheme_offset];
    egui::accesskit::TextPosition {
        node: run.id.accesskit_id(),
        character_index,
    }
}

fn accessibility_target_key(target: &TextSurfaceAccessibilityTarget) -> String {
    match target {
        TextSurfaceAccessibilityTarget::Surface => "surface".to_string(),
        TextSurfaceAccessibilityTarget::GutterRow { logical_row } => {
            format!("gutter-row:{logical_row}")
        }
        TextSurfaceAccessibilityTarget::GutterMarker {
            logical_row,
            marker_id,
        } => format!("gutter-marker:{logical_row}:{marker_id}"),
        TextSurfaceAccessibilityTarget::ContextSelection => "context-selection".to_string(),
    }
}

fn bounds(value: UiRect) -> egui::accesskit::Rect {
    egui::accesskit::Rect {
        x0: value.x.into(),
        y0: value.y.into(),
        x1: value.x.saturating_add(value.width as i32).into(),
        y1: value.y.saturating_add(value.height as i32).into(),
    }
}

fn egui_rect(value: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(value.x as f32, value.y as f32),
        egui::vec2(value.width as f32, value.height as f32),
    )
}
