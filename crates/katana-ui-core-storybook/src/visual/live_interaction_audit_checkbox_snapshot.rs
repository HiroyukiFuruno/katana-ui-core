use crate::visual::{Canvas, StorybookVisual};

use super::{StorybookLiveInteractionScenario, page_state, scenario};

const CHECKBOX_PAGE: &str = "checkbox";
const PREVIEW_RIGHT_EDGE: usize = 1020;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != CHECKBOX_PAGE {
        return Vec::new();
    }
    vec![
        checkbox_initial_snapshot_state_consistency_scenario(),
        checkbox_snapshot_state_consistency_scenario(),
    ]
}

fn checkbox_initial_snapshot_state_consistency_scenario() -> StorybookLiveInteractionScenario {
    let state = page_state(CHECKBOX_PAGE);
    let unchecked = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, 0, 0);
    let passed = has_preview_text(&unchecked, "idle")
        && !has_preview_text(&unchecked, "before=false after=false")
        && has_inspector_text(&unchecked, "screen: idle");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_initial_snapshot_state_consistency",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_snapshot_state_consistency_scenario() -> StorybookLiveInteractionScenario {
    let state = page_state(CHECKBOX_PAGE);
    let clicked =
        StorybookVisual.render_clicked_preset_with_scrollbar("dark", CHECKBOX_PAGE, 0, 0, true);
    let passed = has_preview_text(&clicked, "checked=true")
        && has_preview_text(&clicked, "before=false after=true")
        && has_inspector_text(&clicked, "screen: before=false after=true")
        && has_inspector_text(&clicked, "action: checkbox_toggle")
        && has_inspector_text(&clicked, "event: checked_changed");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_snapshot_state_consistency",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn has_preview_text(canvas: &Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() < PREVIEW_RIGHT_EDGE)
}

fn has_inspector_text(canvas: &Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() > PREVIEW_RIGHT_EDGE)
}
