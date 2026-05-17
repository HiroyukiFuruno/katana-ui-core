use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::molecule::{ColorPicker, RgbaColor};
use katana_ui_core::render_model::UiNode;

const DRAG_START_RED: u8 = 24;
const DRAG_START_GREEN: u8 = 48;
const DRAG_START_BLUE: u8 = 96;
const DRAG_START_ALPHA: u8 = 128;
const DRAG_START_HUE: u16 = 210;
const DRAG_END_RED: u8 = 200;
const DRAG_END_GREEN: u8 = 160;
const DRAG_END_BLUE: u8 = 80;
const DRAG_END_ALPHA: u8 = 192;
const DRAG_END_HUE: u16 = 35;
const DEFAULT_COLOR_CHANNEL: u8 = 0;
const DEFAULT_COLOR_ALPHA: u8 = 255;
const DRAG_END_CSS: &str = "rgba(200, 160, 80, 192)";

#[test]
fn color_picker_drag_action_continuously_updates_owned_rgba_alpha_and_preview_state() {
    let mut picker = ColorPicker::new("Color").rgba(RgbaColor::new(
        DRAG_START_RED,
        DRAG_START_GREEN,
        DRAG_START_BLUE,
        DRAG_START_ALPHA,
    ));
    let drag_start = UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(
            DRAG_START_RED,
            DRAG_START_GREEN,
            DRAG_START_BLUE,
            DRAG_START_ALPHA,
        ),
        DRAG_START_HUE,
        true,
    );
    let drag_end = UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(DRAG_END_RED, DRAG_END_GREEN, DRAG_END_BLUE, DRAG_END_ALPHA),
        DRAG_END_HUE,
        false,
    );

    let start_result = picker.apply_action(&drag_start);
    let end_result = picker.apply_action(&drag_end);
    let node = UiNode::from(picker.clone());

    assert!(start_result.handled);
    assert!(end_result.handled);
    assert_eq!("color_drag", start_result.callback_log[0].action);
    assert_eq!("color_drag", end_result.callback_log[0].action);
    assert_eq!(
        RgbaColor::new(DRAG_END_RED, DRAG_END_GREEN, DRAG_END_BLUE, DRAG_END_ALPHA),
        picker.color_value()
    );
    assert_eq!(DRAG_END_ALPHA, picker.alpha_value());
    assert_eq!(DRAG_END_HUE, picker.hue_value());
    assert!(!picker.previews_color());
    assert_eq!(DRAG_END_CSS, node.props().interaction.value);
}

#[test]
fn color_picker_drag_action_serializes_with_callback_log_name() -> serde_json::Result<()> {
    let mut picker = ColorPicker::new("Color");
    let action = UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(DRAG_END_RED, DRAG_END_GREEN, DRAG_END_BLUE, DRAG_END_ALPHA),
        DRAG_END_HUE,
        true,
    );
    let encoded_action = serde_json::to_string(&action)?;
    let decoded_action: UiAction = serde_json::from_str(&encoded_action)?;
    let result = picker.apply_action(&decoded_action);

    assert!(encoded_action.contains("\"ColorPicker\""));
    assert!(result.handled);
    assert_eq!("color_drag", result.callback_log[0].action);
    Ok(())
}

#[test]
fn color_picker_is_not_complete_with_only_generic_value_state() {
    let mut picker = ColorPicker::new("Color");
    let result = picker.apply_action(&UiAction::set_value(
        picker.state_id().clone(),
        "rgba(1, 2, 3, 4)",
    ));

    assert!(result.handled);
    assert_eq!(
        RgbaColor::new(
            DEFAULT_COLOR_CHANNEL,
            DEFAULT_COLOR_CHANNEL,
            DEFAULT_COLOR_CHANNEL,
            DEFAULT_COLOR_ALPHA
        ),
        picker.color_value()
    );
    assert_eq!(DEFAULT_COLOR_ALPHA, picker.alpha_value());
    assert!(picker.previews_color());
}
