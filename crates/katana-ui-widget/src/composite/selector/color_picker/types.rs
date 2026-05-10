use crate::theme::color::Color;
use std::rc::Rc;

/// Alpha editing mode matching katana's egui color picker wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPickerAlpha {
    #[default]
    Opaque,
    BlendOrAdditive,
}

impl ColorPickerAlpha {
    #[must_use]
    pub const fn allows_alpha(self) -> bool {
        matches!(self, Self::BlendOrAdditive)
    }
}

/// RGBA channel controlled by the color picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgbaChannel {
    Red,
    Green,
    Blue,
    Alpha,
}

/// Properties shared by inline and labeled color pickers.
#[derive(Clone)]
pub struct InlineColorPickerProps {
    pub value: Color,
    pub alpha: ColorPickerAlpha,
    pub disabled: bool,
    pub readonly: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(Color)>,
}

/// Properties for a katana settings-style labeled color picker row.
#[derive(Clone)]
pub struct LabeledColorPickerProps {
    pub label: String,
    pub label_width: f32,
    pub spacing: f32,
    pub offset_y: f32,
    pub picker: InlineColorPickerProps,
}

/// Properties for the backwards-compatible RGBA entry point.
pub type ColorPickerRgbaProps = InlineColorPickerProps;

/// Resolved state for `InlineColorPicker`.
#[derive(Clone)]
pub struct ResolvedInlineColorPicker {
    pub value: Color,
    pub alpha: ColorPickerAlpha,
    pub disabled: bool,
    pub readonly: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(Color)>,
}

/// Resolved state for `LabeledColorPicker`.
#[derive(Clone)]
pub struct ResolvedLabeledColorPicker {
    pub label: String,
    pub label_width: f32,
    pub spacing: f32,
    pub offset_y: f32,
    pub picker: ResolvedInlineColorPicker,
}

/// Resolved state for `ColorPickerRgba`.
pub type ResolvedColorPickerRgba = ResolvedInlineColorPicker;

/// Inline color button picker matching katana's `InlineColorPicker`.
#[derive(Clone)]
pub struct InlineColorPicker {
    pub(super) props: InlineColorPickerProps,
}

/// Labeled color picker row matching katana's settings color rows.
#[derive(Clone)]
pub struct LabeledColorPicker {
    pub(super) props: LabeledColorPickerProps,
}

/// Backwards-compatible RGBA color picker entry point.
#[derive(Clone)]
pub struct ColorPickerRgba {
    pub(super) props: ColorPickerRgbaProps,
}
