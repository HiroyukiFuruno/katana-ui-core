use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{SplitPane, SplitPaneAxis};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

const MIN_PERCENT: u8 = 25;
const MAX_PERCENT: u8 = 75;
const HANDLE_WIDTH_PX: u8 = 8;
const RESET_PERCENT: u8 = 55;
const KEYBOARD_PERCENT: u8 = 40;

#[test]
fn split_pane_owns_axis_ratio_constraints_and_children() {
    let split = SplitPane::new()
        .axis(SplitPaneAxis::Vertical)
        .min_percent(MIN_PERCENT)
        .max_percent(MAX_PERCENT)
        .handle_width_px(HANDLE_WIDTH_PX)
        .reset_percent(RESET_PERCENT)
        .ratio_percent(KEYBOARD_PERCENT)
        .child(Text::new("Top"))
        .child(Text::new("Bottom"));

    assert_eq!(SplitPaneAxis::Vertical, split.axis_value());
    assert_eq!(KEYBOARD_PERCENT, split.ratio_percent_value());
    assert_eq!(MIN_PERCENT, split.min_percent_value());
    assert_eq!(MAX_PERCENT, split.max_percent_value());
    assert_eq!(HANDLE_WIDTH_PX, split.handle_width_px_value());
    assert_eq!(RESET_PERCENT, split.reset_percent_value());
    assert_eq!(2, split.children().len());

    let tree = UiTree::new(split);

    assert_eq!(UiNodeKind::SplitPane, tree.root().kind());
    assert_eq!(
        KEYBOARD_PERCENT.to_string(),
        tree.root().props().interaction.value
    );
    assert_eq!(2, tree.root().children().len());
}

#[test]
fn split_pane_resize_action_clamps_value_and_logs_event() {
    let mut split = SplitPane::new()
        .min_percent(MIN_PERCENT)
        .max_percent(MAX_PERCENT)
        .reset_percent(RESET_PERCENT);
    let action = UiAction::split_pane_resized(split.state_id().clone(), 90);
    let keyboard = UiAction::split_pane_keyboard_resize(split.state_id().clone(), KEYBOARD_PERCENT);
    let drag = UiAction::dragging(split.state_id().clone(), true);
    let reset = UiAction::split_pane_reset(split.state_id().clone());

    let result = split.apply_action(&action);
    let keyboard_result = split.apply_action(&keyboard);
    let drag_result = split.apply_action(&drag);
    let reset_result = split.apply_action(&reset);
    let tree = UiTree::new(split);

    assert!(result.handled);
    assert_eq!("split_pane_resized", result.callback_log[0].action);
    assert_eq!(MAX_PERCENT.to_string(), result.after.value);
    assert!(keyboard_result.handled);
    assert_eq!(
        "split_pane_keyboard_resize",
        keyboard_result.callback_log[0].action
    );
    assert_eq!(KEYBOARD_PERCENT.to_string(), keyboard_result.after.value);
    assert!(drag_result.handled);
    assert!(drag_result.after.dragging);
    assert!(reset_result.handled);
    assert_eq!("split_pane_reset", reset_result.callback_log[0].action);
    assert_eq!(
        RESET_PERCENT.to_string(),
        tree.root().props().interaction.value
    );
}
