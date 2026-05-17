use crate::theme::color::Color;
use floem::View;

/// Visual alignment for toolbar slot content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarAlignment {
    Top,
    #[default]
    Center,
    Bottom,
}

/// Raw properties held by [`Toolbar`].
pub(crate) struct ToolbarProps {
    pub leading: Option<Box<dyn View>>,
    pub trailing: Option<Box<dyn View>>,
    pub gap: Option<f32>,
    pub alignment: ToolbarAlignment,
    pub height: Option<f32>,
    pub padding: Option<f32>,
    pub background: Option<Color>,
    pub show_border: bool,
}

/// Resolved values used at render time.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedToolbar {
    pub gap: f32,
    pub alignment: ToolbarAlignment,
    pub height: Option<f32>,
    pub padding: f32,
    pub background: Option<Color>,
    pub show_border: bool,
    pub border_color: Color,
}
