use super::hsva::ColorPickerHsva;
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

/// Blending mode exposed to the picker panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPickerBlendMode {
    #[default]
    Normal,
    Additive,
}

impl ColorPickerBlendMode {
    pub const fn allows_alpha(self) -> bool {
        matches!(self, Self::Normal)
    }

    pub const fn to_alpha(self) -> ColorPickerAlpha {
        ColorPickerAlpha::BlendOrAdditive
    }

    pub const fn from_alpha(alpha: ColorPickerAlpha) -> Self {
        match alpha {
            ColorPickerAlpha::Opaque => Self::Normal,
            ColorPickerAlpha::BlendOrAdditive => Self::Normal,
        }
    }
}

/// Picker value with color and blending mode handled at the same time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPickerValue {
    pub color: Color,
    pub alpha: ColorPickerAlpha,
    pub blending_mode: ColorPickerBlendMode,
    pub hsva: ColorPickerHsva,
}

impl ColorPickerValue {
    #[must_use]
    pub fn new(color: Color, blending_mode: ColorPickerBlendMode) -> Self {
        Self::with_modes(color, ColorPickerAlpha::BlendOrAdditive, blending_mode)
    }

    #[must_use]
    pub fn with_alpha_mode(color: Color, alpha: ColorPickerAlpha) -> Self {
        Self::with_modes(color, alpha, ColorPickerBlendMode::from_alpha(alpha))
    }

    #[must_use]
    pub fn with_modes(
        color: Color,
        alpha: ColorPickerAlpha,
        blending_mode: ColorPickerBlendMode,
    ) -> Self {
        let resolved_blending = if alpha.allows_alpha() {
            blending_mode
        } else {
            ColorPickerBlendMode::Normal
        };
        let resolved_color = resolve_color(color, alpha, resolved_blending);
        Self {
            color: resolved_color,
            alpha,
            blending_mode: resolved_blending,
            hsva: ColorPickerHsva::from_color(resolved_color),
        }
    }
}

fn resolve_color(
    color: Color,
    alpha: ColorPickerAlpha,
    blending_mode: ColorPickerBlendMode,
) -> Color {
    if alpha.allows_alpha() && blending_mode.allows_alpha() {
        color
    } else {
        Color {
            a: u8::MAX,
            ..color
        }
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

/// Size preset for the compact color trigger button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPickerTriggerSize {
    Xs,
    Sm,
    #[default]
    Mid,
    Large,
    Xlarge,
}

/// Properties shared by inline and labeled color pickers.
#[derive(Clone)]
pub struct InlineColorPickerProps {
    pub value: Color,
    pub title: Option<String>,
    pub alpha: ColorPickerAlpha,
    pub panel_scale: f32,
    pub trigger_size: ColorPickerTriggerSize,
    pub trigger_border: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub a11y_label: String,
    pub open: bool,
    pub on_change: Rc<dyn Fn(Color)>,
    pub on_pick_color: Rc<dyn Fn()>,
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
    pub title: Option<String>,
    pub alpha: ColorPickerAlpha,
    pub panel_scale: f32,
    pub trigger_size: ColorPickerTriggerSize,
    pub trigger_border: bool,
    pub blending_mode: ColorPickerBlendMode,
    pub open: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(Color)>,
    pub on_pick_color: Rc<dyn Fn()>,
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
