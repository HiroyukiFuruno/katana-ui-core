use super::super::{StorybookWindowState, apply_click};
use crate::catalog::StoryCatalog;
use crate::visual::dedicated_dod_form_combo_live;
use crate::visual::dedicated_dod_form_input_live;
use crate::visual::dedicated_dod_form_select_live;
use crate::visual::dedicated_dod_form_selection_list_live;
use crate::visual::visual_interaction_test_support::component_body_pixel_diff;
use crate::visual::{layout_metrics::LayoutRect, selection_control_metrics as sc};
use crate::visual::{preview_detail, render};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule;
use std::collections::BTreeSet;

const SELECT_BOX_PAGE: &str = "select-box";
const COMBO_BOX_PAGE: &str = "combo-box";
const SEARCH_BOX_PAGE: &str = "search-box";
const SELECTION_LIST_PAGE: &str = "selection-list";
const COMPONENT_BODY_DIFF_THRESHOLD: usize = 80;
const CLICK_CENTER: usize = 2;
const TRIGGER_X_OFFSET: usize = 20;
const TRIGGER_Y_OFFSET: usize = 34;
const OPTION_X_OFFSET: usize = 20;
const SELECT_DARK_OPTION_Y_OFFSET: usize = 90;
const COMBO_TWO_OPTION_Y_OFFSET: usize = 80;
const SEARCH_FIELD_Y: usize = 36;
const SEARCH_FIELD_X: usize = 18;
const SEARCH_FIELD_WIDTH: usize = 210;
const SEARCH_FIELD_HEIGHT: usize = 34;
const SEARCH_CLEAR_X: usize = 208;
const SEARCH_CLEAR_Y: usize = 46;
const SEARCH_CLEAR_SIZE: usize = 14;
const SEARCH_STATUS_X: usize = 246;
const SEARCH_STATUS_Y: usize = 76;
const SEARCH_STATUS_WIDTH: usize = 84;
const SEARCH_STATUS_HEIGHT: usize = 20;
const SEARCH_STATUS_GAP: usize = 8;
const SEARCH_STATUS_ROW_COUNT: usize = 3;

#[test]
fn select_box_surface_opens_then_selects_option() {
    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let before = render_state(&state);

    assert!(apply_click(
        &mut state,
        rect.x + TRIGGER_X_OFFSET,
        rect.y + TRIGGER_Y_OFFSET
    ));
    assert_eq!("select_open", state.screen_state.last_action);
    assert_eq!("select_opened", state.screen_state.last_event);
    assert_eq!("open=true", state.screen_state.state_label);
    let opened = render_state(&state);
    assert!(
        component_body_pixel_diff(SELECT_BOX_PAGE, &before, &opened)
            > COMPONENT_BODY_DIFF_THRESHOLD
    );

    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + SELECT_DARK_OPTION_Y_OFFSET
    ));
    assert_eq!("select_option", state.screen_state.last_action);
    assert_eq!("select_changed", state.screen_state.last_event);
    assert_eq!("selected=dark", state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(SELECT_BOX_PAGE, &opened, &render_state(&state))
            > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn combo_box_surface_filters_then_selects_option() {
    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    let before = render_state(&state);

    assert!(apply_click(
        &mut state,
        rect.x + TRIGGER_X_OFFSET,
        rect.y + TRIGGER_Y_OFFSET
    ));
    assert_eq!("combo_filter", state.screen_state.last_action);
    assert_eq!("combo_filtered", state.screen_state.last_event);
    assert_eq!("query=tw", state.screen_state.state_label);
    let filtered = render_state(&state);
    assert!(
        component_body_pixel_diff(COMBO_BOX_PAGE, &before, &filtered)
            > COMPONENT_BODY_DIFF_THRESHOLD
    );

    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + COMBO_TWO_OPTION_Y_OFFSET
    ));
    assert_eq!("combo_select", state.screen_state.last_action);
    assert_eq!("combo_selected", state.screen_state.last_event);
    assert_eq!("selected=two", state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(COMBO_BOX_PAGE, &filtered, &render_state(&state))
            > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn select_box_option_click_without_opening_is_ignored() {
    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);

    assert_eq!(None, state.screen_state.selection.select_selected_index);
    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + SELECT_DARK_OPTION_Y_OFFSET
    ));
    assert_eq!(None, state.screen_state.selection.select_selected_index);
}

#[test]
fn combo_box_filtered_option_region_is_not_clickable_when_closed() {
    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);

    assert_eq!(None, state.screen_state.selection.combo_selected_index);
    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + COMBO_TWO_OPTION_Y_OFFSET
    ));
    assert_eq!(None, state.screen_state.selection.combo_selected_index);
}

#[test]
fn search_box_layout_parts_do_not_overlap_on_base_state() {
    let rect = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);

    let field = LayoutRect::new(
        rect.x + SEARCH_FIELD_X,
        rect.y + SEARCH_FIELD_Y,
        SEARCH_FIELD_WIDTH,
        SEARCH_FIELD_HEIGHT,
    );
    let clear = LayoutRect::new(
        rect.x + SEARCH_CLEAR_X,
        rect.y + SEARCH_CLEAR_Y,
        SEARCH_CLEAR_SIZE,
        SEARCH_CLEAR_SIZE,
    );
    let status = LayoutRect::new(
        rect.x + SEARCH_STATUS_X,
        rect.y + SEARCH_STATUS_Y,
        SEARCH_STATUS_WIDTH,
        SEARCH_STATUS_HEIGHT,
    );
    let status_row_span = SEARCH_STATUS_HEIGHT + SEARCH_STATUS_GAP;
    let read = dedicated_dod_form_input_live::search_state_read_button_rect(rect.x, rect.y);
    let type_query = dedicated_dod_form_input_live::search_type_query_button_rect(rect.x, rect.y);
    let submit = dedicated_dod_form_input_live::search_submit_button_rect(rect.x, rect.y);
    let clear_button = dedicated_dod_form_input_live::search_clear_button_rect(rect.x, rect.y);
    let case_toggle = dedicated_dod_form_input_live::search_case_toggle_button_rect(rect.x, rect.y);
    let regex_toggle =
        dedicated_dod_form_input_live::search_regex_toggle_button_rect(rect.x, rect.y);

    assert!(field.contains(clear.x, clear.y));
    assert!(field.contains(clear.right() - 1, clear.bottom() - 1));
    assert!(!field.overlaps(status));
    assert!(!status.overlaps(read));
    assert!(!read.overlaps(type_query));
    assert!(!submit.overlaps(clear_button));
    assert!(!case_toggle.overlaps(regex_toggle));
    let mut prev_row: Option<LayoutRect> = None;
    for row in 0..SEARCH_STATUS_ROW_COUNT {
        let row_rect = LayoutRect::new(
            status.x,
            status.y + row * status_row_span,
            status.width,
            status.height,
        );
        assert_eq!(status.x, row_rect.x);
        assert_eq!(status.width, row_rect.width);
        assert!(row_rect.x >= status.x);
        assert!(row_rect.right() <= status.right());
        if let Some(previous_row) = prev_row {
            assert_eq!(row_rect.y, previous_row.bottom() + SEARCH_STATUS_GAP);
            assert!(!row_rect.overlaps(previous_row));
        }
        assert!(!field.overlaps(row_rect));
        prev_row = Some(row_rect);
    }
}

#[test]
fn search_box_control_buttons_apply_expected_actions_and_state_changes() {
    let mut state = state_for(SEARCH_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);
    let read = dedicated_dod_form_input_live::search_state_read_button_rect(rect.x, rect.y);
    let type_query = dedicated_dod_form_input_live::search_type_query_button_rect(rect.x, rect.y);
    let submit = dedicated_dod_form_input_live::search_submit_button_rect(rect.x, rect.y);
    let clear = dedicated_dod_form_input_live::search_clear_button_rect(rect.x, rect.y);
    let case_toggle = dedicated_dod_form_input_live::search_case_toggle_button_rect(rect.x, rect.y);
    let regex_toggle =
        dedicated_dod_form_input_live::search_regex_toggle_button_rect(rect.x, rect.y);

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("search_state_read", state.screen_state.last_action);
    assert_eq!("search_value_read", state.screen_state.last_event);
    assert_eq!(
        "value=query case=false regex=false",
        state.screen_state.state_label
    );

    assert!(apply_click(
        &mut state,
        type_query.x + CLICK_CENTER,
        type_query.y + CLICK_CENTER
    ));
    assert_eq!("search_type_query", state.screen_state.last_action);
    assert_eq!("input_value", state.screen_state.last_event);
    assert_eq!(
        "value=typed query case=false regex=false",
        state.screen_state.state_label
    );
    assert!(state.screen_state.search_box.typed);
    assert!(!state.screen_state.search_box.cleared);
    assert!(!state.screen_state.search_box.submitted);

    assert!(apply_click(
        &mut state,
        submit.x + CLICK_CENTER,
        submit.y + CLICK_CENTER
    ));
    assert_eq!("search_submit", state.screen_state.last_action);
    assert_eq!("search_submitted", state.screen_state.last_event);
    assert_eq!(
        "value=typed query submitted=true",
        state.screen_state.state_label
    );
    assert!(state.screen_state.search_box.typed);
    assert!(state.screen_state.search_box.submitted);

    assert!(apply_click(
        &mut state,
        clear.x + CLICK_CENTER,
        clear.y + CLICK_CENTER
    ));
    assert_eq!("search_clear", state.screen_state.last_action);
    assert_eq!("clear_value", state.screen_state.last_event);
    assert_eq!(
        "value=empty case=false regex=false",
        state.screen_state.state_label
    );
    assert!(state.screen_state.search_box.cleared);
    assert!(!state.screen_state.search_box.typed);
    assert!(!state.screen_state.search_box.submitted);

    assert!(apply_click(
        &mut state,
        case_toggle.x + CLICK_CENTER,
        case_toggle.y + CLICK_CENTER
    ));
    assert_eq!("search_case_toggle", state.screen_state.last_action);
    assert_eq!("search_option_changed", state.screen_state.last_event);
    assert_eq!(
        "value=empty case=true regex=false",
        state.screen_state.state_label
    );

    assert!(apply_click(
        &mut state,
        regex_toggle.x + CLICK_CENTER,
        regex_toggle.y + CLICK_CENTER
    ));
    assert_eq!("search_regex_toggle", state.screen_state.last_action);
    assert_eq!("search_option_changed", state.screen_state.last_event);
    assert_eq!(
        "value=empty case=true regex=true",
        state.screen_state.state_label
    );
}

#[test]
fn search_box_hit_target_includes_field_clear_and_control_buttons() {
    let mut state = state_for(SEARCH_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);
    let read = dedicated_dod_form_input_live::search_state_read_button_rect(rect.x, rect.y);
    let type_query = dedicated_dod_form_input_live::search_type_query_button_rect(rect.x, rect.y);
    let submit = dedicated_dod_form_input_live::search_submit_button_rect(rect.x, rect.y);
    let clear = dedicated_dod_form_input_live::search_clear_button_rect(rect.x, rect.y);
    let case_toggle = dedicated_dod_form_input_live::search_case_toggle_button_rect(rect.x, rect.y);
    let regex_toggle =
        dedicated_dod_form_input_live::search_regex_toggle_button_rect(rect.x, rect.y);

    let field = dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y);
    let inline_clear = dedicated_dod_form_input_live::search_inline_clear_rect(rect.x, rect.y);
    assert!(rect.contains(SEARCH_CLEAR_X + rect.x, SEARCH_CLEAR_Y + rect.y));
    assert!(apply_click(
        &mut state,
        field.x + CLICK_CENTER,
        field.y + CLICK_CENTER
    ));
    assert_eq!("search_type_query", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        inline_clear.x + CLICK_CENTER,
        inline_clear.y + CLICK_CENTER
    ));
    assert_eq!("search_clear", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        type_query.x + CLICK_CENTER,
        type_query.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        submit.x + CLICK_CENTER,
        submit.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        clear.x + CLICK_CENTER,
        clear.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        case_toggle.x + CLICK_CENTER,
        case_toggle.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        regex_toggle.x + CLICK_CENTER,
        regex_toggle.y + CLICK_CENTER
    ));
    assert_eq!("search_regex_toggle", state.screen_state.last_action);
}

#[test]
fn search_box_visual_and_catalog_use_same_typed_action_names() -> Result<(), String> {
    let search = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == SEARCH_BOX_PAGE)
        .ok_or_else(|| "search-box story missing".to_string())?;
    let catalog_actions: BTreeSet<String> = search
        .callback_logs
        .iter()
        .map(|it| it.action.clone())
        .filter(|it| {
            matches!(
                it.as_str(),
                "search_state_read"
                    | "search_type_query"
                    | "search_submit"
                    | "search_clear"
                    | "search_case_toggle"
                    | "search_regex_toggle"
            )
        })
        .collect();

    let mut state = state_for(SEARCH_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);
    let mut visual_actions: BTreeSet<String> = BTreeSet::new();
    for point in [
        dedicated_dod_form_input_live::search_state_read_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_type_query_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_submit_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_clear_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_case_toggle_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_regex_toggle_button_rect(rect.x, rect.y),
    ] {
        assert!(apply_click(
            &mut state,
            point.x + CLICK_CENTER,
            point.y + CLICK_CENTER
        ));
        visual_actions.insert(state.screen_state.last_action.to_string());
    }

    assert_eq!(catalog_actions, visual_actions);
    Ok(())
}

#[test]
fn search_box_visual_state_matches_core_search_box_state_contract() {
    let mut core_search = molecule::SearchBox::new("Search box")
        .value("query")
        .submit_on_enter(true);
    let target = core_search.state_id().clone();
    let _ = core_search.apply_action(&UiAction::input_value(target.clone(), "typed query"));
    let _ = core_search.apply_action(&UiAction::cursor_selection(target.clone(), 11, 0, 11));
    let core_result = core_search.apply_action(&UiAction::search_submitted(target.clone()));
    assert!(
        core_result
            .callback_log
            .iter()
            .any(|it| it.action == "search_submitted")
    );
    let _ = core_search.apply_action(&UiAction::clear_value(target));
    let core_node: katana_ui_core::render_model::UiNode = core_search.into();
    let core_interaction = &core_node.props().interaction;
    assert_eq!("", core_interaction.value);
    assert_eq!(11, core_interaction.cursor);
    assert_eq!(0, core_interaction.selection_start);
    assert_eq!(11, core_interaction.selection_end);

    let mut visual = state_for(SEARCH_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);
    for point in [
        dedicated_dod_form_input_live::search_type_query_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_submit_button_rect(rect.x, rect.y),
        dedicated_dod_form_input_live::search_clear_button_rect(rect.x, rect.y),
    ] {
        assert!(apply_click(
            &mut visual,
            point.x + CLICK_CENTER,
            point.y + CLICK_CENTER
        ));
    }
    assert_eq!("search_clear", visual.screen_state.last_action);
    assert_eq!(
        "value=empty case=false regex=false",
        visual.screen_state.state_label
    );
    assert!(!visual.screen_state.search_box.typed);
    assert!(visual.screen_state.search_box.cleared);
    assert!(!visual.screen_state.search_box.submitted);
    assert!(!visual.screen_state.search_box.case_sensitive);
    assert!(!visual.screen_state.search_box.regex);
}

#[test]
fn select_box_layout_parts_do_not_overlap_in_open_and_base_states() {
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);

    let trigger = LayoutRect::new(
        rect.x + sc::TRIGGER_X,
        rect.y + sc::TRIGGER_Y,
        sc::TRIGGER_WIDTH,
        sc::TRIGGER_HEIGHT,
    );
    let status = LayoutRect::new(
        rect.x + sc::STATUS_X,
        rect.y + sc::STATUS_Y,
        sc::STATUS_WIDTH,
        sc::STATUS_HEIGHT,
    );
    assert!(!trigger.overlaps(status));

    let mut state = state_for(SELECT_BOX_PAGE);
    let opened_rect = {
        let _ = apply_click(
            &mut state,
            rect.x + sc::TRIGGER_X + 1,
            rect.y + sc::TRIGGER_Y + 1,
        );
        LayoutRect::new(
            rect.x + sc::TRIGGER_X,
            rect.y + sc::SELECT_OPTIONS_Y,
            sc::TRIGGER_WIDTH,
            sc::SELECT_OPTION_HEIGHT * sc::SELECT_OPTION_COUNT,
        )
    };
    assert!(!opened_rect.overlaps(status));
}

#[test]
fn select_box_control_buttons_apply_expected_actions_and_state_changes() {
    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let read = dedicated_dod_form_select_live::select_state_read_button_rect(rect.x, rect.y);
    let open = dedicated_dod_form_select_live::select_open_button_rect(rect.x, rect.y);
    let close = dedicated_dod_form_select_live::select_close_button_rect(rect.x, rect.y);
    let reset = dedicated_dod_form_select_live::select_reset_button_rect(rect.x, rect.y);

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("select_state_read", state.screen_state.last_action);
    assert_eq!("select_state_read", state.screen_state.last_event);
    assert_eq!("open=false selected=none", state.screen_state.state_label);

    assert!(apply_click(
        &mut state,
        open.x + CLICK_CENTER,
        open.y + CLICK_CENTER
    ));
    assert_eq!("select_open", state.screen_state.last_action);
    assert_eq!("select_opened", state.screen_state.last_event);
    assert_eq!("open=true", state.screen_state.state_label);
    assert!(state.screen_state.selection.select_open);

    assert!(apply_click(
        &mut state,
        close.x + CLICK_CENTER,
        close.y + CLICK_CENTER
    ));
    assert_eq!("select_close", state.screen_state.last_action);
    assert_eq!("select_closed", state.screen_state.last_event);
    assert_eq!("open=false", state.screen_state.state_label);
    assert!(!state.screen_state.selection.select_open);

    assert!(apply_click(
        &mut state,
        open.x + CLICK_CENTER,
        open.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + SELECT_DARK_OPTION_Y_OFFSET
    ));
    assert_eq!("selected=dark", state.screen_state.state_label);

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("select_reset", state.screen_state.last_action);
    assert_eq!("select_reset", state.screen_state.last_event);
    assert_eq!("selected=none", state.screen_state.state_label);
    assert_eq!(None, state.screen_state.selection.select_selected_index);
    assert!(!state.screen_state.selection.select_open);
}

#[test]
fn select_box_hit_target_includes_trigger_option_and_control_buttons() {
    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let trigger = LayoutRect::new(
        rect.x + sc::TRIGGER_X,
        rect.y + sc::TRIGGER_Y,
        sc::TRIGGER_WIDTH,
        sc::TRIGGER_HEIGHT,
    );
    let option_label_x = rect.x + sc::TRIGGER_X + sc::TEXT_X;
    let option_label_y =
        rect.y + sc::SELECT_OPTIONS_Y + sc::SELECT_OPTION_HEIGHT * 2 + CLICK_CENTER;
    let read = dedicated_dod_form_select_live::select_state_read_button_rect(rect.x, rect.y);
    let open = dedicated_dod_form_select_live::select_open_button_rect(rect.x, rect.y);
    let close = dedicated_dod_form_select_live::select_close_button_rect(rect.x, rect.y);
    let reset = dedicated_dod_form_select_live::select_reset_button_rect(rect.x, rect.y);

    assert!(rect.contains(trigger.x + CLICK_CENTER, trigger.y + CLICK_CENTER));
    assert!(!read.overlaps(open));
    assert!(!open.overlaps(close));
    assert!(!close.overlaps(reset));

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("select_state_read", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        open.x + CLICK_CENTER,
        open.y + CLICK_CENTER
    ));
    assert_eq!("select_open", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        close.x + CLICK_CENTER,
        close.y + CLICK_CENTER
    ));
    assert_eq!("select_close", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("select_reset", state.screen_state.last_action);

    assert!(apply_click(
        &mut state,
        trigger.x + CLICK_CENTER,
        trigger.y + CLICK_CENTER
    ));
    assert!(apply_click(&mut state, option_label_x, option_label_y));
    assert_eq!("select_option", state.screen_state.last_action);
    assert_eq!("select_changed", state.screen_state.last_event);
    assert_eq!("selected=dark", state.screen_state.state_label);
}

#[test]
fn select_box_visual_and_catalog_use_same_typed_action_names() -> Result<(), String> {
    let select = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == "select-box")
        .ok_or_else(|| "select-box story missing".to_string())?;
    let catalog_actions: BTreeSet<String> = select
        .callback_logs
        .iter()
        .map(|it| it.action.clone())
        .filter(|it| {
            matches!(
                it.as_str(),
                "select_state_read"
                    | "select_open"
                    | "select_close"
                    | "select_option"
                    | "select_reset"
            )
        })
        .collect();

    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let read = dedicated_dod_form_select_live::select_state_read_button_rect(rect.x, rect.y);
    let open = dedicated_dod_form_select_live::select_open_button_rect(rect.x, rect.y);
    let close = dedicated_dod_form_select_live::select_close_button_rect(rect.x, rect.y);
    let reset = dedicated_dod_form_select_live::select_reset_button_rect(rect.x, rect.y);
    let mut visual_actions: BTreeSet<String> = BTreeSet::new();
    for point in [
        (read.x + CLICK_CENTER, read.y + CLICK_CENTER),
        (open.x + CLICK_CENTER, open.y + CLICK_CENTER),
        (close.x + CLICK_CENTER, close.y + CLICK_CENTER),
        (open.x + CLICK_CENTER, open.y + CLICK_CENTER),
        (
            rect.x + OPTION_X_OFFSET,
            rect.y + SELECT_DARK_OPTION_Y_OFFSET,
        ),
        (reset.x + CLICK_CENTER, reset.y + CLICK_CENTER),
    ] {
        assert!(apply_click(&mut state, point.0, point.1));
        visual_actions.insert(state.screen_state.last_action.to_string());
    }

    assert_eq!(catalog_actions, visual_actions);
    Ok(())
}

#[test]
fn select_box_visual_state_matches_core_select_box_selected_contract() {
    let mut core_select = molecule::SelectBox::new("Select box")
        .item(molecule::ChoiceItem::new("light", "Light"))
        .item(molecule::ChoiceItem::new("dark", "Dark"))
        .item(molecule::ChoiceItem::new("system", "System"));
    let target = core_select.state_id().clone();
    let core_result = core_select.apply_action(&UiAction::select_box_selected(target, 1));
    assert!(
        core_result
            .callback_log
            .iter()
            .any(|it| it.action == "select_box_selected")
    );
    let core_node: katana_ui_core::render_model::UiNode = core_select.into();
    let core_interaction = &core_node.props().interaction;
    assert_eq!(1, core_interaction.selected_index);
    assert!(core_interaction.has_selection);
    assert_eq!("dark", core_interaction.value);
    assert!(!core_interaction.open);

    let mut visual = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    assert!(apply_click(
        &mut visual,
        rect.x + sc::TRIGGER_X + CLICK_CENTER,
        rect.y + sc::TRIGGER_Y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut visual,
        rect.x + OPTION_X_OFFSET,
        rect.y + SELECT_DARK_OPTION_Y_OFFSET
    ));
    assert_eq!(Some(2), visual.screen_state.selection.select_selected_index);
    assert_eq!("selected=dark", visual.screen_state.state_label);
}

#[test]
fn combo_box_control_buttons_apply_expected_actions_and_state_changes() {
    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    let read = dedicated_dod_form_combo_live::combo_state_read_button_rect(rect.x, rect.y);
    let filter = dedicated_dod_form_combo_live::combo_filter_button_rect(rect.x, rect.y);
    let select = dedicated_dod_form_combo_live::combo_select_button_rect(rect.x, rect.y);
    let reset = dedicated_dod_form_combo_live::combo_reset_button_rect(rect.x, rect.y);

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("combo_state_read", state.screen_state.last_action);
    assert_eq!("combo_state_read", state.screen_state.last_event);
    assert_eq!(
        "open=false query=empty selected=none",
        state.screen_state.state_label
    );

    assert!(apply_click(
        &mut state,
        filter.x + CLICK_CENTER,
        filter.y + CLICK_CENTER
    ));
    assert_eq!("combo_filter", state.screen_state.last_action);
    assert_eq!("combo_filtered", state.screen_state.last_event);
    assert_eq!("query=tw", state.screen_state.state_label);
    assert!(state.screen_state.selection.combo_open);
    assert!(state.screen_state.selection.combo_filtered);

    assert!(apply_click(
        &mut state,
        select.x + CLICK_CENTER,
        select.y + CLICK_CENTER
    ));
    assert_eq!("combo_select", state.screen_state.last_action);
    assert_eq!("combo_selected", state.screen_state.last_event);
    assert_eq!("selected=two", state.screen_state.state_label);
    assert_eq!(Some(1), state.screen_state.selection.combo_selected_index);
    assert!(!state.screen_state.selection.combo_open);

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("combo_reset", state.screen_state.last_action);
    assert_eq!("combo_reset", state.screen_state.last_event);
    assert_eq!("query=empty selected=none", state.screen_state.state_label);
    assert_eq!(None, state.screen_state.selection.combo_selected_index);
    assert!(!state.screen_state.selection.combo_open);
    assert!(!state.screen_state.selection.combo_filtered);
}

#[test]
fn combo_box_hit_target_includes_trigger_option_and_control_buttons() {
    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    let trigger = LayoutRect::new(
        rect.x + sc::TRIGGER_X,
        rect.y + sc::TRIGGER_Y,
        sc::TRIGGER_WIDTH,
        sc::TRIGGER_HEIGHT,
    );
    let option_label_x = rect.x + sc::TRIGGER_X + sc::TEXT_X;
    let option_label_y = rect.y + sc::COMBO_OPTIONS_Y + CLICK_CENTER;
    let read = dedicated_dod_form_combo_live::combo_state_read_button_rect(rect.x, rect.y);
    let filter = dedicated_dod_form_combo_live::combo_filter_button_rect(rect.x, rect.y);
    let select = dedicated_dod_form_combo_live::combo_select_button_rect(rect.x, rect.y);
    let reset = dedicated_dod_form_combo_live::combo_reset_button_rect(rect.x, rect.y);

    assert!(rect.contains(trigger.x + CLICK_CENTER, trigger.y + CLICK_CENTER));
    assert!(!read.overlaps(filter));
    assert!(!filter.overlaps(select));
    assert!(!select.overlaps(reset));

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("combo_state_read", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        filter.x + CLICK_CENTER,
        filter.y + CLICK_CENTER
    ));
    assert_eq!("combo_filter", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        select.x + CLICK_CENTER,
        select.y + CLICK_CENTER
    ));
    assert_eq!("combo_select", state.screen_state.last_action);
    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("combo_reset", state.screen_state.last_action);

    assert!(apply_click(
        &mut state,
        trigger.x + CLICK_CENTER,
        trigger.y + CLICK_CENTER
    ));
    assert!(apply_click(&mut state, option_label_x, option_label_y));
    assert_eq!("combo_select", state.screen_state.last_action);
    assert_eq!("combo_selected", state.screen_state.last_event);
    assert_eq!("selected=two", state.screen_state.state_label);
}

#[test]
fn combo_box_visual_and_catalog_use_same_typed_action_names() -> Result<(), String> {
    let combo = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == "combo-box")
        .ok_or_else(|| "combo-box story missing".to_string())?;
    let catalog_actions: BTreeSet<String> = combo
        .callback_logs
        .iter()
        .map(|it| it.action.clone())
        .filter(|it| {
            matches!(
                it.as_str(),
                "combo_state_read" | "combo_filter" | "combo_select" | "combo_reset"
            )
        })
        .collect();

    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    let read = dedicated_dod_form_combo_live::combo_state_read_button_rect(rect.x, rect.y);
    let filter = dedicated_dod_form_combo_live::combo_filter_button_rect(rect.x, rect.y);
    let select = dedicated_dod_form_combo_live::combo_select_button_rect(rect.x, rect.y);
    let reset = dedicated_dod_form_combo_live::combo_reset_button_rect(rect.x, rect.y);
    let mut visual_actions: BTreeSet<String> = BTreeSet::new();
    for point in [
        (read.x + CLICK_CENTER, read.y + CLICK_CENTER),
        (filter.x + CLICK_CENTER, filter.y + CLICK_CENTER),
        (select.x + CLICK_CENTER, select.y + CLICK_CENTER),
        (reset.x + CLICK_CENTER, reset.y + CLICK_CENTER),
    ] {
        assert!(apply_click(&mut state, point.0, point.1));
        visual_actions.insert(state.screen_state.last_action.to_string());
    }

    assert_eq!(catalog_actions, visual_actions);
    Ok(())
}

#[test]
fn combo_box_visual_state_matches_core_combo_box_selected_contract() {
    let mut core_combo = molecule::ComboBox::new("Combo box")
        .item(molecule::ChoiceItem::new("one", "One"))
        .item(molecule::ChoiceItem::new("two", "Two"));
    let target = core_combo.state_id().clone();
    let core_result = core_combo.apply_action(&UiAction::select_box_selected(target, 1));
    assert!(
        core_result
            .callback_log
            .iter()
            .any(|it| it.action == "select_box_selected")
    );
    let core_node: katana_ui_core::render_model::UiNode = core_combo.into();
    let core_interaction = &core_node.props().interaction;
    assert_eq!(1, core_interaction.selected_index);
    assert!(core_interaction.has_selection);
    assert_eq!("two", core_interaction.value);
    assert!(!core_interaction.open);

    let mut visual = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    assert!(apply_click(
        &mut visual,
        rect.x + sc::TRIGGER_X + CLICK_CENTER,
        rect.y + sc::TRIGGER_Y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut visual,
        rect.x + OPTION_X_OFFSET,
        rect.y + COMBO_TWO_OPTION_Y_OFFSET
    ));
    assert_eq!(Some(1), visual.screen_state.selection.combo_selected_index);
    assert_eq!("selected=two", visual.screen_state.state_label);
}

#[test]
fn selection_list_row_click_updates_list_state_and_component_body() {
    let mut state = state_for(SELECTION_LIST_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let before = render_state(&state);

    assert_eq!(
        None,
        state.screen_state.selection.selection_list_selected_index
    );
    assert!(apply_click(
        &mut state,
        rect.x + sc::TRIGGER_X + sc::OPTION_ROW_INSET,
        rect.y
            + sc::SELECTION_LIST_Y
            + sc::SELECTION_LIST_ROW_HEIGHT
            + sc::SELECTION_LIST_ROW_HEIGHT / 2,
    ));
    assert_eq!(
        Some(1),
        state.screen_state.selection.selection_list_selected_index
    );
    assert_eq!("selection_list_select_row", state.screen_state.last_action);
    assert_eq!("selection_list_changed", state.screen_state.last_event);
    assert_eq!(
        "single=1 multi=none focus=1",
        state.screen_state.state_label
    );
    assert!(
        component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &render_state(&state))
            > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn selection_list_rows_and_status_do_not_overlap() {
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let status = LayoutRect::new(
        rect.x + sc::STATUS_X,
        rect.y + sc::STATUS_Y,
        sc::STATUS_WIDTH,
        sc::STATUS_HEIGHT,
    );

    let rows_height = sc::SELECTION_LIST_ROW_HEIGHT * sc::SELECTION_LIST_ROW_COUNT;
    let rows = LayoutRect::new(
        rect.x + sc::TRIGGER_X,
        rect.y + sc::SELECTION_LIST_Y,
        sc::TRIGGER_WIDTH,
        rows_height,
    );
    assert!(!rows.overlaps(status));
    for index in 0..sc::SELECTION_LIST_ROW_COUNT {
        let row = LayoutRect::new(
            rows.x,
            rows.y + index * sc::SELECTION_LIST_ROW_HEIGHT,
            rows.width,
            sc::SELECTION_LIST_ROW_HEIGHT,
        );
        assert!(rows.overlaps(row));
        assert!(rows.contains(row.x, row.y));
        assert!(rows.contains(row.right() - 1, row.bottom() - 1));
    }
}

#[test]
fn selection_list_control_buttons_apply_expected_actions_and_state_changes() {
    let mut state = state_for(SELECTION_LIST_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let state_read = dedicated_dod_form_selection_list_live::selection_list_state_read_button_rect(
        rect.x, rect.y,
    );
    let select_row = dedicated_dod_form_selection_list_live::selection_list_select_row_button_rect(
        rect.x, rect.y,
    );
    let multi_toggle =
        dedicated_dod_form_selection_list_live::selection_list_multi_toggle_button_rect(
            rect.x, rect.y,
        );
    let keyboard_next =
        dedicated_dod_form_selection_list_live::selection_list_keyboard_next_button_rect(
            rect.x, rect.y,
        );
    let reset =
        dedicated_dod_form_selection_list_live::selection_list_reset_button_rect(rect.x, rect.y);

    assert!(apply_click(
        &mut state,
        state_read.x + CLICK_CENTER,
        state_read.y + CLICK_CENTER
    ));
    assert_eq!("selection_list_state_read", state.screen_state.last_action);
    assert_eq!(
        "single=none multi=none focus=none",
        state.screen_state.state_label
    );

    assert!(apply_click(
        &mut state,
        select_row.x + CLICK_CENTER,
        select_row.y + CLICK_CENTER
    ));
    assert_eq!("selection_list_select_row", state.screen_state.last_action);
    assert_eq!(
        "single=1 multi=none focus=1",
        state.screen_state.state_label
    );

    assert!(apply_click(
        &mut state,
        multi_toggle.x + CLICK_CENTER,
        multi_toggle.y + CLICK_CENTER
    ));
    assert_eq!(
        "selection_list_multi_toggle",
        state.screen_state.last_action
    );
    assert_eq!("single=1 multi=1 focus=1", state.screen_state.state_label);

    assert!(apply_click(
        &mut state,
        keyboard_next.x + CLICK_CENTER,
        keyboard_next.y + CLICK_CENTER
    ));
    assert_eq!(
        "selection_list_keyboard_next",
        state.screen_state.last_action
    );
    assert_eq!("single=2 multi=1 focus=2", state.screen_state.state_label);

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("selection_list_reset", state.screen_state.last_action);
    assert_eq!(
        "single=none multi=none focus=none",
        state.screen_state.state_label
    );
}

#[test]
fn selection_list_visual_and_catalog_use_same_typed_action_names() -> Result<(), String> {
    let list = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == SELECTION_LIST_PAGE)
        .ok_or_else(|| "selection-list story missing".to_string())?;
    let catalog_actions: BTreeSet<String> = list
        .callback_logs
        .iter()
        .map(|it| it.action.clone())
        .filter(|it| {
            matches!(
                it.as_str(),
                "selection_list_state_read"
                    | "selection_list_select_row"
                    | "selection_list_multi_toggle"
                    | "selection_list_keyboard_next"
                    | "selection_list_reset"
            )
        })
        .collect();

    let mut state = state_for(SELECTION_LIST_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let mut visual_actions: BTreeSet<String> = BTreeSet::new();
    for point in [
        dedicated_dod_form_selection_list_live::selection_list_state_read_button_rect(
            rect.x, rect.y,
        ),
        dedicated_dod_form_selection_list_live::selection_list_select_row_button_rect(
            rect.x, rect.y,
        ),
        dedicated_dod_form_selection_list_live::selection_list_multi_toggle_button_rect(
            rect.x, rect.y,
        ),
        dedicated_dod_form_selection_list_live::selection_list_keyboard_next_button_rect(
            rect.x, rect.y,
        ),
        dedicated_dod_form_selection_list_live::selection_list_reset_button_rect(rect.x, rect.y),
    ] {
        assert!(apply_click(
            &mut state,
            point.x + CLICK_CENTER,
            point.y + CLICK_CENTER
        ));
        visual_actions.insert(state.screen_state.last_action.to_string());
    }
    assert_eq!(catalog_actions, visual_actions);
    Ok(())
}

#[test]
fn selection_list_visual_state_matches_core_selection_list_state_contract() {
    let mut core_list = molecule::SelectionList::new("Selection list")
        .item(molecule::ChoiceItem::new("first", "First"))
        .item(molecule::ChoiceItem::new("second", "Second"))
        .item(molecule::ChoiceItem::new("third", "Third"));
    let target = core_list.state_id().clone();
    let selected = core_list.apply_action(&UiAction::select_box_selected(target.clone(), 1));
    let moved = core_list.apply_action(&UiAction::set_selected_index(target, 2));
    assert!(
        selected
            .callback_log
            .iter()
            .any(|it| it.action == "select_box_selected")
    );
    assert!(
        moved
            .callback_log
            .iter()
            .any(|it| it.action == "set_selected_index")
    );
    let core_node: katana_ui_core::render_model::UiNode = core_list.into();
    let core_interaction = &core_node.props().interaction;
    assert_eq!(2, core_interaction.selected_index);
    assert!(core_interaction.has_selection);
    assert_eq!("third", core_interaction.value);
    assert!(!core_interaction.open);

    let mut visual = state_for(SELECTION_LIST_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    for point in [
        dedicated_dod_form_selection_list_live::selection_list_select_row_button_rect(
            rect.x, rect.y,
        ),
        dedicated_dod_form_selection_list_live::selection_list_keyboard_next_button_rect(
            rect.x, rect.y,
        ),
    ] {
        assert!(apply_click(
            &mut visual,
            point.x + CLICK_CENTER,
            point.y + CLICK_CENTER
        ));
    }
    assert_eq!(
        "selection_list_keyboard_next",
        visual.screen_state.last_action
    );
    assert_eq!(
        "single=2 multi=none focus=2",
        visual.screen_state.state_label
    );
}

fn state_for(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> crate::visual::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
