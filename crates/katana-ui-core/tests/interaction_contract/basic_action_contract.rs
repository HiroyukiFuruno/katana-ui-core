use katana_ui_core::atom::{
    Badge, Button, Checkbox, ColorSwatch, Input, ProgressBar, Radio, SlideControl, Toggle,
};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::state::UiStateHandle;

const PROGRESS_PERCENT: u8 = 64;
const COLOR_RED: u8 = 64;
const COLOR_GREEN: u8 = 128;
const COLOR_BLUE: u8 = 255;
const COLOR_ALPHA: u8 = 204;
const COLOR_HUE: u16 = 215;
const COLOR_RGBA: &str = "rgba(64, 128, 255, 204)";

#[test]
fn action_targets_only_the_matching_component_state() {
    let mut first = Button::new("Save");
    let mut second = Button::new("Save");
    let action = UiAction::button_press(first.state_id().clone());

    let first_result = first.apply_action(&action);
    let second_result = second.apply_action(&action);

    assert!(first_result.handled);
    assert!(!second_result.handled);
    assert_eq!(1, first_result.callback_log.len());
    assert_eq!("button_press", first_result.callback_log[0].action);
}

#[test]
fn button_press_is_repeatable_and_does_not_become_selection_state() {
    let mut button = Button::new("Next page");
    let action = UiAction::button_press(button.state_id().clone());

    let first = button.apply_action(&action);
    let second = button.apply_action(&action);
    let node = UiNode::from(button);

    assert!(first.handled);
    assert!(second.handled);
    assert_eq!("button_press", first.callback_log[0].action);
    assert_eq!("button_press", second.callback_log[0].action);
    assert!(!node.props().interaction.has_selection);
    assert!(!node.props().interaction.active);
}

#[test]
fn app_global_state_updates_component_owned_state_via_handle() {
    let mut button = Button::new("Next page").loading(true);
    let button_state = button.state_handle();
    let press_while_loading = UiAction::button_press(button.state_id().clone());

    assert!(!button.apply_action(&press_while_loading).handled);
    button_state.update(|state| {
        state.loading = false;
    });
    let mut button = button.sync_state(&button_state);
    let press_after_loading = UiAction::button_press(button.state_id().clone());

    assert!(button.apply_action(&press_after_loading).handled);
}

#[test]
fn state_handle_supports_react_like_get_set_and_update_without_global_store() {
    let mut initial_state = Button::new("Save").state_snapshot();
    initial_state.loading = true;
    let state_handle = UiStateHandle::new(initial_state);

    assert!(state_handle.get().loading);
    state_handle.update(|state| {
        state.loading = false;
        state.interaction.value = "ready".to_string();
    });
    assert!(!state_handle.get().loading);
    assert_eq!("ready", state_handle.get().interaction.value);

    let mut next_state = state_handle.get();
    next_state.disabled = true;
    state_handle.set(next_state);

    assert!(state_handle.with(|state| state.disabled));
}

#[test]
fn passive_text_ignores_click_but_button_accepts_generic_click() {
    let mut text = katana_ui_core::atom::Text::new("Tree row");
    let mut button = Button::new("Open row");
    let text_result = text.apply_action(&UiAction::click(text.state_id().clone()));
    let button_result = button.apply_action(&UiAction::click(button.state_id().clone()));

    assert!(!text_result.handled);
    assert!(button_result.handled);
    assert_eq!("click", button_result.callback_log[0].action);
    assert_eq!(button.state_id(), &button_result.callback_log[0].target);
}

#[test]
fn state_id_is_unique_and_value_updates_only_matching_target() {
    let mut first = Input::new("Name");
    let mut second = Input::new("Name");
    let action = UiAction::input_value(first.state_id().clone(), "KUC");

    assert_ne!(first.state_id(), second.state_id());
    let first_result = first.apply_action(&action);
    let second_result = second.apply_action(&action);

    assert!(first_result.handled);
    assert!(!second_result.handled);
    assert_eq!("KUC", UiNode::from(first).props().interaction.value);
    assert_eq!("", UiNode::from(second).props().interaction.value);
}

#[test]
fn component_action_updates_owned_state_before_rendering() {
    let mut input = Input::new("Text input");
    let action = UiAction::input_value(input.state_id().clone(), "typed");

    let result = input.apply_action(&action);
    let node = UiNode::from(input);

    assert!(result.handled);
    assert_eq!("typed", result.after.value);
    assert_eq!("typed", node.props().interaction.value);
    assert_eq!("input_value", result.callback_log[0].action);
}

#[test]
fn readonly_input_rejects_value_mutation_actions() {
    let mut input = Input::new("Text input").value("locked").readonly(true);
    let input_value_result =
        input.apply_action(&UiAction::input_value(input.state_id().clone(), "changed"));
    let clear_value_result = input.apply_action(&UiAction::clear_value(input.state_id().clone()));
    let cursor_result = input.apply_action(&UiAction::cursor_selection(
        input.state_id().clone(),
        4,
        1,
        4,
    ));
    let node = UiNode::from(input);

    assert!(!input_value_result.handled);
    assert!(!clear_value_result.handled);
    assert!(cursor_result.handled);
    assert!(input_value_result.callback_log.is_empty());
    assert!(clear_value_result.callback_log.is_empty());
    assert_eq!("locked", node.props().interaction.value);
    assert_eq!(4, node.props().interaction.cursor);
    assert_eq!(1, node.props().interaction.selection_start);
    assert_eq!(4, node.props().interaction.selection_end);
}

#[test]
fn focus_hover_active_and_cursor_state_are_owned_by_target_component() {
    let mut button = Button::new("Save").focusable(true);
    let mut input = Input::new("Text input");

    let focus_result = button.apply_action(&UiAction::focus(button.state_id().clone()));
    let hover_result = button.apply_action(&UiAction::hover(button.state_id().clone(), true));
    let active_result = button.apply_action(&UiAction::active(button.state_id().clone(), true));
    let cursor_result = input.apply_action(&UiAction::cursor_selection(
        input.state_id().clone(),
        3,
        1,
        3,
    ));

    let button_node = UiNode::from(button);
    let input_node = UiNode::from(input);

    assert!(focus_result.handled);
    assert!(hover_result.handled);
    assert!(active_result.handled);
    assert!(cursor_result.handled);
    assert!(button_node.props().interaction.focused);
    assert!(button_node.props().interaction.hovered);
    assert!(button_node.props().interaction.active);
    assert_eq!(3, input_node.props().interaction.cursor);
    assert_eq!(1, input_node.props().interaction.selection_start);
    assert_eq!(3, input_node.props().interaction.selection_end);
}

#[test]
fn loading_button_suppresses_press_until_loading_finishes() {
    let mut button = Button::new("Save").loading(true);
    let result = button.apply_action(&UiAction::button_press(button.state_id().clone()));

    assert!(!result.handled);
    assert!(result.callback_log.is_empty());
}

#[test]
fn clear_action_clears_input_value_only_when_editable() {
    let mut editable = Input::new("Text input")
        .value("typed")
        .clear_action("Clear");
    let mut disabled = Input::new("Text input").value("locked").disabled(true);
    let clear_editable = UiAction::clear_value(editable.state_id().clone());
    let clear_disabled = UiAction::clear_value(disabled.state_id().clone());

    let editable_result = editable.apply_action(&clear_editable);
    let disabled_result = disabled.apply_action(&clear_disabled);

    assert!(editable_result.handled);
    assert!(!disabled_result.handled);
    assert_eq!("", UiNode::from(editable).props().interaction.value);
    assert_eq!("locked", UiNode::from(disabled).props().interaction.value);
}

#[test]
fn selection_actions_update_only_their_owned_state() {
    let mut checkbox = Checkbox::new("Enabled");
    let mut radio = Radio::new("Manual");
    let mut toggle = Toggle::new("Live");

    let checkbox_result = checkbox.apply_action(&UiAction::checkbox_checked(
        checkbox.state_id().clone(),
        true,
    ));
    let radio_result = radio.apply_action(&UiAction::radio_selected(radio.state_id().clone()));
    let toggle_result =
        toggle.apply_action(&UiAction::toggle_checked(toggle.state_id().clone(), true));
    let checkbox_focus = checkbox.apply_action(&UiAction::focus(checkbox.state_id().clone()));
    let radio_hover = radio.apply_action(&UiAction::hover(radio.state_id().clone(), true));
    let toggle_focus = toggle.apply_action(&UiAction::focus(toggle.state_id().clone()));
    let toggle_hover = toggle.apply_action(&UiAction::hover(toggle.state_id().clone(), true));

    assert!(checkbox_result.handled);
    assert!(radio_result.handled);
    assert!(toggle_result.handled);
    assert!(checkbox_focus.handled);
    assert!(radio_hover.handled);
    assert!(toggle_focus.handled);
    assert!(toggle_hover.handled);
    assert!(UiNode::from(checkbox).props().checked);
    let radio_node = UiNode::from(radio);
    let toggle_node = UiNode::from(toggle);
    assert!(radio_node.props().checked);
    assert!(radio_node.props().interaction.hovered);
    assert!(toggle_node.props().checked);
    assert!(toggle_node.props().interaction.focused);
    assert!(toggle_node.props().interaction.hovered);
    assert_eq!("checkbox_checked", checkbox_result.callback_log[0].action);
    assert_eq!("radio_selected", radio_result.callback_log[0].action);
    assert_eq!("toggle_checked", toggle_result.callback_log[0].action);
    assert!(checkbox_focus.after.focused);
}

#[test]
fn progress_action_updates_typed_progress_props() {
    let mut progress = ProgressBar::new("Sync");
    let result = progress.apply_action(&UiAction::progress_changed(
        progress.state_id().clone(),
        true,
        PROGRESS_PERCENT,
    ));
    let node = UiNode::from(progress);

    assert!(result.handled);
    assert!(node.props().determinate);
    assert_eq!(PROGRESS_PERCENT, node.props().progress_percent);
    assert_eq!("progress_changed", result.callback_log[0].action);
}

#[test]
fn color_action_updates_typed_color_swatch_props() {
    let mut swatch = ColorSwatch::new("Accent");
    let result = swatch.apply_action(&UiAction::color_drag(
        swatch.state_id().clone(),
        RgbaActionValue::new(COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_ALPHA),
        COLOR_HUE,
        false,
    ));
    let node = UiNode::from(swatch);

    assert!(result.handled);
    assert_eq!(COLOR_RGBA, node.props().color_swatch.selected_color);
    assert_eq!(COLOR_RGBA, node.props().interaction.value);
    assert_eq!("color_drag", result.callback_log[0].action);
}

#[test]
fn slide_action_updates_value_with_component_specific_event_name() {
    let mut slide = SlideControl::new("Opacity");
    let result = slide.apply_action(&UiAction::slide_changed(slide.state_id().clone(), "0.72"));
    let focus = slide.apply_action(&UiAction::focus(slide.state_id().clone()));
    let hover = slide.apply_action(&UiAction::hover(slide.state_id().clone(), true));
    let dragging = slide.apply_action(&UiAction::dragging(slide.state_id().clone(), true));
    let node = UiNode::from(slide);

    assert!(result.handled);
    assert!(focus.handled);
    assert!(hover.handled);
    assert!(dragging.handled);
    assert_eq!("0.72", node.props().interaction.value);
    assert!(node.props().interaction.focused);
    assert!(node.props().interaction.hovered);
    assert!(node.props().interaction.dragging);
    assert_eq!("slide_changed", result.callback_log[0].action);
}

#[test]
fn passive_badge_ignores_dismiss_action() {
    let mut badge = Badge::new("Ready");
    let result = badge.apply_action(&UiAction::dismiss(badge.state_id().clone()));

    assert!(!result.handled);
    assert!(result.callback_log.is_empty());
}
