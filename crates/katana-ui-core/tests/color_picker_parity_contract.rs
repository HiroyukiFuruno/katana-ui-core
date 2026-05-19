use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::molecule::{ColorPicker, RgbaColor};
use katana_ui_core::render_model::{UiNode, UiSize};

const START_RED: u8 = 12;
const START_GREEN: u8 = 34;
const START_BLUE: u8 = 56;
const TRANSLUCENT_ALPHA: u8 = 78;
const DRAG_RED: u8 = 90;
const DRAG_GREEN: u8 = 120;
const DRAG_BLUE: u8 = 210;
const FIRST_DRAG_ALPHA: u8 = 64;
const SECOND_DRAG_ALPHA: u8 = 192;
const FIRST_DRAG_HUE: u16 = 140;
const SECOND_DRAG_HUE: u16 = 260;
const DEFAULT_PANEL_SCALE_PERCENT: u16 = 75;
const CUSTOM_PANEL_SCALE_PERCENT: u16 = 90;
const OPAQUE_ALPHA: u8 = 255;

#[test]
fn rgb_only_mode_keeps_alpha_opaque_and_hides_alpha_semantics() {
    let mut picker = ColorPicker::new("RGB")
        .rgba_mode(false)
        .rgba(RgbaColor::new(
            START_RED,
            START_GREEN,
            START_BLUE,
            TRANSLUCENT_ALPHA,
        ))
        .alpha(FIRST_DRAG_ALPHA);
    let result = picker.apply_action(&UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(DRAG_RED, DRAG_GREEN, DRAG_BLUE, SECOND_DRAG_ALPHA),
        FIRST_DRAG_HUE,
        true,
    ));

    assert!(result.handled);
    assert!(!picker.uses_rgba_mode());
    assert!(!picker.alpha_control_visible());
    assert!(!picker.panel_exposes_alpha());
    assert_eq!(OPAQUE_ALPHA, picker.color_value().alpha);
    assert_eq!(OPAQUE_ALPHA, picker.alpha_value());
    assert_eq!(
        "rgba(90, 120, 210, 255)",
        picker.trigger_transparent_preview()
    );
    assert_eq!(
        picker.trigger_opaque_preview(),
        picker.trigger_transparent_preview()
    );
    assert!(!picker.trigger_uses_checker_background());
}

#[test]
fn rgba_mode_exposes_alpha_and_alpha_drag_updates_continuously() {
    let mut picker = ColorPicker::new("RGBA").rgba(RgbaColor::new(
        START_RED,
        START_GREEN,
        START_BLUE,
        TRANSLUCENT_ALPHA,
    ));
    let first = picker.apply_action(&UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(DRAG_RED, DRAG_GREEN, DRAG_BLUE, FIRST_DRAG_ALPHA),
        FIRST_DRAG_HUE,
        true,
    ));
    let second = picker.apply_action(&UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(DRAG_RED, DRAG_GREEN, DRAG_BLUE, SECOND_DRAG_ALPHA),
        SECOND_DRAG_HUE,
        true,
    ));

    assert!(first.handled);
    assert!(second.handled);
    assert!(picker.uses_rgba_mode());
    assert!(picker.alpha_control_visible());
    assert!(picker.panel_exposes_alpha());
    assert_eq!(SECOND_DRAG_ALPHA, picker.color_value().alpha);
    assert_eq!(SECOND_DRAG_ALPHA, picker.alpha_value());
    assert_eq!(SECOND_DRAG_HUE, picker.hue_value());
    assert_eq!(
        "rgba(90, 120, 210, 192)",
        picker.trigger_transparent_preview()
    );
    assert_eq!("rgba(90, 120, 210, 255)", picker.trigger_opaque_preview());
    assert!(picker.trigger_uses_checker_background());
}

#[test]
fn readonly_and_disabled_suppress_color_changes_while_open_close_and_dismiss_work() {
    let mut readonly = ColorPicker::new("Readonly")
        .rgba(RgbaColor::new(
            START_RED,
            START_GREEN,
            START_BLUE,
            TRANSLUCENT_ALPHA,
        ))
        .readonly(true);
    let blocked = readonly.apply_action(&UiAction::color_drag(
        readonly.state_id().clone(),
        RgbaActionValue::new(DRAG_RED, DRAG_GREEN, DRAG_BLUE, SECOND_DRAG_ALPHA),
        SECOND_DRAG_HUE,
        true,
    ));
    let opened = readonly.apply_action(&UiAction::set_open(readonly.state_id().clone(), true));
    let closed = readonly.apply_action(&UiAction::set_open(readonly.state_id().clone(), false));
    let dismissed = readonly.apply_action(&UiAction::dismiss(readonly.state_id().clone()));

    assert!(!blocked.handled);
    assert_eq!(TRANSLUCENT_ALPHA, readonly.color_value().alpha);
    assert!(opened.handled);
    assert!(opened.after.open);
    assert!(closed.handled);
    assert!(!closed.after.open);
    assert!(dismissed.handled);
    assert!(!dismissed.after.open);

    let mut disabled = ColorPicker::new("Disabled")
        .rgba(RgbaColor::new(
            START_RED,
            START_GREEN,
            START_BLUE,
            TRANSLUCENT_ALPHA,
        ))
        .disabled(true);
    let disabled_result = disabled.apply_action(&UiAction::color_drag(
        disabled.state_id().clone(),
        RgbaActionValue::new(DRAG_RED, DRAG_GREEN, DRAG_BLUE, SECOND_DRAG_ALPHA),
        SECOND_DRAG_HUE,
        true,
    ));

    assert!(!disabled_result.handled);
    assert_eq!(TRANSLUCENT_ALPHA, disabled.color_value().alpha);
}

#[test]
fn trigger_metadata_sizes_border_eyedropper_and_panel_scale_are_modeled() {
    for size in [
        UiSize::XSmall,
        UiSize::Small,
        UiSize::Medium,
        UiSize::Large,
        UiSize::XLarge,
    ] {
        let picker = ColorPicker::new("Size")
            .rgba(RgbaColor::new(
                START_RED,
                START_GREEN,
                START_BLUE,
                TRANSLUCENT_ALPHA,
            ))
            .trigger_size(size);
        let node = UiNode::from(picker.clone());

        assert_eq!(size, picker.trigger_size_model());
        assert_eq!(size, node.props().size);
    }

    let default_picker = ColorPicker::new("Default").rgba(RgbaColor::new(
        START_RED,
        START_GREEN,
        START_BLUE,
        TRANSLUCENT_ALPHA,
    ));
    let borderless = ColorPicker::new("Borderless")
        .trigger_border(false)
        .panel_scale_percent(CUSTOM_PANEL_SCALE_PERCENT);
    let default_node = UiNode::from(default_picker.clone());
    let borderless_node = UiNode::from(borderless.clone());

    assert!(default_picker.has_trigger_border());
    assert!(default_node.props().common.border.visible);
    assert!(!default_picker.trigger_shows_numeric_value());
    assert!(!default_picker.shows_eyedropper_control());
    assert!(!default_picker.panel_shows_eyedropper());
    assert_eq!(
        DEFAULT_PANEL_SCALE_PERCENT,
        default_picker.panel_scale_percent_model()
    );
    assert!(!borderless.has_trigger_border());
    assert!(!borderless_node.props().common.border.visible);
    assert_eq!(
        CUSTOM_PANEL_SCALE_PERCENT,
        borderless.panel_scale_percent_model()
    );
}
