use super::super::ops;
use super::super::types::{
    ColorPickerAlpha, ColorPickerTriggerSize, InlineColorPicker, LabeledColorPicker,
    LabeledColorPickerProps, ResolvedLabeledColorPicker,
};
use super::super::{
    COLOR_LABEL_WIDTH, COLOR_OFFSET_Y, COLOR_PICKER_MAX_PANEL_SCALE, COLOR_PICKER_MIN_PANEL_SCALE,
    COLOR_SPACING,
};
use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;

impl LabeledColorPicker {
    #[must_use]
    pub fn new(label: impl Into<String>, value: Color) -> Self {
        let label = label.into();
        Self {
            props: LabeledColorPickerProps {
                label: label.clone(),
                label_width: COLOR_LABEL_WIDTH,
                spacing: COLOR_SPACING,
                offset_y: COLOR_OFFSET_Y,
                picker: InlineColorPicker::new(value, label).props,
            },
        }
    }

    #[must_use]
    pub fn rgba(mut self, is_rgba: bool) -> Self {
        self.props.picker.alpha = if is_rgba {
            ColorPickerAlpha::BlendOrAdditive
        } else {
            ColorPickerAlpha::Opaque
        };
        self.props.picker.value =
            ops::ColorPickerOps::resolve_value(self.props.picker.value, self.props.picker.alpha);
        self
    }

    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.props.picker.open = open;
        self
    }

    #[must_use]
    pub fn label_width(mut self, width: f32) -> Self {
        self.props.label_width = width;
        self
    }

    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.props.spacing = spacing;
        self
    }

    #[must_use]
    pub fn offset_y(mut self, offset: f32) -> Self {
        self.props.offset_y = offset;
        self
    }

    #[must_use]
    pub fn panel_scale(mut self, scale: f32) -> Self {
        self.props.picker.panel_scale =
            scale.clamp(COLOR_PICKER_MIN_PANEL_SCALE, COLOR_PICKER_MAX_PANEL_SCALE);
        self
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.picker.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn trigger_size(mut self, size: ColorPickerTriggerSize) -> Self {
        self.props.picker.trigger_size = size;
        self
    }

    #[must_use]
    pub fn trigger_border(mut self, border: bool) -> Self {
        self.props.picker.trigger_border = border;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.picker.disabled = disabled;
        self
    }

    #[must_use]
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.props.picker.readonly = readonly;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Color) + 'static) -> Self {
        self.props.picker.on_change = Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn on_pick_color(mut self, on_pick_color: impl Fn() + 'static) -> Self {
        self.props.picker.on_pick_color = Rc::new(on_pick_color);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedLabeledColorPicker {
        let picker = InlineColorPicker {
            props: self.props.picker.clone(),
        }
        .resolve(theme);
        ResolvedLabeledColorPicker {
            label: self.props.label.clone(),
            label_width: self.props.label_width,
            spacing: self.props.spacing,
            offset_y: self.props.offset_y,
            picker,
        }
    }
}
