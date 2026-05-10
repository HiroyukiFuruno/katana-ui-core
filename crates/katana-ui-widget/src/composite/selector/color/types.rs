use crate::theme::color::Color;

/// Size of each color swatch cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwatchSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `ColorSwatch`.
#[derive(Debug, Clone)]
pub struct ColorSwatchProps {
    pub value: Color,
    pub palette: Vec<Color>,
    pub size: SwatchSize,
    pub disabled: bool,
    pub a11y_label: String,
}
