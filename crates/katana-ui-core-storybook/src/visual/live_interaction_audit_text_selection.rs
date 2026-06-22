use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_text_copy_shortcut_for_audit, apply_text_paste_shortcut_for_audit,
    apply_text_selection_drag_for_audit,
};

const TEXT_INPUT_PASTE_SELECTION_START: usize = 1;
const TEXT_INPUT_PASTE_SELECTION_END: usize = 4;
const TEXT_AREA_PASTE_SELECTION_START: usize = 1;
const TEXT_AREA_PASTE_SELECTION_END: usize = 3;

pub(in crate::visual) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    let mut scenarios = match page {
        "text" => vec![
            text_drag_selection_scenario(page),
            text_keyboard_copy_scenario(page),
            text_keyboard_paste_scenario(page),
            text_zero_distance_drag_scenario(page),
        ],
        "text-input" | "text-area" => vec![text_keyboard_paste_scenario(page)],
        _ => Vec::new(),
    };
    scenarios.extend(super::live_interaction_audit_text_controls::scenarios(page));
    scenarios
}

fn text_keyboard_paste_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    state.clipboard_text = "文".to_string();
    match page {
        "text-input" => {
            state
                .screen_state
                .register_text_input_focus_for("text-input.preview", "abcdef", false);
            state.screen_state.set_text_input_selection_for(
                "text-input.preview",
                TEXT_INPUT_PASTE_SELECTION_START,
                TEXT_INPUT_PASTE_SELECTION_END,
            );
            let pasted = apply_text_paste_shortcut_for_audit(&mut state);
            let passed = pasted
                && state.screen_state.text_input_value() == "a文ef"
                && state.screen_state.last_action == "text_input_paste"
                && state.screen_state.last_event == "clipboard_paste";
            scenario(
                page,
                "text_keyboard_paste",
                "keyboard",
                pasted,
                passed,
                0,
                &state,
            )
        }
        "text-area" => {
            state
                .screen_state
                .register_text_area_focus_for("text-area.preview", false, false);
            state
                .screen_state
                .set_text_area_value_for("text-area.preview", "A日🔷b");
            state.screen_state.set_text_area_selection_for(
                "text-area.preview",
                TEXT_AREA_PASTE_SELECTION_START,
                TEXT_AREA_PASTE_SELECTION_END,
            );
            let pasted = apply_text_paste_shortcut_for_audit(&mut state);
            let passed = pasted
                && state.screen_state.text_area_value() == "A文b"
                && state.screen_state.last_action == "text_area_paste"
                && state.screen_state.last_event == "clipboard_paste";
            scenario(
                page,
                "text_keyboard_paste",
                "keyboard",
                pasted,
                passed,
                0,
                &state,
            )
        }
        "text" => {
            let pasted = apply_text_paste_shortcut_for_audit(&mut state);
            let passed = !pasted
                && state.screen_state.last_action == "none"
                && state.screen_state.last_event == "none";
            state.clipboard_text.clear();
            scenario(
                page,
                "text_keyboard_paste",
                "keyboard",
                pasted,
                passed,
                0,
                &state,
            )
        }
        _ => scenario(
            page,
            "text_keyboard_paste",
            "keyboard",
            false,
            false,
            0,
            &state,
        ),
    }
}

fn text_drag_selection_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let frame = before.clone();
    let Some(rect) = first_selectable_text_rect(page, &frame) else {
        return scenario(page, "text_drag_selection", "drag", false, false, 0, &state);
    };
    let dragged = apply_text_selection_drag_for_audit(
        &mut state,
        &frame,
        (rect.x, rect.y),
        (rect.right(), rect.bottom()),
    );
    let passed = dragged
        && state.text_selection_start == Some((rect.x, rect.y))
        && state.text_selection_end == Some((rect.right(), rect.bottom()));
    let after = render_text_selection_state(page, &state);
    let body_pixel_diff = selection_pixel_diff(page, &before, &after);
    scenario(
        page,
        "text_drag_selection",
        "drag",
        dragged,
        passed && body_pixel_diff > 0,
        body_pixel_diff,
        &state,
    )
}

fn text_keyboard_copy_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let frame = render_state(page, &state);
    let Some(rect) = first_selectable_text_rect(page, &frame) else {
        return scenario(
            page,
            "text_keyboard_copy",
            "keyboard",
            false,
            false,
            0,
            &state,
        );
    };
    let dragged = apply_text_selection_drag_for_audit(
        &mut state,
        &frame,
        (rect.x, rect.y),
        (rect.right(), rect.bottom()),
    );
    let copied = dragged && apply_text_copy_shortcut_for_audit(&mut state, &frame);
    let passed = copied && !state.clipboard_text.trim().is_empty();
    scenario(
        page,
        "text_keyboard_copy",
        "keyboard",
        copied,
        passed,
        0,
        &state,
    )
}

fn text_zero_distance_drag_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let frame = before.clone();
    let Some(rect) = first_selectable_text_rect(page, &frame) else {
        return scenario(
            page,
            "text_zero_distance_drag_no_selection",
            "drag",
            false,
            false,
            0,
            &state,
        );
    };
    let dragged =
        apply_text_selection_drag_for_audit(&mut state, &frame, (rect.x, rect.y), (rect.x, rect.y));
    let copied = apply_text_copy_shortcut_for_audit(&mut state, &frame);
    let after = render_text_selection_state(page, &state);
    let body_pixel_diff = selection_pixel_diff(page, &before, &after);
    let passed = !dragged
        && !copied
        && state.text_selection_start == Some((rect.x, rect.y))
        && state.text_selection_end.is_none()
        && state.screen_state.last_action == "none"
        && state.screen_state.last_event == "none"
        && state.clipboard_text.is_empty()
        && body_pixel_diff == 0;
    scenario(
        page,
        "text_zero_distance_drag_no_selection",
        "drag",
        dragged,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn first_selectable_text_rect(
    page: &'static str,
    frame: &super::super::Canvas,
) -> Option<LayoutRect> {
    let body = preview_detail::component_action_hit_rect(page);
    frame
        .text_runs()
        .iter()
        .find(|run| {
            let rect = run.rect();
            !run.text().trim().is_empty()
                && rect.width > 0
                && rect.height > 0
                && rect_overlaps(body, rect)
        })
        .map(|run| run.rect())
        .or_else(|| {
            frame
                .text_runs()
                .iter()
                .find(|run| {
                    let rect = run.rect();
                    !run.text().trim().is_empty() && rect.width > 0 && rect.height > 0
                })
                .map(|run| run.rect())
        })
}

fn rect_overlaps(a: LayoutRect, b: LayoutRect) -> bool {
    a.x < b.right() && a.right() > b.x && a.y < b.bottom() && a.bottom() > b.y
}

fn selection_pixel_diff(
    page: &'static str,
    before: &super::super::Canvas,
    after: &super::super::Canvas,
) -> usize {
    let component_diff = component_body_pixel_diff(page, before, after);
    if component_diff > 0 {
        return component_diff;
    }
    before
        .pixels()
        .iter()
        .zip(after.pixels())
        .filter(|(before, after)| before != after)
        .count()
}

fn render_text_selection_state(
    page: &'static str,
    state: &super::StorybookWindowState,
) -> super::super::Canvas {
    let mut frame = render_state(page, state);
    let colors = super::super::palette::VisualPalette::from_theme(
        &katana_ui_core::theme::ThemeSnapshot::dark(),
    );
    super::super::text_selection_overlay::draw_text_selection_highlight(
        &mut frame,
        state.text_selection_start,
        state.text_selection_end,
        colors.selection,
    );
    frame
}
