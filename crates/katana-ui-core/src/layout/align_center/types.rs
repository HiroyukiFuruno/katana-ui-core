use crate::theme::color::Color;

/// Raw config stored by [`AlignCenterWrapper`].
#[derive(Debug, Clone, Copy)]
pub struct AlignCenterWrapperProps {
    pub horizontal: bool,
    pub vertical: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: f32,
    pub gap: f32,
    pub background: Option<Color>,
    pub disabled: bool,
}

/// Resolved properties for rendering [`AlignCenterWrapper`].
#[derive(Debug, Clone, Copy)]
pub struct ResolvedAlignCenterWrapper {
    pub horizontal: bool,
    pub vertical: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: f32,
    pub gap: f32,
    pub background: Option<Color>,
    pub disabled: bool,
}
