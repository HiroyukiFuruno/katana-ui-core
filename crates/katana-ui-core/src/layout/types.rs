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
