use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Length {
    Px(f32),
    Fill,
    Fit,
}

impl Length {
    #[must_use]
    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }
}

impl From<Length> for crate::render_model::UiDimension {
    fn from(value: Length) -> Self {
        match value {
            Length::Px(px) => Self::Px(px.max(0.0).round() as u16),
            Length::Fill => Self::Fill,
            Length::Fit => Self::FitContent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EdgeInsets {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl From<Alignment> for crate::render_model::UiAlignItems {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Start => Self::Start,
            Alignment::Center => Self::Center,
            Alignment::End => Self::End,
            Alignment::Stretch => Self::Stretch,
        }
    }
}

impl From<Alignment> for crate::render_model::UiJustifyContent {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Start => Self::Start,
            Alignment::Center => Self::Center,
            Alignment::End => Self::End,
            Alignment::Stretch => Self::Stretch,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutAxis {
    Horizontal,
    Vertical,
    Both,
    Overlay,
}

impl From<LayoutAxis> for crate::render_model::UiLayoutAxis {
    fn from(value: LayoutAxis) -> Self {
        match value {
            LayoutAxis::Horizontal => Self::Horizontal,
            LayoutAxis::Vertical => Self::Vertical,
            LayoutAxis::Both => Self::Both,
            LayoutAxis::Overlay => Self::Overlay,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverflowBehavior {
    Fit,
    Hidden,
    Scroll,
    Auto,
}

impl From<OverflowBehavior> for crate::render_model::UiOverflow {
    fn from(value: OverflowBehavior) -> Self {
        match value {
            OverflowBehavior::Fit => Self::Visible,
            OverflowBehavior::Hidden => Self::Hidden,
            OverflowBehavior::Scroll => Self::Scroll,
            OverflowBehavior::Auto => Self::Auto,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SizePolicy {
    pub width: Length,
    pub height: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignHorizontal {
    Start,
    Center,
    End,
    Stretch,
}

impl AlignHorizontal {
    pub(super) const fn to_justify(self) -> crate::render_model::UiJustifyContent {
        match self {
            Self::Start => crate::render_model::UiJustifyContent::Start,
            Self::Center => crate::render_model::UiJustifyContent::Center,
            Self::End => crate::render_model::UiJustifyContent::End,
            Self::Stretch => crate::render_model::UiJustifyContent::Stretch,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignVertical {
    Start,
    Center,
    End,
    Stretch,
}

impl AlignVertical {
    pub(super) const fn to_items(self) -> crate::render_model::UiAlignItems {
        match self {
            Self::Start => crate::render_model::UiAlignItems::Start,
            Self::Center => crate::render_model::UiAlignItems::Center,
            Self::End => crate::render_model::UiAlignItems::End,
            Self::Stretch => crate::render_model::UiAlignItems::Stretch,
        }
    }
}
