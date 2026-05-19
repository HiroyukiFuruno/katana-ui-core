use super::ColorPicker;
use crate::render_model::{UiBorder, UiNode, UiNodeKind};

const TRIGGER_BORDER_WIDTH_PX: u16 = 1;
const TRIGGER_BORDER_RADIUS_PX: u16 = 4;

impl From<ColorPicker> for UiNode {
    fn from(value: ColorPicker) -> Self {
        let common = value.trigger_common_props();
        let alpha_class = value.alpha_style_class();
        let border_class = value.border_style_class();
        let panel_scale_class = value.panel_scale_style_class();
        let mut node = value
            .state
            .node(UiNodeKind::ColorPicker, value.label)
            .common(common)
            .size(value.trigger_size)
            .style_class("kuc-color-picker-trigger")
            .style_class(alpha_class)
            .style_class(border_class)
            .style_class("kuc-color-picker-eyedropper-hidden")
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

    fn panel_scale_style_class(&self) -> String {
        format!("kuc-color-picker-panel-scale-{}", self.panel_scale_percent)
    }
}
