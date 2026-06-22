use super::{SettingsList, SettingsListAction, SettingsListLayoutMetrics};
use crate::render_model::UiCursor;

#[path = "hit_test_types.rs"]
mod hit_test_types;

use hit_test_types::action_for_result;
pub use hit_test_types::{
    SettingsListHitRect, SettingsListHitTarget, SettingsListHitTestInput,
    SettingsListHitTestResult, SettingsListInteraction,
};

pub(super) fn hit_test(
    list: &SettingsList,
    input: SettingsListHitTestInput,
) -> SettingsListHitTestResult {
    unbounded_interaction_for_hit(list, input).result
}

pub(super) fn cursor_for_hit(list: &SettingsList, input: SettingsListHitTestInput) -> UiCursor {
    unbounded_interaction_for_hit(list, input).cursor
}

pub(super) fn action_for_hit(
    list: &SettingsList,
    input: SettingsListHitTestInput,
) -> Option<SettingsListAction> {
    unbounded_interaction_for_hit(list, input).action
}

pub(super) fn hit_targets(list: &SettingsList, viewport_width: u32) -> Vec<SettingsListHitTarget> {
    let metrics = list.layout_metrics();
    let mut targets = Vec::new();
    let mut row_top = 0;
    for row in settings_rows(list, metrics) {
        if let Some(target) = row.target(list, row_top, viewport_width, metrics) {
            targets.push(target);
        }
        row_top += row.height;
    }
    targets
}

pub(super) fn hit_target_for_field(
    list: &SettingsList,
    field_id: &str,
    viewport_width: u32,
) -> Option<SettingsListHitTarget> {
    hit_target_for_named_row(
        list,
        viewport_width,
        |result| matches!(result, SettingsListHitTestResult::Field { field_id: id } if id == field_id),
    )
}

pub(super) fn hit_target_for_section(
    list: &SettingsList,
    section_id: &str,
    viewport_width: u32,
) -> Option<SettingsListHitTarget> {
    hit_target_for_named_row(list, viewport_width, |result| {
        matches!(
            result,
            SettingsListHitTestResult::ToggleSection { section_id: id } if id == section_id
        )
    })
}

pub(super) fn hit_target_for_hit(
    list: &SettingsList,
    input: SettingsListHitTestInput,
    viewport_width: u32,
) -> Option<SettingsListHitTarget> {
    interaction_for_hit(list, input, viewport_width).target
}

pub(super) fn interaction_for_hit(
    list: &SettingsList,
    input: SettingsListHitTestInput,
    viewport_width: u32,
) -> SettingsListInteraction {
    let metrics = list.layout_metrics();
    let absolute_y = input.pointer_y.saturating_add(input.scroll_offset_y);
    let mut row_top = 0;
    for row in settings_rows(list, metrics) {
        let row_bottom = row_top + row.height;
        if absolute_y >= row_top && absolute_y < row_bottom {
            return row.interaction(
                list,
                row_top,
                input.pointer_x,
                absolute_y,
                viewport_width,
                metrics,
            );
        }
        row_top = row_bottom;
    }
    SettingsListInteraction::none()
}

fn unbounded_interaction_for_hit(
    list: &SettingsList,
    input: SettingsListHitTestInput,
) -> SettingsListInteraction {
    interaction_for_hit(list, input, u32::MAX)
}

fn hit_target_for_named_row(
    list: &SettingsList,
    viewport_width: u32,
    mut matches_row: impl FnMut(&SettingsListHitTestResult) -> bool,
) -> Option<SettingsListHitTarget> {
    let metrics = list.layout_metrics();
    let mut row_top = 0;
    for row in settings_rows(list, metrics) {
        if matches_row(&row.result) {
            return row.target(list, row_top, viewport_width, metrics);
        }
        row_top += row.height;
    }
    None
}

pub(super) fn content_height(list: &SettingsList) -> u32 {
    let metrics = list.layout_metrics();
    settings_rows(list, metrics)
        .into_iter()
        .map(|row| row.height)
        .sum()
}

fn settings_rows(list: &SettingsList, metrics: SettingsListLayoutMetrics) -> Vec<SettingsHitRow> {
    let mut rows = vec![SettingsHitRow::new(
        metrics.title_height(),
        SettingsListHitTestResult::None,
    )];
    rows.push(SettingsHitRow::new(
        metrics.search_box_height(),
        SettingsListHitTestResult::None,
    ));
    for visible in list.visible_sections() {
        rows.push(SettingsHitRow::new(
            metrics.section_height(),
            SettingsListHitTestResult::ToggleSection {
                section_id: visible.section.id.clone(),
            },
        ));
        for field in visible.fields {
            rows.push(SettingsHitRow::new(
                metrics.field_height(),
                SettingsListHitTestResult::Field {
                    field_id: field.id.clone(),
                },
            ));
        }
        if visible.section.footer.is_some() {
            rows.push(SettingsHitRow::new(
                metrics.footer_height(),
                SettingsListHitTestResult::None,
            ));
        }
    }
    rows
}

struct SettingsHitRow {
    height: u32,
    result: SettingsListHitTestResult,
}

impl SettingsHitRow {
    const fn new(height: u32, result: SettingsListHitTestResult) -> Self {
        Self { height, result }
    }

    fn result_for_x(
        &self,
        _pointer_x: u32,
        _metrics: SettingsListLayoutMetrics,
    ) -> SettingsListHitTestResult {
        self.result.clone()
    }

    fn target(
        &self,
        list: &SettingsList,
        row_top: u32,
        viewport_width: u32,
        _metrics: SettingsListLayoutMetrics,
    ) -> Option<SettingsListHitTarget> {
        match &self.result {
            SettingsListHitTestResult::Field { field_id } => {
                let action = list.activation_action_for_field(field_id)?;
                Some(SettingsListHitTarget {
                    rect: SettingsListHitRect {
                        x: 0,
                        y: row_top,
                        width: viewport_width,
                        height: self.height,
                    },
                    result: self.result.clone(),
                    cursor: UiCursor::Pointer,
                    hover_node_id: Some(SettingsList::field_node_id(field_id)),
                    action: Some(action),
                })
            }
            SettingsListHitTestResult::ToggleSection { section_id } => {
                Some(SettingsListHitTarget {
                    rect: SettingsListHitRect {
                        x: 0,
                        y: row_top,
                        width: viewport_width,
                        height: self.height,
                    },
                    result: self.result.clone(),
                    cursor: UiCursor::Pointer,
                    hover_node_id: Some(SettingsList::section_node_id(section_id)),
                    action: action_for_result(&self.result),
                })
            }
            SettingsListHitTestResult::None => None,
        }
    }

    fn interaction(
        &self,
        list: &SettingsList,
        row_top: u32,
        pointer_x: u32,
        absolute_y: u32,
        viewport_width: u32,
        metrics: SettingsListLayoutMetrics,
    ) -> SettingsListInteraction {
        let result = self.result_for_x(pointer_x, metrics);
        let target = self
            .target(list, row_top, viewport_width, metrics)
            .filter(|target| target.rect.contains(pointer_x, absolute_y));
        SettingsListInteraction::from_result_and_target(result, target)
    }
}
