use crate::render_model::{UiJustifyContent, UiSize};
use serde::{Deserialize, Serialize};

pub const COMPACT_CONTROL_SIZE_PX: u16 = 28;
pub const DEFAULT_CONTROL_SIZE_PX: u16 = 32;
pub const TALL_CONTROL_SIZE_PX: u16 = 44;

pub const LEADING_POSITION_INDEX: usize = 0;
pub const TRAILING_POSITION_INDEX: usize = 1;
pub const AUTO_POSITION_INDEX: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlKind {
    Close,
    Minimize,
    Maximize,
    Restore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlsPosition {
    Leading,
    Trailing,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlVisibility {
    Always,
    Hover,
    FullscreenHover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlSize {
    Compact,
    Default,
    Tall,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowControlButtonGroupOptions {
    pub controls: Vec<WindowControlKind>,
    pub position: WindowControlsPosition,
    pub visibility: WindowControlVisibility,
    pub size: WindowControlSize,
}

impl Default for WindowControlButtonGroupOptions {
    fn default() -> Self {
        Self {
            controls: vec![
                WindowControlKind::Close,
                WindowControlKind::Minimize,
                WindowControlKind::Maximize,
            ],
            position: WindowControlsPosition::Auto,
            visibility: WindowControlVisibility::Always,
            size: WindowControlSize::Default,
        }
    }
}

impl WindowControlSize {
    #[must_use]
    pub const fn pixels(self) -> u16 {
        match self {
            Self::Compact => COMPACT_CONTROL_SIZE_PX,
            Self::Default => DEFAULT_CONTROL_SIZE_PX,
            Self::Tall => TALL_CONTROL_SIZE_PX,
        }
    }

    #[must_use]
    pub const fn ui_size(self) -> UiSize {
        match self {
            Self::Compact => UiSize::Small,
            Self::Default => UiSize::Medium,
            Self::Tall => UiSize::Large,
        }
    }
}

impl WindowControlButtonGroupOptions {
    pub(crate) fn visible(&self, hovered: bool, fullscreen: bool) -> bool {
        match self.visibility {
            WindowControlVisibility::Always => true,
            WindowControlVisibility::Hover => hovered,
            WindowControlVisibility::FullscreenHover => !fullscreen || hovered,
        }
    }
}

impl WindowControlsPosition {
    pub(crate) fn index(self) -> usize {
        match self {
            Self::Leading => LEADING_POSITION_INDEX,
            Self::Trailing => TRAILING_POSITION_INDEX,
            Self::Auto => AUTO_POSITION_INDEX,
        }
    }

    pub(crate) fn justify_content(self) -> UiJustifyContent {
        match self {
            Self::Leading | Self::Auto => UiJustifyContent::Start,
            Self::Trailing => UiJustifyContent::End,
        }
    }

    pub(crate) fn style_class(self) -> &'static str {
        match self {
            Self::Leading => "window-controls-leading",
            Self::Trailing => "window-controls-trailing",
            Self::Auto => "window-controls-auto",
        }
    }
}

impl WindowControlVisibility {
    pub(crate) fn style_class(self) -> &'static str {
        match self {
            Self::Always => "window-controls-visible",
            Self::Hover => "window-controls-hover",
            Self::FullscreenHover => "window-controls-fullscreen-hover",
        }
    }
}
