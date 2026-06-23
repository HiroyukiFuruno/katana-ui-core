use super::{ColorBlendingMode, ColorPicker};
use crate::render_model::{
    UiBorder, UiColorBlendingMode, UiColorPickerProps, UiColorPickerTriggerKind, UiNode, UiNodeKind,
};

const TRIGGER_BORDER_WIDTH_PX: u16 = 1;
const TRIGGER_BORDER_RADIUS_PX: u16 = 4;

impl From<ColorPicker> for UiNode {
    fn from(value: ColorPicker) -> Self {
        let common = value.trigger_common_props();
        let alpha_class = value.alpha_style_class();
        let border_class = value.border_style_class();
        let eyedropper_class = value.eyedropper_style_class();
        let panel_scale_class = value.panel_scale_style_class();
        let color_picker = value.color_picker_props();
        let mut node = value
            .state
            .node(UiNodeKind::ColorPicker, value.label)
            .common(common)
            .size(value.trigger_size)
            .color_picker(color_picker)
            .style_class("kuc-color-picker-trigger")
            .style_class(alpha_class)
            .style_class(border_class)
            .style_class(eyedropper_class)
            .style_class(panel_scale_class);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

impl ColorPicker {
    fn trigger_common_props(&self) -> crate::render_model::UiCommonProps {
        let mut common = self.state.common.clone();
        common.border = if self.trigger_border {
            UiBorder::solid(
                TRIGGER_BORDER_WIDTH_PX,
                TRIGGER_BORDER_RADIUS_PX,
                "color-picker.trigger.border",
            )
        } else {
            UiBorder::none()
        };
        common
    }

    fn alpha_style_class(&self) -> &'static str {
        if self.rgba_mode {
            "kuc-color-picker-alpha-visible"
        } else {
            "kuc-color-picker-alpha-hidden"
        }
    }

    fn border_style_class(&self) -> &'static str {
        if self.trigger_border {
            "kuc-color-picker-trigger-bordered"
        } else {
            "kuc-color-picker-trigger-borderless"
        }
    }

    fn eyedropper_style_class(&self) -> &'static str {
        if self.shows_eyedropper_control() {
            "kuc-color-picker-eyedropper-visible"
        } else {
            "kuc-color-picker-eyedropper-hidden"
        }
    }

    fn panel_scale_style_class(&self) -> String {
        format!("kuc-color-picker-panel-scale-{}", self.panel_scale_percent)
    }

    fn color_picker_props(&self) -> UiColorPickerProps {
        UiColorPickerProps {
            trigger_kind: UiColorPickerTriggerKind::ColorButton,
            rgba_css: self.trigger_transparent_preview(),
            opaque_preview_css: self.trigger_opaque_preview(),
            checker_background: self.trigger_uses_checker_background(),
            rgba_mode: self.uses_rgba_mode(),
            alpha_slider_visible: self.panel_exposes_alpha(),
            eyedropper_visible: self.panel_shows_eyedropper(),
            hue_degrees: self.hue_value(),
            alpha: self.alpha_value(),
            blending: self.blending_mode().into(),
            color_plane: self.color_area_model().to_owned(),
            eyedropper_action: self.eyedropper_callback_model().to_owned(),
            panel_scale_percent: self.panel_scale_percent_model(),
        }
    }
}

impl From<ColorBlendingMode> for UiColorBlendingMode {
    fn from(value: ColorBlendingMode) -> Self {
        match value {
            ColorBlendingMode::Normal => Self::Normal,
            ColorBlendingMode::Additive => Self::Additive,
            ColorBlendingMode::Replace => Self::Replace,
            ColorBlendingMode::Multiply => Self::Multiply,
            ColorBlendingMode::Screen => Self::Screen,
        }
    }
}
