use crate::theme::color::Color;
use std::rc::Rc;

/// Size of each color swatch cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwatchSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Shape of each color swatch cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwatchShape {
    #[default]
    RoundedRect,
    Circle,
}

/// Properties for `ColorSwatch`.
#[derive(Clone)]
pub struct ColorSwatchProps {
    pub value: Color,
    pub palette: Vec<Color>,
    pub size: SwatchSize,
    pub shape: SwatchShape,
    pub disabled: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(Color)>,
}

/// Resolved visual properties for a single swatch cell.
#[derive(Debug, Clone)]
pub struct ResolvedSwatchCell {
    pub color: Color,
    pub cell_size: f32,
    pub selected: bool,
    pub ring_width: f32,
    pub ring_color: Color,
    pub border_radius: f32,
}

/// Resolved visual properties for `ColorSwatch`.
#[derive(Debug, Clone)]
pub struct ResolvedColorSwatch {
    pub cells: Vec<ResolvedSwatchCell>,
    pub disabled: bool,
    pub a11y_label: String,
}

/// Builder for the ColorSwatch composite widget.
#[derive(Clone)]
pub struct ColorSwatch {
    pub(super) props: ColorSwatchProps,
}
