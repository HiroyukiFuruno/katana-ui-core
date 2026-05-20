use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::molecule::{ColorBlendingMode, ColorPicker, RgbaColor};
use katana_ui_core::render_model::{
    UiColorBlendingMode, UiColorPickerTriggerKind, UiNodeKind, UiSize, UiTree,
};

#[test]
fn color_picker_render_props_describe_popup_rgba_panel() {
    let tree = UiTree::new(
        ColorPicker::new("Brand")
            .open(true)
            .rgba(RgbaColor::new(64, 128, 255, 204))
            .hue(212)
            .alpha(204)
            .blending(ColorBlendingMode::Additive)
            .color_area("saturation/value square")
            .trigger_size(UiSize::Large)
            .title("Brand accent")
            .rgba_mode(true)
            .trigger_border(true)
            .eyedropper_callback("pick-screen-color")
            .panel_scale_percent(90),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::ColorPicker, tree.root().kind());
    assert_eq!(
        UiColorPickerTriggerKind::ColorButton,
        props.color_picker.trigger_kind
    );
    assert_eq!("rgba(64, 128, 255, 204)", props.color_picker.rgba_css);
    assert_eq!(
        "rgba(64, 128, 255, 255)",
        props.color_picker.opaque_preview_css
    );
    assert!(props.color_picker.checker_background);
    assert!(props.color_picker.rgba_mode);
    assert!(props.color_picker.alpha_slider_visible);
    assert!(props.color_picker.eyedropper_visible);
    assert_eq!(212, props.color_picker.hue_degrees);
    assert_eq!(204, props.color_picker.alpha);
    assert_eq!(UiColorBlendingMode::Additive, props.color_picker.blending);
    assert_eq!("saturation/value square", props.color_picker.color_plane);
    assert_eq!("pick-screen-color", props.color_picker.eyedropper_action);
    assert_eq!(90, props.color_picker.panel_scale_percent);
    assert!(props.interaction.open);
}

#[test]
fn color_picker_channel_hue_alpha_and_blending_actions_share_one_state() {
    let mut picker = ColorPicker::new("Brand")
        .rgba(RgbaColor::new(16, 32, 48, 255))
        .blending(ColorBlendingMode::Normal);

    let drag = picker.apply_action(&UiAction::color_drag(
        picker.state_id().clone(),
        RgbaActionValue::new(120, 80, 40, 128),
        28,
        true,
    ));
    let blend = picker.apply_action(&UiAction::color_blending_changed(
        picker.state_id().clone(),
        "additive",
    ));
    let tree = UiTree::new(picker.clone());

    assert!(drag.handled);
    assert!(blend.handled);
    assert_eq!(RgbaColor::new(120, 80, 40, 128), picker.color_value());
    assert_eq!(28, picker.hue_value());
    assert_eq!(128, picker.alpha_value());
    assert_eq!(ColorBlendingMode::Additive, picker.blending_mode());
    assert_eq!(
        "rgba(120, 80, 40, 128)",
        tree.root().props().interaction.value
    );
    assert_eq!(
        UiColorBlendingMode::Additive,
        tree.root().props().color_picker.blending
    );
}

#[test]
fn color_picker_readonly_and_disabled_block_all_color_panel_mutations() {
    let mut readonly = ColorPicker::new("Readonly")
        .rgba(RgbaColor::new(10, 20, 30, 40))
        .readonly(true);
    let mut disabled = ColorPicker::new("Disabled")
        .rgba(RgbaColor::new(10, 20, 30, 40))
        .disabled(true);

    let readonly_color = readonly.apply_action(&UiAction::color_drag(
        readonly.state_id().clone(),
        RgbaActionValue::new(200, 180, 160, 120),
        40,
        true,
    ));
    let readonly_blending = readonly.apply_action(&UiAction::color_blending_changed(
        readonly.state_id().clone(),
        "additive",
    ));
    let disabled_color = disabled.apply_action(&UiAction::color_drag(
        disabled.state_id().clone(),
        RgbaActionValue::new(200, 180, 160, 120),
        40,
        true,
    ));

    assert!(!readonly_color.handled);
    assert!(!readonly_blending.handled);
    assert!(!disabled_color.handled);
    assert_eq!(RgbaColor::new(10, 20, 30, 40), readonly.color_value());
    assert_eq!(ColorBlendingMode::Replace, readonly.blending_mode());
}

#[test]
fn color_picker_rgb_mode_hides_alpha_and_forces_opaque_preview() {
    let tree = UiTree::new(
        ColorPicker::new("RGB")
            .rgba_mode(false)
            .rgba(RgbaColor::new(1, 2, 3, 4))
            .alpha(32),
    );
    let props = &tree.root().props().color_picker;

    assert!(!props.rgba_mode);
    assert!(!props.alpha_slider_visible);
    assert!(!props.checker_background);
    assert_eq!("rgba(1, 2, 3, 255)", props.rgba_css);
    assert_eq!(255, props.alpha);
}
